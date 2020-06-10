use super::resources::Resources;
use ggez::{graphics, Context, GameResult};

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Stone(Stone),
    Wall,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stone {
    Black,
    White,
}

pub struct Board {
    inner: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new() -> Self {
        let mut inner = vec![vec![Cell::Empty; 9]; 9];
        for i in 0..9 {
            inner[i][0] = Cell::Wall;
            inner[i][8] = Cell::Wall;
            inner[0][i] = Cell::Wall;
            inner[8][i] = Cell::Wall;
        }
        inner[4][4] = Cell::Stone(Stone::White);
        inner[5][5] = Cell::Stone(Stone::White);
        inner[4][5] = Cell::Stone(Stone::Black);
        inner[5][4] = Cell::Stone(Stone::Black);

        Self { inner }
    }

    pub fn draw(&self, ctx: &mut Context, res: &mut Resources) -> GameResult {
        graphics::draw(ctx, &res.board, ([0.0, 0.0],))
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
                    (0, _) | (8, _) | (_, 0) | (_, 8) => assert_eq!(Cell::Wall, *cell),
                    (4, 4) | (5, 5) => assert_eq!(Cell::Stone(Stone::White), *cell),
                    (4, 5) | (5, 4) => assert_eq!(Cell::Stone(Stone::Black), *cell),
                    _ => assert_eq!(Cell::Empty, *cell),
                };
            }
        }
    }
}
