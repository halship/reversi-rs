use super::resources::Resources;
use ggez::graphics::{self, DrawParam};
use ggez::{Context, GameResult};
use itertools::iproduct;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Stone(Stone),
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stone {
    Black,
    White,
}

impl Stone {
    fn reversed(&self) -> Stone {
        match self {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        }
    }
}

pub struct Board {
    inner: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Self {
        let mut inner = vec![vec![Cell::Empty; 10]; 10];
        for i in 0..10 {
            inner[i][0] = Cell::Wall;
            inner[i][9] = Cell::Wall;
            inner[0][i] = Cell::Wall;
            inner[9][i] = Cell::Wall;
        }
        inner[4][4] = Cell::Stone(Stone::White);
        inner[5][5] = Cell::Stone(Stone::White);
        inner[4][5] = Cell::Stone(Stone::Black);
        inner[5][4] = Cell::Stone(Stone::Black);

        Self { inner }
    }

    pub fn draw(&self, ctx: &mut Context, res: &mut Resources) -> GameResult {
        graphics::draw(ctx, &res.board, ([0.0, 0.0],))?;

        res.stones.clear();
        for (i, j) in iproduct!(1..9, 1..9) {
            let (x, y) = ((j - 1) as f32 * 50.0, (i - 1) as f32 * 50.0);
            match self.inner[i][j] {
                Cell::Stone(Stone::Black) => {
                    res.stones.add(
                        DrawParam::default()
                            .dest([x, y])
                            .src([0.0, 0.0, 0.5, 1.0].into()),
                    );
                }
                Cell::Stone(Stone::White) => {
                    res.stones.add(
                        DrawParam::default()
                            .dest([x, y])
                            .src([0.5, 0.0, 0.5, 1.0].into()),
                    );
                }
                _ => (),
            };
        }
        graphics::draw(ctx, &res.stones, ([0.0, 0.0],))
    }

    pub fn black_n(&self) -> usize {
        self.inner
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| **cell == Cell::Stone(Stone::Black))
            .count()
    }

    pub fn white_n(&self) -> usize {
        self.inner
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| **cell == Cell::Stone(Stone::White))
            .count()
    }

    pub fn reversible_n(&self, stone: Stone, i: usize, j: usize) -> usize {
        let mut result = 0;

        for (di, dj) in &DIRECTIONS {
            result += self.reversible_n_sub(stone, i, j, *di, *dj);
        }
        result
    }

    fn reversible_n_sub(&self, stone: Stone, i: usize, j: usize, di: isize, dj: isize) -> usize {
        let mut result = 0;
        let (mut ti, mut tj) = (i, j);
        let mut reversible = false;

        loop {
            ti = (ti as isize + di) as usize;
            tj = (tj as isize + dj) as usize;

            match &self.inner[ti][tj] {
                Cell::Stone(st) => {
                    if *st == stone.reversed() {
                        result += 1
                    } else {
                        reversible = true;
                        break;
                    }
                }
                _ => break,
            }
        }

        if reversible {
            result
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_board() {
        let board = Board::new();
        for (i, row) in board.inner.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match (i, j) {
                    (0, _) | (9, _) | (_, 0) | (_, 9) => assert_eq!(Cell::Wall, *cell),
                    (4, 4) | (5, 5) => assert_eq!(Cell::Stone(Stone::White), *cell),
                    (4, 5) | (5, 4) => assert_eq!(Cell::Stone(Stone::Black), *cell),
                    _ => assert_eq!(Cell::Empty, *cell),
                };
            }
        }
    }

    #[test]
    fn get_number_of_black_stones() {
        let board = Board::new();
        assert_eq!(2, board.black_n());
    }

    #[test]
    fn get_number_of_white_stones() {
        let board = Board::new();
        assert_eq!(2, board.white_n());
    }

    #[test]
    fn get_number_of_reversible() {
        let board = Board::new();
        assert_eq!(1, board.reversible_n(Stone::Black, 4, 3));
        assert_eq!(0, board.reversible_n(Stone::White, 4, 3));
    }
}
