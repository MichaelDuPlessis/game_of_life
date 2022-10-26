mod game;
mod screen;

fn main() -> Result<(), std::io::Error> {
    screen::start(game::Game::new(20, 15))?;
    
    Ok(())
}
