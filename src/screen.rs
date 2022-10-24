/*
    Used to draw to screen
*/

use std::{time::Duration, thread::{self, sleep}, io::{self, Stdout}};
use crossterm::{terminal::{disable_raw_mode, LeaveAlternateScreen, enable_raw_mode}, execute, event::{DisableMouseCapture, Event, self, KeyCode}};
use tui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders, Paragraph}, layout::{Layout, Direction, Constraint, Alignment}, Frame, text::{Span, Spans}, style::{Style, Color}};
use crate::game::{Game, self, Cell};

pub struct Screen {
    width: usize,
    height: usize,
    game: Game,
}

impl Screen {
    // creates a new screen from width and height
    pub fn new(width: usize, height: usize) -> Result<Self, io::Error> {
        Ok(Self {
            width,
            height,
            game: game::Game::new(width * height),
        })
    }

    // draws an array of bools as blocks to screen based on width and size
    pub fn start(&self) -> Result<(), io::Error> {
        // creating terminal
        enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        
        // creating ui
        terminal.draw(|frame| self.build_screen(frame))?;
        loop {
            let timeout = Duration::from_millis(32);
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('a') => loop {
                            terminal.draw(|frame| self.build_screen(frame))?;
                            sleep(Duration::from_millis(32));

                            if (crossterm::event::poll(Duration::from_millis(1)))? {
                                if let Event::Key(k) = event::read()? {
                                    if k.code == KeyCode::Char('s') {
                                        break;
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
        let mut spans = Vec::with_capacity(self.height);

        // do every row first
        for i in (0..self.width*self.height).step_by(self.width) {
            let row = &self.game.cells[i..i + self.width];
            let mut text = String::new();

            // create the cells
            for cell in row {
                // maybe convert to some thing that pads string
                text.push_str(
                    if cell == &Cell::Alive {
                        "⬜"
                    } else {
                        "  " // two spaces to match above string
                    }
                );
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