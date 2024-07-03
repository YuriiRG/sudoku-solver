use std::{io::stdout, num::NonZeroU8};

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
    selected: (u16, u16),
    board: Board,
}

#[derive(Clone, Copy, Debug, Default)]
struct Board {
    values: [[Option<NonZeroU8>; 9]; 9],
}

impl Board {
    fn set(&mut self, coords: (u16, u16), value: Option<NonZeroU8>) {
        self.values[coords.0 as usize][coords.1 as usize] = value;
    }
    fn get(&mut self, coords: (u16, u16)) -> Option<NonZeroU8> {
        self.values[coords.0 as usize][coords.1 as usize]
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
    };
    loop {
        terminal.draw(|frame| {
            let frame_size = frame.size();
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
                    KeyCode::Char(character @ '1'..='9') => {
                        app.board.set(
                            app.selected,
                            Some(NonZeroU8::new(character.to_digit(10).unwrap() as u8).unwrap()),
                        );
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
