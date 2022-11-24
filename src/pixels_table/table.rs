use crate::{position::Position, ToPosition};

use super::{
    drawable::Drawable, ColorSelector, DrawingContext, MoreMethodsForPixelsTable, PixelDescriptor,
};

#[derive(Debug)]
pub struct PixelNotFound(Position);

pub trait PixelsTable<const H: usize, const W: usize> {
    fn table(&self) -> &[[PixelDescriptor; H]; W];

    fn table_mut(&mut self) -> &mut [[PixelDescriptor; H]; W];

    fn pixel_at<P: ToPosition<H, W>>(&self, pos: P) -> Result<&PixelDescriptor, PixelNotFound> {
        let pos = pos.get_position();
        self.table()
            .get(pos.row())
            .ok_or(PixelNotFound(pos))?
            .get(pos.column())
            .ok_or(PixelNotFound(pos))
    }

    fn pixel_at_mut<P: ToPosition<H, W>>(
        &mut self,
        pos: P,
    ) -> Result<&mut PixelDescriptor, PixelNotFound> {
        let pos = pos.get_position();
        self.table_mut()
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
        let pixel = self.pixel_at_mut(pos)?;
        *pixel = pd;
        Ok(())
    }

    fn draw_from_table<
        const H1: usize,
        const W1: usize,
        P: PixelsTable<H1, W1>,
        P1: ToPosition<H, W>,
    >(
        &mut self,
        other: &P,
        pos: P1,
    ) {
        let my_table = self.table_mut();
        let other_table = other.table();

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

    fn draw_from_table_exact<P: PixelsTable<H, W>>(&mut self, pixel_table: &P) {
        self.draw_from_table(pixel_table, (0, 0))
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

        self.draw_from_table(drawing_ctx.canvas(), pos);
    }

    fn draw_exact<D: Drawable<H, W>, P: ToPosition<H, W>>(&mut self, drawable: D, pos: P)
    where
        Self: Sized,
    {
        let mut drawing_ctx = DrawingContext::default();
        drawable.setup(&mut drawing_ctx);

        self.draw_from_table(drawing_ctx.canvas(), pos);
    }

    fn draw_on_table<
        const H1: usize,
        const W1: usize,
        P: PixelsTable<H1, W1>,
        P1: ToPosition<H1, W1>,
    >(
        &self,
        other: &mut P,
        pos: P1,
    ) where
        Self: Sized,
    {
        other.draw_from_table(self, pos)
    }

    fn draw_on_exact<P: PixelsTable<H, W>>(&self, other: &mut P)
    where
        Self: Sized,
    {
        other.draw_from_table_exact(self)
    }

    fn default_table() -> [[PixelDescriptor; H]; W] {
        [[PixelDescriptor::default(); H]; W]
    }

    fn filled_table<C: ColorSelector>(color: C) -> [[PixelDescriptor; H]; W] {
        [[PixelDescriptor::Pixel(color.rgb().into()); H]; W]
    }
}

impl<const H: usize, const W: usize, T> MoreMethodsForPixelsTable<H, W> for T where
    T: PixelsTable<H, W>
{
}
