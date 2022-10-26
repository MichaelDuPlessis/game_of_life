mod game;
mod screen;

fn main() {
    let mut screen = screen::Screen::new(25, 25).unwrap();

    screen.start().unwrap();
}
