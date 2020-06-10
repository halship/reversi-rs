use ggez::graphics::{spritebatch::SpriteBatch, Image};
use ggez::{Context, GameResult};

pub struct Resources {
    pub board: Image,
    pub stones: SpriteBatch,
}

impl Resources {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let board = Image::new(ctx, "/board.png")?;
        let stones = SpriteBatch::new(Image::new(ctx, "/stones.png")?);

        Ok(Self { board, stones })
    }
}
