mod board;
mod resources;

use board::Board;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::{graphics, timer, Context, ContextBuilder, GameResult};
use resources::Resources;

struct Game {
    res: Resources,
    board: Board,
}

impl Game {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let res = Resources::new(ctx)?;
        let board = Board::new();

        Ok(Self { res, board })
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 60) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        self.board.draw(ctx, &mut self.res)?;
        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    let (mut ctx, mut events_loop) = ContextBuilder::new("reversi", "halship")
        .window_setup(WindowSetup::default().title("リバーシ"))
        .window_mode(WindowMode::default().dimensions(640.0, 480.0))
        .add_resource_path("./resources")
        .build()?;
    let mut game = Game::new(&mut ctx)?;

    event::run(&mut ctx, &mut events_loop, &mut game)
}
