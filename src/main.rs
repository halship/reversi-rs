mod board;
mod information;
mod player;
mod resources;

use board::{Board, Stone};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::{graphics, timer, Context, ContextBuilder, GameResult};
use information::Information;
use player::Player;
use resources::Resources;

struct Game {
    res: Resources,
    board: Board,
    player: Vec<Player>,
    tern: usize,
    info: Information,
}

impl Game {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let res = Resources::new(ctx)?;
        let board = Board::new();
        let player = vec![
            Player::new_human(Stone::Black),
            Player::new_human(Stone::White),
        ];
        let info = Information::default();

        Ok(Self {
            res,
            board,
            player,
            tern: 0,
            info,
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, 60) {
            if self.player[self.tern].put_stone(&mut self.board) {
                self.tern = (self.tern + 1) % 2;
            }

            self.info.tern_stone = self.player[self.tern].stone();
            self.info.black_n = self.board.black_n();
            self.info.white_n = self.board.white_n();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        self.board.draw(ctx, &mut self.res)?;
        self.info.draw(ctx, self.res.font)?;
        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.player[self.tern].mouse_button_down(&button, x, y);
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
