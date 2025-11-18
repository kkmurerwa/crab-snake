use crate::utilities::constants::{
    FOOD_HEIGHT, FOOD_WIDTH, SCREEN_PADDING, SNAKE_HEIGHT, SNAKE_WIDTH,
};
use crate::utilities::helpers::clamp;
use crate::utilities::player_direction::PlayerDirection;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::input::keyboard::KeyCode;
use ggez::mint::Point2;
use ggez::{event, Context, GameError};
use rand::Rng;

pub struct GameState {
    player_pos: Point2<f32>,
    player_velocity: f32,
    player_direction: PlayerDirection,
    food_pos: Point2<f32>,
    screen_h: f32,
    screen_w: f32,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        let player_pos = Point2 {
            x: SCREEN_PADDING + SNAKE_WIDTH,
            y: screen_h_half,
        };
        let player_velocity = 50.0;

        let mut rng = rand::rng();
        let x = rng.random_range(0.0..screen_w);
        let y = rng.random_range(0.0..screen_h);

        let food_pos = Point2 { x, y };
        let player_direction = PlayerDirection::Right;

        GameState {
            player_pos,
            player_velocity,
            player_direction,
            food_pos,
            screen_h,
            screen_w,
        }
    }

    fn clamp_y(&mut self) {
        clamp(
            &mut self.food_pos.y,
            SCREEN_PADDING,
            self.screen_h - (SNAKE_HEIGHT + SNAKE_HEIGHT + SCREEN_PADDING),
        );
    }

    fn clamp_x(&mut self) {
        clamp(
            &mut self.food_pos.x,
            SCREEN_PADDING,
            self.screen_w - (SNAKE_WIDTH + SNAKE_WIDTH + SCREEN_PADDING),
        );
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let dt = ctx.time.delta().as_secs_f32();

        if ctx.keyboard.is_key_pressed(KeyCode::W) && self.player_direction != PlayerDirection::Up {
            self.player_direction = PlayerDirection::Up;
        } else if ctx.keyboard.is_key_pressed(KeyCode::A)
            && self.player_direction != PlayerDirection::Left
        {
            self.player_direction = PlayerDirection::Left;
        } else if ctx.keyboard.is_key_pressed(KeyCode::S)
            && self.player_direction != PlayerDirection::Down
        {
            self.player_direction = PlayerDirection::Down;
        } else if ctx.keyboard.is_key_pressed(KeyCode::D)
            && self.player_direction != PlayerDirection::Right
        {
            self.player_direction = PlayerDirection::Right;
        } else {
        }

        if self.player_direction == PlayerDirection::Right {
            self.player_pos.x += self.player_velocity * dt;
        } else if self.player_direction == PlayerDirection::Left {
            self.player_pos.x -= self.player_velocity * dt;
        } else if self.player_direction == PlayerDirection::Down {
            self.player_pos.y += self.player_velocity * dt;
        } else if self.player_direction == PlayerDirection::Up {
            self.player_pos.y -= self.player_velocity * dt;
        }

        self.clamp_y();
        self.clamp_x();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        self.clamp_y();
        self.clamp_x();

        let player_x_position = self.player_pos.x + SCREEN_PADDING;
        let player_y_position = self.player_pos.y + SCREEN_PADDING;
        let player_rect = Rect::new(
            player_x_position,
            player_y_position,
            FOOD_WIDTH,
            FOOD_HEIGHT,
        );
        let player_rect_mesh =
            Mesh::new_rectangle(ctx, DrawMode::fill(), player_rect, Color::WHITE)?;

        let food_x_position = self.food_pos.x + SCREEN_PADDING;
        let food_y_position = self.food_pos.y + SCREEN_PADDING;
        let food_circle_mesh = Mesh::new_circle(
            ctx,
            DrawMode::stroke(2.0),
            [food_x_position, food_y_position],
            SNAKE_WIDTH / 2.0,
            0.1,
            Color::WHITE,
        )?;

        canvas.draw(&player_rect_mesh, DrawParam::default());
        canvas.draw(&food_circle_mesh, DrawParam::default());
        canvas.finish(ctx)
    }
}
