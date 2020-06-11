use crate::board::Stone;
use ggez::graphics::{self, Font, Scale, Text, TextFragment};
use ggez::{Context, GameResult};

pub struct Information {
    pub tern_stone: Stone,
    pub black_n: usize,
    pub white_n: usize,
}

impl Information {
    pub fn draw(&self, ctx: &mut Context, font: Font) -> GameResult {
        let mut text = Text::new(
            TextFragment::new(format!("{}の番です\n\n", self.tern_stone))
                .font(font)
                .scale(Scale::uniform(24.0)),
        );
        text.add(
            TextFragment::new(format!("黒: {}\n白: {}", self.black_n, self.white_n))
                .font(font)
                .scale(Scale::uniform(20.0)),
        );
        graphics::draw(ctx, &text, ([410.0, 10.0],))
    }
}

impl Default for Information {
    fn default() -> Self {
        Self {
            tern_stone: Stone::Black,
            black_n: 0,
            white_n: 0,
        }
    }
}
