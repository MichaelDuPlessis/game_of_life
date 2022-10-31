mod game;
mod screen;

fn main() -> Result<(), std::io::Error> {
    let mut screen = screen::Screen::new(game::Game::new(30, 30));
    screen.start()?;
    
    Ok(())
}
