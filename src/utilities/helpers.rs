use crate::utilities::constants::{GRID_COLS, GRID_ROWS};
use ggez::mint::Point2;
use rand::Rng;

pub fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

pub fn clamp_point(value: &mut i32, high: i32) {
    if *value < 0 {
        *value = 0;
    } else if *value > high {
        *value = high;
    }
}

pub fn cell_to_pos(col: usize, row: usize, cell_w: f32, cell_h: f32) -> Point2<f32> {
    Point2 {
        x: col as f32 * cell_w,
        y: row as f32 * cell_h,
    }
}

pub fn random_grid_point() -> (i32, i32) {
    let mut rng = rand::rng();
    let x = rng.random_range(0..GRID_COLS) as i32;
    let y = rng.random_range(0..GRID_ROWS) as i32;
    (x, y)
}
