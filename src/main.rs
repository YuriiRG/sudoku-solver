use std::{io::stdout, iter::repeat};

use anyhow::Result;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols,
    text::{Line, ToText},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

struct App {
    is_error: bool,
    selected: (u16, u16),
    board: Board,
}

#[derive(Clone, Copy, Debug, Default)]
struct Board {
    values: [[Option<u8>; 9]; 9],
}

#[derive(Clone, Copy, Debug, Default)]
struct SolvingBoard {
    values: [[SolvingCell; 9]; 9],
}

#[derive(Clone, Copy, Debug)]
struct SolvingCell([bool; 9]);

impl SolvingCell {
    fn eliminate(&mut self, number: u8) {
        self.0[number as usize - 1] = false;
    }
    fn leave_only(&mut self, number: u8) {
        if self.0[number as usize - 1] {
            self.0.fill(false);
            self.0[number as usize - 1] = true;
        } else {
            panic!("{number} is not a candidate, therefore it cannot be left.");
        }
    }
    fn contains(&self, number: u8) -> bool {
        self.0[number as usize - 1]
    }
    fn count(&self) -> usize {
        self.0.iter().filter(|&&is_candidate| is_candidate).count()
    }
    fn is_invalid(&self) -> bool {
        self.0.iter().all(|&is_candidate| !is_candidate)
    }
    fn is_definitive(&self) -> bool {
        self.count() == 1
    }
    fn definitive_value(&self) -> Option<u8> {
        if self.is_definitive() {
            Some(
                self.0
                    .iter()
                    .enumerate()
                    .find_map(|(i, &is_candidate)| if is_candidate { Some(i) } else { None })
                    .unwrap() as u8,
            )
        } else {
            None
        }
    }
}

impl Default for SolvingCell {
    fn default() -> Self {
        SolvingCell([true; 9])
    }
}
impl From<u8> for SolvingCell {
    fn from(value: u8) -> Self {
        let mut cell = SolvingCell([false; 9]);
        cell.0[value as usize - 1] = true;
        cell
    }
}

impl From<Board> for SolvingBoard {
    fn from(board: Board) -> Self {
        Self {
            values: board.values.map(|col| {
                col.map(|cell| match cell {
                    Some(value) => value.into(),
                    None => Default::default(),
                })
            }),
        }
    }
}

impl Board {
    fn set(&mut self, coords: (u16, u16), value: Option<u8>) {
        self.values[coords.0 as usize][coords.1 as usize] = value;
    }
    fn get(&mut self, coords: (u16, u16)) -> Option<u8> {
        self.values[coords.0 as usize][coords.1 as usize]
    }
    fn solve(&mut self) -> Result<(), ()> {
        let mut board: SolvingBoard = (*self).into();
        let mut board_changed = true;

        while board_changed {
            board_changed = false;
            // techinque where only one number is possible in a specific cell
            for x in 0..9 {
                for y in 0..9 {
                    let old_count = board.values[x][y].count();
                    if old_count > 1 {
                        let col = board.col(x);
                        let row = board.row(y);
                        let square = board.square(x, y);
                        let candidates = &mut board.values[x][y];
                        for num in col.into_iter().chain(row).chain(square) {
                            candidates.eliminate(num);
                        }
                        if candidates.is_invalid() {
                            return Err(());
                        }
                        if candidates.count() < old_count {
                            board_changed = true;
                        }
                    }
                }
            }
            // technique when a cell is an only possible place
            // to put a number in a col/row/square
            let cols = board.all_cols();
            let rows = board.all_rows();
            let squares = board.all_squares();
            for n in 1..10 {
                for group in cols.iter().chain(rows.iter()).chain(squares.iter()) {
                    if group.iter().filter(|&cell| cell.2.contains(n)).count() == 1 {
                        let (&x, &y) = group
                            .iter()
                            .find_map(|(x, y, candidates)| candidates.contains(n).then_some((x, y)))
                            .unwrap();
                        if board.values[x][y].is_invalid() {
                            return Err(());
                        }
                        if !board.values[x][y].is_definitive() {
                            board.values[x][y].leave_only(n);
                            board_changed = true;
                        }
                    }
                }
            }
        }

        for x in 0..9 {
            for y in 0..9 {
                let candidates = &board.values[x][y];
                self.values[x][y] = candidates.definitive_value();
            }
        }
        Ok(())
    }
}

impl SolvingBoard {
    fn col(&self, x: usize) -> Vec<u8> {
        self.values[x]
            .iter()
            .filter_map(|candidates| candidates.definitive_value())
            .collect()
    }
    fn row(&self, y: usize) -> Vec<u8> {
        self.values
            .iter()
            .filter_map(|col| col[y].definitive_value())
            .collect()
    }
    fn square(&self, x: usize, y: usize) -> Vec<u8> {
        self.values
            .iter()
            .enumerate()
            .filter(|(i, _)| (i / 3 == x / 3))
            .flat_map(|(_, col)| {
                col.iter()
                    .enumerate()
                    .filter(|(i, _)| i / 3 == y / 3)
                    .filter_map(|(_, candidates)| candidates.definitive_value())
                    .collect::<Vec<_>>()
            })
            .collect()
    }
    fn all_cols(&self) -> Vec<Vec<(usize, usize, SolvingCell)>> {
        self.values
            .into_iter()
            .enumerate()
            .map(|(x, col)| {
                col.into_iter()
                    .enumerate()
                    .map(|(y, cell)| (x, y, cell))
                    .collect()
            })
            .collect()
    }
    fn all_rows(&self) -> Vec<Vec<(usize, usize, SolvingCell)>> {
        (0..9)
            .map(|y| {
                self.values
                    .iter()
                    .enumerate()
                    .map(|(x, col)| (x, y, col[y]))
                    .collect()
            })
            .collect()
    }
    fn all_squares(&self) -> Vec<Vec<(usize, usize, SolvingCell)>> {
        (0..9)
            .map(|i| (i % 3, i / 3))
            .map(|(square_x, square_y)| {
                self.values
                    .iter()
                    .enumerate()
                    .filter(|(x, _)| x / 3 == square_x)
                    .flat_map(|(x, col)| {
                        col.iter()
                            .enumerate()
                            .filter(|(y, _)| y / 3 == square_y)
                            .map(|(y, &cell)| (x, y, cell))
                            .collect::<Vec<_>>()
                    })
                    .collect()
            })
            .collect()
    }
}

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App {
        selected: (0, 0),
        board: Board::default(),
        is_error: false,
    };
    loop {
        terminal.draw(|frame| {
            let frame_size = frame.size();
            if app.is_error {
                frame.render_widget(
                    Paragraph::new("Error happened while solving. Your input was invalid."),
                    frame_size,
                );
                return;
            }
            if frame_size.width < 11 || frame_size.height < 11 {
                frame.render_widget(
                    Paragraph::new("Your terminal window is too small. It must be at least 11x11."),
                    frame_size,
                );
                return;
            }

            // draw the table
            [
                (3, 0, 1, 11, Borders::LEFT),
                (7, 0, 1, 11, Borders::LEFT),
                (0, 3, 11, 1, Borders::TOP),
                (0, 7, 11, 1, Borders::TOP),
            ]
            .into_iter()
            .for_each(|(x, y, width, height, border_type)| {
                frame.render_widget(
                    Block::new().borders(border_type),
                    Rect::new(x, y, width, height),
                );
            });
            [(3, 3, 1, 1), (3, 7, 1, 1), (7, 3, 1, 1), (7, 7, 1, 1)]
                .into_iter()
                .for_each(|(x, y, width, height)| {
                    frame.render_widget(
                        Paragraph::new(symbols::line::CROSS),
                        Rect::new(x, y, width, height),
                    );
                });

            let main_layout =
                Layout::horizontal([Constraint::Length(12), Constraint::Min(0)]).split(frame_size);
            let instructions =
                Layout::vertical(repeat(Constraint::Length(1)).take(4)).split(main_layout[1]);

            // draw the instructions
            [
                vec!["Navigation ".into(), "<Arrows>".bold()],
                vec!["Solve ".into(), "<S>".bold()],
                vec!["Reset ".into(), "<R>".bold()],
                vec!["Quit ".into(), "<Q>".bold()],
            ]
            .into_iter()
            .enumerate()
            .for_each(|(i, line)| {
                frame.render_widget(Paragraph::new(Line::from(line)), instructions[i])
            });
            for x in 0..9 {
                for y in 0..9 {
                    let position = Rect::new(x + x / 3, y + y / 3, 1, 1);
                    let mut cell = Paragraph::new(
                        app.board
                            .get((x, y))
                            .map_or("-".to_text(), |num| num.to_text()),
                    );
                    if x == app.selected.0 && y == app.selected.1 {
                        cell = cell.reversed();
                    }
                    frame.render_widget(cell, position);
                }
            }
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            app.is_error = false;
            if let event::Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right => app.selected.0 = modulo_add(app.selected.0, 1, 9),
                    KeyCode::Left => app.selected.0 = modulo_add(app.selected.0, -1, 9),
                    KeyCode::Down => app.selected.1 = modulo_add(app.selected.1, 1, 9),
                    KeyCode::Up => app.selected.1 = modulo_add(app.selected.1, -1, 9),
                    KeyCode::Char(' ') => {
                        app.board.set(app.selected, None);
                        app.selected = inc_carry_over(app.selected, 9);
                    }
                    KeyCode::Char(character @ '1'..='9') => {
                        app.board
                            .set(app.selected, Some(character.to_digit(10).unwrap() as u8));
                        app.selected = inc_carry_over(app.selected, 9);
                    }
                    KeyCode::Char('s') => {
                        match app.board.solve() {
                            Ok(()) => {}
                            Err(()) => app.is_error = true,
                        };
                    }
                    KeyCode::Char('r') => {
                        app.board = Board::default();
                    }
                    _ => {}
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn modulo_add(value: u16, add: i32, modulo: u16) -> u16 {
    ((value as i32) + add).rem_euclid(modulo as i32) as u16
}

fn inc_carry_over((mut x, mut y): (u16, u16), modulo: u16) -> (u16, u16) {
    if x < modulo - 1 {
        x += 1;
    } else {
        x = 0;
        y = modulo_add(y, 1, modulo)
    }
    (x, y)
}
