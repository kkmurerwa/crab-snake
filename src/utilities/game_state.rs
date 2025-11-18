use crate::utilities::constants::{GRID_COLS, GRID_ROWS};
use crate::utilities::grid_point::GridPoint;
use crate::utilities::helpers::cell_to_pos;
use crate::utilities::player_direction::PlayerDirection;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::input::keyboard::KeyCode;
use ggez::mint::Point2;
use ggez::{event, Context, GameError};
use rand::Rng;
use std::time::{Duration, Instant};

pub struct GameState {
    last_update: Instant,
    player_pos: GridPoint,
    cell_w: f32,
    cell_h: f32,
    player_velocity: f32,
    player_direction: PlayerDirection,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        let screen_h_half = screen_h * 0.5;

        let cell_w = screen_w / GRID_COLS as f32;
        let cell_h = screen_h / GRID_ROWS as f32;

        let player_pos = GridPoint { x: 5, y: 5 };

        let player_velocity = 20.0;

        let mut rng = rand::rng();
        let x = rng.random_range(0..GRID_COLS);
        let y = rng.random_range(0..GRID_ROWS);

        let player_direction = PlayerDirection::Right;

        GameState {
            last_update: Instant::now(),
            cell_w,
            cell_h,
            player_pos,
            player_velocity,
            player_direction,
        }
    }

    pub fn get_player_pos(&self) -> Point2<f32> {
        let player_pos = cell_to_pos(
            self.player_pos.x as usize,
            self.player_pos.y as usize,
            self.cell_w,
            self.cell_h,
        );
        player_pos
    }

    pub fn player_exceeds_bounds(&self) -> bool {
        self.player_pos.x < 0
            || self.player_pos.x >= GRID_COLS as i32
            || self.player_pos.y < 0
            || self.player_pos.y >= GRID_ROWS as i32
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        if ctx.keyboard.is_key_pressed(KeyCode::W)
            && self.player_direction != PlayerDirection::Up
            && self.player_direction != PlayerDirection::Down
        {
            self.player_direction = PlayerDirection::Up;
        } else if ctx.keyboard.is_key_pressed(KeyCode::A)
            && self.player_direction != PlayerDirection::Left
            && self.player_direction != PlayerDirection::Right
        {
            self.player_direction = PlayerDirection::Left;
        } else if ctx.keyboard.is_key_pressed(KeyCode::S)
            && self.player_direction != PlayerDirection::Down
            && self.player_direction != PlayerDirection::Up
        {
            self.player_direction = PlayerDirection::Down;
        } else if ctx.keyboard.is_key_pressed(KeyCode::D)
            && self.player_direction != PlayerDirection::Right
            && self.player_direction != PlayerDirection::Left
        {
            self.player_direction = PlayerDirection::Right;
        }

        let now = Instant::now();
        if now.duration_since(self.last_update) < Duration::from_millis(500) {
            return Ok(());
        }

        self.last_update = now;

        if self.player_direction == PlayerDirection::Right {
            self.player_pos.x += 1;
        } else if self.player_direction == PlayerDirection::Left {
            self.player_pos.x -= 1;
        } else if self.player_direction == PlayerDirection::Down {
            self.player_pos.y += 1;
        } else if self.player_direction == PlayerDirection::Up {
            self.player_pos.y -= 1;
        }

        if self.player_exceeds_bounds() {
            println!("Game over!!!");
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        let player_pos = self.get_player_pos();
        let player_x_position = player_pos.x;
        let player_y_position = player_pos.y;
        let player_rect = Rect::new(
            player_x_position,
            player_y_position,
            self.cell_w,
            self.cell_h,
        );
        let player_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), player_rect, Color::WHITE)?;

        canvas.draw(&player_mesh, DrawParam::default());
        canvas.finish(ctx)
    }
}
