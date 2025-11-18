use ggez::mint::Point2;

pub fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
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
