mod screen;
mod game;

fn main() {
    let screen = screen::Screen::new(10, 10).unwrap();

    screen.start().unwrap();
}
