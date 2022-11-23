use crate::{position::Position, ToPosition};

use super::{
    drawable::Drawable, ColorSelector, DrawingContext, MoreMethodsForPixelsTable, PixelDescriptor,
};

#[derive(Debug)]
pub struct PixelNotFound(Position);

pub trait PixelsTable<const H: usize, const W: usize> {
    fn get_table(&self) -> &[[PixelDescriptor; H]; W];

    fn get_mut_table(&mut self) -> &mut [[PixelDescriptor; H]; W];

    fn get_pixel_at<P: ToPosition<H, W>>(&self, pos: P) -> Result<&PixelDescriptor, PixelNotFound> {
        let pos = pos.get_position();
        self.get_table()
            .get(pos.row())
            .ok_or(PixelNotFound(pos))?
            .get(pos.column())
            .ok_or(PixelNotFound(pos))
    }

    fn get_mut_pixel_at<P: ToPosition<H, W>>(
        &mut self,
        pos: P,
    ) -> Result<&mut PixelDescriptor, PixelNotFound> {
        let pos = pos.get_position();
        self.get_mut_table()
            .get_mut(pos.row())
            .ok_or(PixelNotFound(pos))?
            .get_mut(pos.column())
            .ok_or(PixelNotFound(pos))
    }

    fn change_pixel_at<P: ToPosition<H, W>>(
        &mut self,
        pos: P,
        pd: PixelDescriptor,
    ) -> Result<(), PixelNotFound> {
        let pixel = self.get_mut_pixel_at(pos)?;
        *pixel = pd;
        Ok(())
    }

    fn draw_from<const H1: usize, const W1: usize, P: PixelsTable<H1, W1>, P1: ToPosition<H, W>>(
        &mut self,
        other: &P,
        pos: P1,
    ) {
        let my_table = self.get_mut_table();
        let other_table = other.get_table();

        let pos = pos.get_position();
        let row_offset = pos.row();
        let column_offset = pos.column();

        for (i, row) in other_table.iter().enumerate() {
            for (j, pixel) in row.iter().enumerate() {
                if let PixelDescriptor::Pixel(_) = pixel {
                    if let Some(target_row) = my_table.get_mut(i + row_offset) {
                        if let Some(target_pixel) = target_row.get_mut(j + column_offset) {
                            *target_pixel = pixel.to_owned()
                        }
                    }
                }
            }
        }
    }

    fn draw_exactly_from<P: PixelsTable<H, W>>(&mut self, pixel_table: &P) {
        self.draw_from(pixel_table, (0, 0))
    }

    fn draw<const H1: usize, const W1: usize, D: Drawable<H1, W1>, P: ToPosition<H, W>>(
        &mut self,
        drawable: D,
        pos: P,
    ) where
        Self: Sized,
    {
        let mut drawing_ctx = DrawingContext::default();
        drawable.setup(&mut drawing_ctx);

        self.draw_from(drawing_ctx.canvas(), pos);
    }

    fn draw_on<const H1: usize, const W1: usize, P: PixelsTable<H1, W1>, P1: ToPosition<H1, W1>>(
        &self,
        other: &mut P,
        pos: P1,
    ) where
        Self: Sized,
    {
        other.draw_from(self, pos)
    }

    fn draw_exactly_on<P: PixelsTable<H, W>>(&self, other: &mut P)
    where
        Self: Sized,
    {
        other.draw_exactly_from(self)
    }

    fn get_default_table() -> [[PixelDescriptor; H]; W] {
        [[PixelDescriptor::default(); H]; W]
    }

    fn get_filled_table<C: ColorSelector>(color: C) -> [[PixelDescriptor; H]; W] {
        [[PixelDescriptor::Pixel(color.get_rgb().into()); H]; W]
    }
}

impl<const H: usize, const W: usize, T> MoreMethodsForPixelsTable<H, W> for T where
    T: PixelsTable<H, W>
{
}
