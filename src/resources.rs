use ggez::graphics::{spritebatch::SpriteBatch, Font, Image};
use ggez::{Context, GameResult};

pub struct Resources {
    pub board: Image,
    pub stones: SpriteBatch,
    pub font: Font,
}

impl Resources {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let board = Image::new(ctx, "/board.png")?;
        let stones = SpriteBatch::new(Image::new(ctx, "/stones.png")?);
        let font = Font::new(ctx, "/mplus-1p-regular.ttf")?;

        Ok(Self {
            board,
            stones,
            font,
        })
    }
}
