/*
    Used to draw to screen
*/

use crate::game::{self, Cell, Game};
use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use std::{
    io::{self, Stdout},
    thread::sleep,
    time::Duration,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

pub struct Screen {
    game: Game,
}

impl Screen {
    // creates a new screen from width and height
    pub fn new(width: usize, height: usize) -> Result<Self, io::Error> {
        Ok(Self {
            game: game::Game::new(width, height),
        })
    }

    // draws an array of bools as blocks to screen based on width and size
    pub fn start(&mut self) -> Result<(), io::Error> {
        // creating terminal
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // creating ui
        terminal.draw(|frame| self.build_screen(frame))?;
        // be able to break out of main loop
        'main: loop {
            if crossterm::event::poll(Duration::from_millis(32))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('a') => loop {
                            terminal.draw(|frame| self.build_screen(frame))?;

                            self.game.next_gen();
                            sleep(Duration::from_millis(100));

                            if (crossterm::event::poll(Duration::from_millis(1)))? {
                                if let Event::Key(k) = event::read()? {
                                    match k.code {
                                        KeyCode::Char('s') => break,
                                        KeyCode::Char('q') => break 'main,
                                        _ => (),
                                    }
                                }
                            }
                        },
                        _ => (),
                    }
                }
            }
        }

        // resetting terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn build_screen(&self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let size = frame.size();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);

        frame.render_widget(self.build_game(), layout[0]);
    }

    fn build_game(&self) -> Paragraph {
        // number of columns
        let mut spans = Vec::with_capacity(self.game.height);

        // do every row first
        for i in (0..self.game.size()).step_by(self.game.width) {
            let row = &self.game.cells[i..i + self.game.width];
            let mut text = String::new();

            // create the cells
            for cell in row {
                // maybe convert to some thing that pads string
                text.push_str(if cell == &Cell::Alive {
                    "⬜"
                } else {
                    "  " // two spaces to match above string
                });
            }

            spans.push(Spans::from(text))
        }

        let block = Block::default()
            .title("Game Of Life")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL);

        Paragraph::new(spans)
            .style(Style::default().bg(Color::Black))
            .block(block)
    }
}
