use crate::utilities::constants::{GRID_COLS, GRID_ROWS};
use crate::utilities::current_screen::CurrentScreen;
use crate::utilities::filled_button::FilledButton;
use crate::utilities::grid_point::GridPoint;
use crate::utilities::helpers::{cell_to_pos, random_grid_point};
use crate::utilities::player_direction::PlayerDirection;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::input::keyboard::KeyCode;
use ggez::mint::Point2;
use ggez::{event, Context, GameError};
use rand::Rng;
use std::time::{Duration, Instant};

pub struct GameState {
    last_update: Instant,
    cell_w: f32,
    cell_h: f32,
    player_pos: GridPoint,
    player_direction: PlayerDirection,
    food_pos: GridPoint,
    current_screen: CurrentScreen,
    score: i32,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        let screen_h_half = screen_h * 0.5;

        let cell_w = screen_w / GRID_COLS as f32;
        let cell_h = screen_h / GRID_ROWS as f32;

        let player_pos = GridPoint { x: 5, y: 5 };

        let (x, y) = random_grid_point();
        let food_pos = GridPoint { x, y };

        GameState {
            last_update: Instant::now(),
            cell_w,
            cell_h,
            player_pos,
            player_direction: PlayerDirection::Right,
            food_pos,
            current_screen: CurrentScreen::GameScreen,
            score: 0,
        }
    }

    pub fn get_point_pos(&self, grid_point: &GridPoint) -> Point2<f32> {
        cell_to_pos(
            grid_point.x as usize,
            grid_point.y as usize,
            self.cell_w,
            self.cell_h,
        )
    }

    pub fn player_exceeds_bounds(&self) -> bool {
        self.player_pos.x < 0
            || self.player_pos.x >= GRID_COLS as i32
            || self.player_pos.y < 0
            || self.player_pos.y >= GRID_ROWS as i32
    }

    pub fn render_launch_screen(&self, canvas: &mut Canvas, ctx: &Context) {
        let button = FilledButton {
            rect: Rect::new(100.0, 100.0, 200.0, 60.0),
            normal_color: Color::from_rgb(100, 0, 0),
        };

        button.draw(canvas, ctx);
    }

    pub fn listen_and_render_launch_screen(&self, ctx: &mut Context) {}

    pub fn render_game_screen(&mut self, canvas: &mut Canvas, ctx: &Context) {
        let player_pos = self.get_point_pos(&self.player_pos);
        let player_rect = Rect::new(player_pos.x, player_pos.y, self.cell_w, self.cell_h);
        let player_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), player_rect, Color::WHITE)
            .expect("Failed to render player mesh!");
        canvas.draw(&player_mesh, DrawParam::default());

        let food_pos = self.get_point_pos(&self.food_pos);
        let center_x = food_pos.x + self.cell_w / 2.0;
        let center_y = food_pos.y + self.cell_h / 2.0;
        let radius = self.cell_w.min(self.cell_h) * 0.4;
        let food_mesh = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [center_x, center_y],
            radius,
            0.1,
            Color::GREEN,
        )
        .expect("Failed to render food mesh!");

        canvas.draw(&food_mesh, DrawParam::default());
    }

    pub fn listen_and_update_game_screen(&mut self, ctx: &Context) {
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

        if self.player_pos == self.food_pos {
            self.score += 1;

            println!("New score: {}", self.score);

            let (x, y) = random_grid_point();
            self.food_pos = GridPoint { x, y };
        }

        let now = Instant::now();
        if now.duration_since(self.last_update) < Duration::from_millis(500) {
            return;
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
            self.current_screen = CurrentScreen::ScoreScreen;
        }
    }

    pub fn render_score_screen(&self, canvas: &mut Canvas, ctx: &Context) {}
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        match self.current_screen {
            CurrentScreen::LaunchScreen => {
                self.listen_and_render_launch_screen(ctx);
            }
            CurrentScreen::GameScreen => {
                self.listen_and_update_game_screen(ctx);
            }
            CurrentScreen::ScoreScreen => {}
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        match self.current_screen {
            CurrentScreen::LaunchScreen => {
                self.render_launch_screen(&mut canvas, &ctx);
            }
            CurrentScreen::GameScreen => {
                self.render_game_screen(&mut canvas, &ctx);
            }
            CurrentScreen::ScoreScreen => {
                self.render_score_screen(&mut canvas, &ctx);
            }
        }

        canvas.finish(ctx)
    }
}
