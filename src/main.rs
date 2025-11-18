#![allow(unused)]
#![allow(dead_code)]

mod utilities;

use crate::utilities::game_state::GameState;
use ggez::{event, GameResult};

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("crab_snake", "Murerwa");

    let (mut ctx, mut event_loop) = cb.build()?;

    ctx.gfx.set_window_title("CrabSnake");

    let mut state = GameState::new(&mut ctx);

    event::run(ctx, event_loop, state);
}
