/*
    functions to draw to screen
*/

// enum representing types of input
#[non_exhaustive]
struct Input {}

impl Input {
    const ANIMATE: KeyCode = KeyCode::Char('a');
    const QUIT: KeyCode = KeyCode::Char('q');
    const STOP_ANIMATION: KeyCode = KeyCode::Char('s');
    const GENERATE: KeyCode = KeyCode::Char('g');
    const NEXT: KeyCode = KeyCode::Char('n');
    const INPUT: KeyCode = KeyCode::Char('i');
}

use crate::game::{Cell, Game};
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
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

// enum to represent what mode the app is in
// i.e. inputing text or normal
enum Mode {
    Normal,
    Editing,
}

pub struct Screen {
    game: Game,
    mode: Mode,
    input: String,
}

impl Screen {
    pub fn new(game: Game) -> Self {
        Self {
            game,
            mode: Mode::Normal,
            input: String::new(),
        }
    }

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
                if let Ok(event) = event::read() {
                    match self.mode {
                        Mode::Normal => match event {
                            Event::Key(key) => match key.code {
                                // quit
                                Input::QUIT => break,
                                // generate new pattern
                                Input::GENERATE => {
                                    self.game.generate();
                                    terminal.draw(|frame| self.build_screen(frame))?;
                                }
                                // animating one step forward
                                Input::NEXT => {
                                    self.game.next_gen();
                                    terminal.draw(|frame| self.build_screen(frame))?;
                                }
                                Input::INPUT => {
                                    self.mode = Mode::Editing;
                                    terminal.draw(|frame| self.build_screen(frame))?;
                                }
                                // start animation
                                Input::ANIMATE => loop {
                                    terminal.draw(|frame| self.build_screen(frame))?;

                                    self.game.next_gen();
                                    sleep(Duration::from_millis(100));

                                    if (crossterm::event::poll(Duration::from_millis(1)))? {
                                        if let Event::Key(k) = event::read()? {
                                            match k.code {
                                                // stop animation
                                                Input::STOP_ANIMATION => break,
                                                // quit
                                                Input::QUIT => break 'main,
                                                _ => (),
                                            }
                                        }
                                    }
                                },
                                _ => (),
                            },
                            Event::Resize(_, _) => {
                                terminal.draw(|frame| self.build_screen(frame))?;
                            }
                            _ => (),
                        },
                        Mode::Editing => {
                            match event {
                                Event::Key(key) => match key.code {
                                    KeyCode::Enter => {
                                        match Game::from_file(&self.input) {
                                            Ok(game) => self.game = game,
                                            Err(_) => self.input = String::from("File not found."),
                                        }

                                        self.mode = Mode::Normal;
                                    }
                                    KeyCode::Char(c) => {
                                        self.input.push(c);
                                    }
                                    KeyCode::Backspace => {
                                        self.input.pop();
                                    }
                                    KeyCode::Esc => {
                                        self.mode = Mode::Normal;
                                    }
                                    _ => (),
                                },
                                _ => (),
                            }

                            terminal.draw(|frame| self.build_screen(frame))?;
                        }
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

    // draws an array of bools as blocks to screen based on width and size
    fn build_screen(&self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let size = frame.size();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(95), Constraint::Percentage(5)].as_ref())
            .split(size);

        frame.render_widget(self.build_game(), layout[0]);
        frame.render_widget(self.build_input(), layout[1]);

        match self.mode {
            Mode::Normal => (),
            Mode::Editing => {
                frame.set_cursor(
                    // Put cursor past the end of the input text
                    layout[1].x + self.input.len() as u16 + 1,
                    // Move one line down, from the border to the input line
                    layout[1].y + 1,
                )
            }
        }
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

    fn build_input<'a>(&'a self) -> Paragraph<'a> {
        let block = Block::default().borders(Borders::ALL);

        Paragraph::new(Span::from(self.input.as_str()))
            .style(Style::default().bg(Color::Black))
            .block(block)
    }
}
