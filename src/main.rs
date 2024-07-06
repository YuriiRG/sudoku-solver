use std::io::stdout;

use anyhow::Result;

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::Rect,
    style::Stylize,
    text::ToText,
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

#[derive(Clone, Debug, Default)]
struct SolvingBoard {
    values: [[Vec<u8>; 9]; 9],
}

impl From<Board> for SolvingBoard {
    fn from(board: Board) -> Self {
        Self {
            values: board.values.map(|col| {
                col.map(|cell| match cell {
                    Some(value) => vec![value],
                    None => vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
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
                    let old_len = board.values[x][y].len();
                    if old_len > 1 {
                        let col = board.col(x);
                        let row = board.row(y);
                        let square = board.square(x, y);
                        let candidates = &mut board.values[x][y];
                        for num in col.into_iter().chain(row).chain(square) {
                            candidates.retain(|&candidate| candidate != num);
                        }
                        if candidates.len() < old_len {
                            board_changed = true;
                        }
                        if candidates.is_empty() {
                            return Err(());
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
                    if group.iter().filter(|&cell| cell.2.contains(&n)).count() == 1 {
                        let (&x, &y) = group
                            .iter()
                            .find_map(|(x, y, candidates)| {
                                if candidates.contains(&n) {
                                    Some((x, y))
                                } else {
                                    None
                                }
                            })
                            .unwrap();
                        if board.values[x][y].len() > 1 {
                            board.values[x][y].retain(|&candidate| candidate == n);
                            board_changed = true;
                        }
                        if board.values[x][y].is_empty() {
                            return Err(());
                        }
                    }
                }
            }
        }

        for x in 0..9 {
            for y in 0..9 {
                let candidates = &board.values[x][y];
                if candidates.len() == 1 {
                    self.values[x][y] = Some(candidates[0]);
                }
            }
        }
        Ok(())
    }
}

impl SolvingBoard {
    fn col(&self, x: usize) -> Vec<u8> {
        self.values[x]
            .iter()
            .filter_map(|candidates| {
                if candidates.len() == 1 {
                    Some(candidates[0])
                } else {
                    None
                }
            })
            .collect()
    }
    fn row(&self, y: usize) -> Vec<u8> {
        self.values
            .iter()
            .filter_map(|col| {
                if col[y].len() == 1 {
                    Some(col[y][0])
                } else {
                    None
                }
            })
            .collect()
    }
    fn square(&self, x: usize, y: usize) -> Vec<u8> {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(i, col)| {
                if i / 3 == x / 3 {
                    Some(
                        col.iter()
                            .enumerate()
                            .filter_map(|(i, candidates)| {
                                if i / 3 == y / 3 && candidates.len() == 1 {
                                    Some(candidates[0])
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>(),
                    )
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }
    fn all_cols(&self) -> Vec<Vec<(usize, usize, Vec<u8>)>> {
        self.values
            .clone()
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
    fn all_rows(&self) -> Vec<Vec<(usize, usize, Vec<u8>)>> {
        (0..9)
            .map(|y| {
                self.values
                    .iter()
                    .enumerate()
                    .map(|(x, col)| (x, y, col[y].clone()))
                    .collect()
            })
            .collect()
    }
    fn all_squares(&self) -> Vec<Vec<(usize, usize, Vec<u8>)>> {
        (0..9)
            .map(|i| (i % 3, i / 3))
            .map(|(square_x, square_y)| {
                self.values
                    .iter()
                    .enumerate()
                    .filter_map(|(x, col)| {
                        if x / 3 == square_x {
                            Some(
                                col.iter()
                                    .enumerate()
                                    .filter_map(|(y, cell)| {
                                        if y / 3 == square_y {
                                            Some((x, y, cell.clone()))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<_>>(),
                            )
                        } else {
                            None
                        }
                    })
                    .flatten()
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
            frame.render_widget(Block::new().borders(Borders::LEFT), Rect::new(3, 0, 1, 11));
            frame.render_widget(Block::new().borders(Borders::LEFT), Rect::new(7, 0, 1, 11));
            frame.render_widget(Block::new().borders(Borders::TOP), Rect::new(0, 3, 11, 1));
            frame.render_widget(Block::new().borders(Borders::TOP), Rect::new(0, 7, 11, 1));

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
                    KeyCode::Right => {
                        app.selected.0 =
                            ((app.selected.0 as i16).wrapping_add(1).rem_euclid(9)) as u16
                    }
                    KeyCode::Left => {
                        app.selected.0 =
                            ((app.selected.0 as i16).wrapping_sub(1).rem_euclid(9)) as u16
                    }
                    KeyCode::Down => {
                        app.selected.1 =
                            ((app.selected.1 as i16).wrapping_add(1).rem_euclid(9)) as u16
                    }
                    KeyCode::Up => {
                        app.selected.1 =
                            ((app.selected.1 as i16).wrapping_sub(1).rem_euclid(9)) as u16
                    }
                    KeyCode::Char(' ') => {
                        app.board.set(app.selected, None);
                        if app.selected.0 < 8 {
                            app.selected.0 += 1;
                        } else {
                            app.selected.0 = 0;
                            app.selected.1 =
                                ((app.selected.1 as i16).wrapping_add(1).rem_euclid(9)) as u16
                        }
                    }
                    KeyCode::Char(character @ '1'..='9') => {
                        app.board
                            .set(app.selected, Some(character.to_digit(10).unwrap() as u8));
                        if app.selected.0 < 8 {
                            app.selected.0 += 1;
                        } else {
                            app.selected.0 = 0;
                            app.selected.1 =
                                ((app.selected.1 as i16).wrapping_add(1).rem_euclid(9)) as u16
                        }
                    }
                    KeyCode::Char('s') => {
                        match app.board.solve() {
                            Ok(()) => {}
                            Err(()) => app.is_error = true,
                        };
                    }
                    _ => {}
                }
            }
        }
    }
    // TODO main loop

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
