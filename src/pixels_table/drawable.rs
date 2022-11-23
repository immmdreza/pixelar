use crate::prelude::Canvas;

use super::ColorSelector;

#[derive(Debug, Default)]
pub struct DrawingContext<const H: usize, const W: usize> {
    canvas: Canvas<H, W>,
}

impl<const H: usize, const W: usize> DrawingContext<H, W> {
    pub fn canvas(&self) -> &Canvas<H, W> {
        &self.canvas
    }

    pub fn fill<C: ColorSelector>(&mut self, color: C) {
        self.canvas.fill(color)
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas<H, W> {
        &mut self.canvas
    }
}

pub trait Drawable<const H: usize, const W: usize> {
    fn setup(&self, drawing_ctx: &mut DrawingContext<H, W>);
}
