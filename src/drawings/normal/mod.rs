use crate::{
    pixels_table::DrawingContext,
    prelude::{ColorSelector, Drawable, RgbColor},
};

pub struct Point(RgbColor);

impl Point {
    pub fn new<C: ColorSelector>(color: C) -> Self {
        Self(color.rgb_color())
    }
}

impl Drawable<1, 1> for Point {
    fn setup(&self, drawing_ctx: &mut DrawingContext<1, 1>) {
        drawing_ctx.fill(self.0);
    }
}
