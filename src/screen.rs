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
}

use crate::game::{self, Cell};
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

// draws an array of bools as blocks to screen based on width and size
pub fn start(mut game: game::Game) -> Result<(), io::Error> {
    // creating terminal
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // creating ui
    terminal.draw(|frame| build_screen(frame, &game))?;
    // be able to break out of main loop
    'main: loop {
        if crossterm::event::poll(Duration::from_millis(32))? {
            if let Ok(event) = event::read() {
                match event {
                    Event::Key(key) => match key.code {
                        // quit
                        Input::QUIT => break,
                        // generate new pattern
                        Input::GENERATE => {
                            game.generate();
                            terminal.draw(|frame| build_screen(frame, &game))?;
                        },
                        Input::NEXT => {
                            game.next_gen();
                            terminal.draw(|frame| build_screen(frame, &game))?;
                        }
                        // start animation
                        Input::ANIMATE => loop {
                            terminal.draw(|frame| build_screen(frame, &game))?;

                            game.next_gen();
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
                        terminal.draw(|frame| build_screen(frame, &game))?;
                    }
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

fn build_screen(frame: &mut Frame<CrosstermBackend<Stdout>>, game: &game::Game) {
    let size = frame.size();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);

    frame.render_widget(build_game(game), layout[0]);
}

fn build_game(game: &game::Game) -> Paragraph {
    // number of columns
    let mut spans = Vec::with_capacity(game.height);

    // do every row first
    for i in (0..game.size()).step_by(game.width) {
        let row = &game.cells[i..i + game.width];
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
