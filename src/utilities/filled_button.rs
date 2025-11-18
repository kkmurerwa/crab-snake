use ggez::graphics::{Color, DrawParam, Drawable, Rect};
use ggez::{graphics, Context};

pub struct FilledButton {
    pub(crate) rect: Rect,
    pub(crate) normal_color: Color,
}

impl FilledButton {
    pub fn draw(&self, canvas: &mut graphics::Canvas, ctx: &Context) {
        let color = self.normal_color;

        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.rect, color)
            .unwrap();

        canvas.draw(&mesh, DrawParam::default());
    }

    pub fn is_hovered(&self, mouse_pos: [f32; 2]) -> bool {
        self.rect.contains(mouse_pos)
    }
}
