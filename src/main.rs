mod game;
mod screen;

fn main() -> Result<(), std::io::Error> {
    screen::start(game::Game::new(30, 30))?;
    
    Ok(())
}
