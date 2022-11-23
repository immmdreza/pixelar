use crate::{prelude::Canvas, Position, ToPosition};

use super::ColorSelector;

#[derive(Debug, Default)]
pub struct DrawingContext<const H: usize, const W: usize, const H1: usize, const W1: usize> {
    canvas: Canvas<H1, W1>,
    draw_pos: Position,
}

impl<const H: usize, const W: usize, const H1: usize, const W1: usize>
    DrawingContext<H, W, H1, W1>
{
    pub fn set_draw_pos<P: ToPosition<H, W>>(&mut self, pos: P) {
        self.draw_pos = pos.get_position();
    }

    pub fn canvas(&self) -> &Canvas<H1, W1> {
        &self.canvas
    }

    pub fn draw_pos(&self) -> Position {
        self.draw_pos
    }

    pub fn fill<C: ColorSelector>(&mut self, color: C) {
        self.canvas.fill(color)
    }

    pub fn canvas_mut(&mut self) -> &mut Canvas<H1, W1> {
        &mut self.canvas
    }
}

pub trait Drawable<const H: usize, const W: usize, const H1: usize, const W1: usize> {
    fn setup(&self, drawing_ctx: &mut DrawingContext<H, W, H1, W1>);
}
