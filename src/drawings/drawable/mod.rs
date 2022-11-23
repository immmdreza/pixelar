use crate::{
    pixels_table::DrawingContext,
    prelude::{ColorSelector, Drawable, RgbColor},
    Position, ToPosition,
};

pub struct Point<const H: usize, const W: usize> {
    color: RgbColor,
    pos: Position,
}

impl<const H: usize, const W: usize> Point<H, W> {
    pub fn new<C: ColorSelector, P: ToPosition<H, W>>(color: C, pos: P) -> Self {
        Self {
            color: color.get_rgb_color(),
            pos: pos.get_position(),
        }
    }
}

impl<const H: usize, const W: usize> Drawable<H, W, 1, 1> for Point<H, W> {
    fn setup(&self, drawing_ctx: &mut DrawingContext<H, W, 1, 1>) {
        drawing_ctx.fill(self.color);
        drawing_ctx.set_draw_pos(self.pos)
    }
}
