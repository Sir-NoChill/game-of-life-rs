use std::cmp::Ordering;

type Liveness = bool;
type Board = Vec<Vec<Liveness>>;

const NEIGHBOR_THRESHOLD: usize = 3;

struct Game {
    board: Board,
    generation: u32,
}

struct Coordinate {
    x: usize,
    y: usize,
}

impl Game {
    /// Return the number of living neighbors of a given cell at pos
    fn count_neighbors(&self, coor: Coordinate) -> usize {
        let possible_neighbors = vec![
            self.board[coor.x][coor.y + 1],
            self.board[coor.x][coor.y - 1],
            self.board[coor.x + 1][coor.y],
            self.board[coor.x - 1][coor.y],
        ];

        // Count all the living neighbors
        possible_neighbors.iter().filter(|&&b| b).count()
    }

    fn check_underpop(&self) {
        todo!()
    }

    fn check_lives_on(&self) {
        todo!()
    }

    fn check_overpop(&self) {
        todo!()
    }

    fn check_reproduce(&self) {
        todo!()
    }

    fn get_dying(&self) {
        todo!()
    }

    fn get_living(&self) {
        todo!()
    }

    /// Advance the game by a single tick.
    ///
    /// In the game of life, births and deaths happen simultaneously according
    /// to the previous state of the game.
    fn tick(&self) -> Game {
        let mut next = Game {board: Board::new(), generation: self.generation + 1};

        for i in self.board.clone().into_iter().enumerate() {
            next.board.push(Vec::new());
            for j in i.1.into_iter().enumerate() {
                let coord = Coordinate {x: j.0, y: i.0};
                match self.count_neighbors(coord).cmp(&NEIGHBOR_THRESHOLD) {
                    Ordering::Less | Ordering::Greater => {next.board[i.0].push(false)},
                    Ordering::Equal => {next.board[i.0].push(true)},
                }
            }
        }

        next
    }
}

impl Iterator for Game {
    type Item = Game;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tick())
    }
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test_game {
    use super::*;

    fn test_stable() {
    }
}
