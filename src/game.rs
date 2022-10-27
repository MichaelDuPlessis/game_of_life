// the game logic
type Position = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Alive,
    Dead,
}

pub struct Game {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::with_capacity(width * height);

        for _ in 0..width * height {
            cells.push(if rand::random() {
                Cell::Alive
            } else {
                Cell::Dead
            });
        }

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn generate(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = if rand::random() {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }
    }

    pub fn with_initial(width: usize, height: usize, cells: Vec<Cell>) -> Self {
        assert_eq!(width * height, cells.len());

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn next_gen(&mut self) {
        let mut new_cells = Vec::<Cell>::with_capacity(self.size());

        for (i, cell) in self.cells.iter().enumerate() {
            let neighbours = self.count_neighbours(i);

            match (cell, neighbours) {
                (Cell::Alive, 2..=3) => new_cells.push(Cell::Alive),
                (Cell::Dead, 3) => new_cells.push(Cell::Alive),
                _ => new_cells.push(Cell::Dead),
            }
        }

        self.cells = new_cells;
    }

    fn count_neighbours(&self, pos: usize) -> u8 {
        let mut neighbours = 0;
        let pos = (
            (pos as isize - (self.width * (pos / self.width)) as isize),
            (pos / self.width) as isize,
        );

        let alive_neighbours = [
            (pos.0 - 1, pos.1),     // -1, 0
            (pos.0 + 1, pos.1),     // 1, 0
            (pos.0, pos.1 - 1),     // 0, -1
            (pos.0, pos.1 + 1),     // 0, 1
            (pos.0 - 1, pos.1 - 1), // -1, -1
            (pos.0 - 1, pos.1 + 1), // -1, 1
            (pos.0 + 1, pos.1 - 1), // 1, -1
            (pos.0 + 1, pos.1 + 1), // 1, 1
        ];

        // mapping position moves to actualy usable coords
        let alive_neighbours: Vec<Position> = alive_neighbours
            .into_iter()
            .map(|el| {
                (
                    el.0.rem_euclid(self.width as isize) as usize,
                    el.1.rem_euclid(self.height as isize) as usize,
                )
            })
            .collect();

        for pos in alive_neighbours.into_iter() {
            if self[pos] == Cell::Alive {
                neighbours += 1;
            }
        }

        neighbours
    }
}

// used to index with Position tuple
impl std::ops::Index<Position> for Game {
    type Output = Cell;

    fn index(&self, index: Position) -> &Self::Output {
        &self.cells[index.0 + index.1 * self.width]
    }
}

impl std::ops::IndexMut<Position> for Game {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.cells[index.0 + index.1 * self.width]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // looks to see if after a generation all neighbours are dead
    fn all_dead() {
        let mut game = Game::with_initial(10, 10, vec![Cell::Dead; 100]);

        for c in game.cells.iter() {
            assert_eq!(c, &Cell::Dead);
        }

        game.next_gen();

        for c in game.cells.iter() {
            assert_eq!(c, &Cell::Dead);
        }
    }

    #[test]
    // checking to see if after a 3x3 grid works
    fn next_gen() {
        let cells = vec![
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
        ];

        let mut game = Game::with_initial(3, 3, cells);

        game.next_gen();

        assert_eq!(
            game.cells,
            vec![
                Cell::Alive,
                Cell::Alive,
                Cell::Alive,
                Cell::Alive,
                Cell::Alive,
                Cell::Alive,
                Cell::Alive,
                Cell::Alive,
                Cell::Alive,
            ]
        );
    }

    #[test]
    // checking to see if after a 3x3 grid works
    fn next_gen_non_square() {
        let cells = vec![
            Cell::Alive,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
        ];

        let mut game = Game::with_initial(4, 3, cells);

        game.next_gen();

        assert_eq!(
            game.cells,
            vec![
                Cell::Dead,
                Cell::Alive,
                Cell::Alive,
                Cell::Dead,
                Cell::Dead,
                Cell::Alive,
                Cell::Alive,
                Cell::Dead,
                Cell::Dead,
                Cell::Alive,
                Cell::Alive,
                Cell::Alive,
            ]
        );
    }

    #[test]
    // checking to see if after a 3x3 grid neighbours are correct
    fn count_neighbours() {
        let cells = vec![
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Dead,
        ];

        let counts = vec![3, 2, 3, 2, 3, 2, 3, 3, 3];

        let game = Game::with_initial(3, 3, cells);

        for (count, (i, _)) in counts.iter().zip(game.cells.iter().enumerate()) {
            assert_eq!(*count, game.count_neighbours(i))
        }
    }
}

// [Dead, Alive, Alive, Dead, Dead, Alive, Alive, Dead, Dead, Alive, Alive, Alive]
// [Dead, Alive, Alive, Dead, Alive, Alive, Alive, Dead, Dead, Alive, Alive, Dead]
