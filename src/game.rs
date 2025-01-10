use std::{cmp::Ordering, fmt::Display};

type Liveness = bool;
type Board = Vec<Vec<Liveness>>;

const NEIGHBOR_MAX: usize = 3;
const NEIGHBOR_MIN: usize = 2;

struct Game {
    board: Board,
    generation: u32,
}

#[derive(Debug, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Game {
    /// Return the number of living neighbors of a given cell at pos
    /// Note that this method should reflect the boundary conditions
    fn count_neighbors(&self, coor: Coordinate) -> usize {
        let min = 0;
        let ymax = self.board.len() - 1;
        let xmax = self.board[0].len() - 1;
        let possible_neighbors = match [coor.x.cmp(&min), coor.y.cmp(&min), coor.x.cmp(&xmax), coor.y.cmp(&ymax)] {
            // Within the board
            [ Ordering::Greater, Ordering::Greater,
              Ordering::Less, Ordering::Less, ] =>  vec![
                self.board[coor.y][coor.x + 1],
                self.board[coor.y][coor.x - 1],
                self.board[coor.y + 1][coor.x],
                self.board[coor.y - 1][coor.x],
            ],

            // Out of bounds
            [Ordering::Less, _, _, _] 
                | [_, Ordering::Less, _, _] 
                | [_, _, Ordering::Greater, _] 
                | [_, _, _, Ordering::Greater] => {
                    panic!("Out of range neighbor ({}, {}) [BOUNDS: (min: {}, xmax: {}, ymax: {})]", 
                        coor.x, coor.y, min, xmax, ymax);
                },
            // 1x1 board
            [Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Equal] => return 0,

            // Corners
            // Top left
            [Ordering::Equal, Ordering::Equal, Ordering::Less, Ordering::Less] => vec![
                self.board[coor.y][coor.x + 1],
                self.board[coor.y + 1][coor.x],
            ],
            // Bottom left
            [Ordering::Equal, Ordering::Greater, Ordering::Less, Ordering::Equal] => vec![
                self.board[coor.y][coor.x + 1],
                self.board[coor.y - 1][coor.x],
            ],
            // Top right 
            [Ordering::Greater, Ordering::Equal, Ordering::Equal, Ordering::Less] => vec![
                self.board[coor.y][coor.x - 1],
                self.board[coor.y + 1][coor.x],
            ],
            // Bottom right
            [Ordering::Greater, Ordering::Greater, Ordering::Equal, Ordering::Equal] => vec![
                self.board[coor.y][coor.x - 1],
                self.board[coor.y - 1][coor.x],
            ],

            // Edges
            // Top edge
            [Ordering::Greater, Ordering::Equal, Ordering::Less, Ordering::Less] =>  vec![
                self.board[coor.y][coor.x + 1],
                self.board[coor.y][coor.x - 1],
                self.board[coor.y + 1][coor.x],
            ],
            // Bottom edge
            [Ordering::Greater, Ordering::Greater, Ordering::Less, Ordering::Equal] =>  vec![
                self.board[coor.y][coor.x - 1],
                self.board[coor.y][coor.x + 1],
                self.board[coor.y - 1][coor.x],
            ],
            // Left edge
            [Ordering::Equal, Ordering::Greater, Ordering::Less, Ordering::Less] => vec![
                self.board[coor.y][coor.x + 1],
                self.board[coor.y + 1][coor.x],
                self.board[coor.y - 1][coor.x],
            ],
            // Right edge
            [Ordering::Greater, Ordering::Greater, Ordering::Equal, Ordering::Less] => vec![
                self.board[coor.y][coor.x - 1],
                self.board[coor.y + 1][coor.x],
                self.board[coor.y - 1][coor.x],
            ],

            // 1-d boards (notated y 'by' x)
            // Middle of 1xN board
            [Ordering::Equal, Ordering::Greater, Ordering::Equal, Ordering::Less] => vec![
                self.board[coor.y][coor.x - 1], self.board[coor.y][coor.x + 1]
            ],
            // Middle of Nx1 board
            [Ordering::Greater, Ordering::Equal, Ordering::Less, Ordering::Equal] => vec![
                self.board[coor.y - 1][coor.x], self.board[coor.y + 1][coor.x]
            ],
            // Left edge of 1xN board
            [Ordering::Equal, Ordering::Equal, Ordering::Less, Ordering::Equal] => vec![self.board[coor.y][coor.x + 1]],
            // Top of Nx1 board
            [Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Less] => vec![self.board[coor.y + 1][coor.x]],
            // Bottom of Nx1 board
            [Ordering::Equal, Ordering::Greater, Ordering::Equal, Ordering::Equal] => vec![self.board[coor.y - 1][coor.x]],
            // Right edge of 1xN board
            [Ordering::Greater, Ordering::Equal, Ordering::Equal, Ordering::Equal] => vec![self.board[coor.y][coor.x - 1]],
        };

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

                // Recall the rules of the game:
                // 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                // 2. Any live cell with two or three live neighbours lives on to the next generation.
                // 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
                // 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                //
                // Which we can simplify to the following
                print!("{:?}, ", coord.clone());
                let num_neighbors = self.count_neighbors(coord);
                println!("num living neighbors: {}", num_neighbors);
                match [num_neighbors.cmp(&NEIGHBOR_MAX), num_neighbors.cmp(&NEIGHBOR_MIN)] {
                    [Ordering::Less, Ordering::Less]
                        | [Ordering::Greater, Ordering::Greater] => {next.board[i.0].push(false)},
                    [Ordering::Equal, _] => next.board[i.0].push(true),
                    _ => {
                        if self.board[i.0][j.0] {
                            next.board[i.0].push(true)
                        } else {
                            next.board[i.0].push(false)
                        }
                    },
                }
            }
        }

        next
    }


    fn print_board(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "# -> Alive, - -> Dead\n").unwrap();
        for i in self.board.clone().into_iter().enumerate() {
            for j in i.1.into_iter().enumerate() {
                if self.board[i.0][j.0] {
                    write!(f, "#").unwrap();
                } else {
                    write!(f, "-").unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Generation: {}\n", self.generation).unwrap();
        self.print_board(f)
    }
}

impl Iterator for Game {
    type Item = Game;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tick())
    }
}

#[cfg(test)]
mod test_game {
    use super::*;

    #[test]
    fn test_stable() {
        let board = vec![
            vec![true, true],
            vec![true, true],
        ];
        let game = Game {board, generation: 0};
        let next_game = game.tick();

        assert!(next_game.board == game.board);
    }

    #[test]
    fn test_dying_no_neighbors() {
        let iboard = vec![
            vec![true, false],
            vec![false, false],
        ];
        let fboard = vec![
            vec![false, false],
            vec![false, false],
        ];
        let istate = Game {board: iboard, generation: 0};
        let fstate = Game {board: fboard, generation: 0};
        let next_game = istate.tick();

        println!("{}", next_game);
        println!("{}", fstate);
        assert!(next_game.board == fstate.board);
    }

    #[test]
    fn test_dying_one_neighbor() {
        let iboard = vec![
            vec![true, true],
            vec![false, false],
        ];
        let fboard = vec![
            vec![false, false],
            vec![false, false],
        ];
        let istate = Game {board: iboard, generation: 0};
        let fstate = Game {board: fboard, generation: 0};
        let next_game = istate.tick();

        println!("{}", next_game);
        println!("{}", fstate);
        assert!(next_game.board == fstate.board);
    }

    #[test]
    fn test_stable_two_neighbors() {
        let iboard = vec![
            vec![true, true],
            vec![true, false],
        ];
        let fboard = vec![
            vec![true, false],
            vec![false, false],
        ];
        let istate = Game {board: iboard, generation: 0};
        let fstate = Game {board: fboard, generation: 0};
        let next_game = istate.tick();

        println!("{}", next_game);
        println!("{}", fstate);
        assert!(next_game.board == fstate.board);
    }

    #[test]
    fn test_stable_three_neighbors() {
        let iboard = vec![
            vec![false, false, false],
            vec![true, true, true],
            vec![false, true, false],
        ];
        let fboard = vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, false],
        ];
        let istate = Game {board: iboard, generation: 0};
        let fstate = Game {board: fboard, generation: 0};
        let next_game = istate.tick();

        println!("{}", next_game);
        println!("{}", fstate);
        assert!(next_game.board == fstate.board);
    }

    #[test]
    fn test_growth_three_neighbors() {
        let iboard = vec![
            vec![false, false, false],
            vec![true, false, true],
            vec![false, true, false],
        ];
        let fboard = vec![
            vec![false, false, false],
            vec![false, true, false],
            vec![false, false, false],
        ];
        let istate = Game {board: iboard, generation: 0};
        let fstate = Game {board: fboard, generation: 0};
        let next_game = istate.tick();

        println!("{}", next_game);
        println!("{}", fstate);
        assert!(next_game.board == fstate.board);
    }

    #[test]
    fn test_death_four_neighbors() {
        let iboard = vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ];
        let fboard = vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ];
        let istate = Game {board: iboard, generation: 0};
        let fstate = Game {board: fboard, generation: 0};
        let next_game = istate.tick();

        println!("{}", next_game);
        println!("{}", fstate);
        assert!(next_game.board == fstate.board);
    }
}
