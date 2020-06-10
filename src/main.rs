use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::{graphics, timer, Context, ContextBuilder, GameResult};

const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

struct Game {}

impl Game {
    fn new(_ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {})
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 60) {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    let (mut ctx, mut events_loop) = ContextBuilder::new("reversi", "halship")
        .window_setup(WindowSetup::default().title("リバーシ"))
        .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;
    let mut game = Game::new(&mut ctx)?;

    event::run(&mut ctx, &mut events_loop, &mut game)
}
