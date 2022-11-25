use crate::{Position, ToPosition};

use super::{ColorSelector, PixelDescriptor, PixelsTable};

pub trait Selection<'t> {
    fn selected_area(&'t self) -> Vec<&'t [PixelDescriptor]>;

    fn selected_area_mut(&'t mut self) -> Vec<&'t mut [PixelDescriptor]>;

    fn apply_pixel(&'t mut self, pixel: PixelDescriptor) {
        for row in self.selected_area_mut() {
            for pix in row {
                *pix = pixel
            }
        }
    }

    fn apply_color<C: ColorSelector + Clone>(&'t mut self, color: C) {
        for row in self.selected_area_mut() {
            for pix in row {
                *pix = color.clone().into();
            }
        }
    }

    fn apply_nothing<C: ColorSelector + Clone>(&'t mut self) {
        for row in self.selected_area_mut() {
            for pix in row {
                *pix = PixelDescriptor::Nothing;
            }
        }
    }

    fn copy_to<const H: usize, const W: usize, P: PixelsTable<H, W>, P1: ToPosition<H, W>>(
        &'t self,
        table: &mut P,
        start: P1,
    ) {
        let start = start.get_position();
        for (i, row) in self.selected_area().iter().enumerate() {
            for (j, pix) in row.iter().enumerate() {
                let _ = table.change_pixel_at((i + start.row(), j + start.column()), *pix);
            }
        }
    }
}

pub struct RectSelection<'t, const H: usize, const W: usize, P: PixelsTable<H, W>> {
    table: &'t mut P,
    start: Position,
    end: Position,
}

impl<'t, const H: usize, const W: usize, P: PixelsTable<H, W>> RectSelection<'t, H, W, P> {
    pub fn new<P1: ToPosition<H, W>, P2: ToPosition<H, W>>(
        table: &'t mut P,
        start: P1,
        end: P2,
    ) -> Self {
        Self {
            table,
            start: start.get_position(),
            end: end.get_position(),
        }
    }
}

impl<'t, const H: usize, const W: usize, P: PixelsTable<H, W>> Selection<'t>
    for RectSelection<'t, H, W, P>
{
    fn selected_area(&'t self) -> Vec<&'t [PixelDescriptor]> {
        let table = self.table.table();
        table[self.start.row()..=self.end.row()]
            .iter()
            .map(|t| &t[self.start.column()..=self.end.column()])
            .collect()
    }

    fn selected_area_mut(&'t mut self) -> Vec<&'t mut [PixelDescriptor]> {
        let start_row = self.start.row();
        let start_col = self.start.column();
        let end_row = self.end.row();
        let end_col = self.end.column();

        let table = self.table.table_mut();
        table[start_row..=end_row]
            .iter_mut()
            .map(|t| &mut t[start_col..=end_col])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{colors::Black, prelude::Canvas};

    use super::*;

    #[test]
    fn test_rect_selection() {
        let mut c = Canvas::<5, 5>::default();

        let mut rs = RectSelection::new(&mut c, 0, 1);

        rs.apply_color(Black);
    }
}
