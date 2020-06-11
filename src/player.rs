use crate::board::{Board, Stone};
use ggez::event::MouseButton;

pub enum Player {
    Human {
        stone: Stone,
        idx: Option<(usize, usize)>,
    },
}

impl Player {
    pub fn new_human(stone: Stone) -> Self {
        Self::Human { stone, idx: None }
    }

    pub fn stone(&self) -> Stone {
        match self {
            Player::Human { stone, .. } => *stone,
        }
    }

    pub fn put_stone(&self, board: &mut Board) -> bool {
        match self {
            Player::Human {
                stone,
                idx: Some((i, j)),
            } => {
                if board.reversible_n(*stone, *i, *j) != 0 {
                    board.set(*stone, *i, *j);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn mouse_button_down(&mut self, button: &MouseButton, x: f32, y: f32) {
        match self {
            Player::Human { idx, .. } => {
                if *button == MouseButton::Left {
                    *idx = Some(((y / 50.0) as usize + 1, (x / 50.0) as usize + 1));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_on_mouse_button_down() {
        let mut player = Player::new_human(Stone::Black);
        player.mouse_button_down(&MouseButton::Left, 120.0, 230.0);
        match player {
            Player::Human {
                idx: Some((i, j)), ..
            } => {
                assert_eq!(5, i);
                assert_eq!(3, j);
            }
            _ => (),
        }
    }
}
