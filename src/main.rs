#![allow(unused)]
#![allow(dead_code)]

use ggez::{event, graphics, Context, GameError, GameResult};

struct GameState {

}

impl GameState {
    pub fn new() -> Self {
        GameState {}
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }
}

fn main() -> GameResult{
    let cb = ggez::ContextBuilder::new("crab_snake", "Murerwa");

    let (mut ctx, mut event_loop) = cb.build()?;

    ctx.gfx.set_window_title("CrabSnake");

    let mut state = GameState::new();

    event::run(ctx, event_loop, state);
}
