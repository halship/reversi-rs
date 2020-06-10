use super::resources::Resources;
use ggez::{graphics, Context, GameResult};

pub struct Board {}

impl Board {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, ctx: &mut Context, res: &mut Resources) -> GameResult {
        graphics::draw(ctx, &res.board, ([0.0, 0.0],))
    }
}
