// the game logic

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Alive,
    Dead,
}

pub struct Game {
    pub cells: Vec<Cell>,
}

impl Game {
    pub fn new(size: usize) -> Self {
        let mut cells = Vec::with_capacity(size);

        for _ in 0..size {
            cells.push(
                if rand::random() {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            );
        }

        Self {
            cells,
        }
    }
}