use crate::{
    drawings::special::{FreeDrawing, StraightLine},
    prelude::{Canvas, Position, RelativePosition, ToPosition},
};

use super::{
    surrounding_pixels::SurroundingPixels, ColorSelector, PixelDescriptor, PixelsTable,
    RectSelection, Selection,
};

pub trait MoreMethodsForPixelsTable<const H: usize, const W: usize>: PixelsTable<H, W> {
    fn change_pixel_color<P: ToPosition<H, W>, C1: ColorSelector>(&mut self, pos: P, color: C1) {
        if let Ok(pix) = self.pixel_at_mut(pos.get_position()) {
            *pix = color.rgb().into()
        }
    }

    fn surrounding_pixels(&self, pos: &Position) -> SurroundingPixels<H, W, Self>
    where
        Self: Sized,
    {
        SurroundingPixels::new(self, *pos)
    }

    fn boundary_fill<P: ToPosition<H, W>, C1: ColorSelector + Clone>(
        &mut self,
        pos: P,
        fill_color: PixelDescriptor,
        boundary_color: C1,
        dimensional_penetration: bool,
    ) {
        let pos = pos.get_position();
        let bc = boundary_color.rgb();

        let pix = match self.pixel_at_mut(pos) {
            Ok(pix) => pix,
            Err(_) => return,
        };

        if *pix != fill_color {
            match pix {
                PixelDescriptor::Nothing => {
                    *pix = fill_color;

                    for dir in [
                        RelativePosition::Top,
                        RelativePosition::Right,
                        RelativePosition::Down,
                        RelativePosition::Left,
                    ] {
                        if let Some(nex_pos) = dir.get_exact(pos, 1) {
                            self.boundary_fill(
                                nex_pos,
                                fill_color,
                                boundary_color.clone(),
                                dimensional_penetration,
                            )
                        }
                    }

                    if dimensional_penetration {
                        for dir in [
                            RelativePosition::RightTop,
                            RelativePosition::RightDown,
                            RelativePosition::LeftDown,
                            RelativePosition::LeftTop,
                        ] {
                            if let Some(nex_pos) = dir.get_exact(pos, 1) {
                                self.boundary_fill(
                                    nex_pos,
                                    fill_color,
                                    boundary_color.clone(),
                                    dimensional_penetration,
                                )
                            }
                        }
                    }
                }
                PixelDescriptor::Pixel(bc1) => {
                    if bc1.color() != bc {
                        *pix = fill_color;

                        for dir in [
                            RelativePosition::Top,
                            RelativePosition::Right,
                            RelativePosition::Down,
                            RelativePosition::Left,
                        ] {
                            if let Some(nex_pos) = dir.get_exact(pos, 1) {
                                self.boundary_fill(
                                    nex_pos,
                                    fill_color,
                                    boundary_color.clone(),
                                    dimensional_penetration,
                                )
                            }
                        }

                        if dimensional_penetration {
                            for dir in [
                                RelativePosition::RightTop,
                                RelativePosition::RightDown,
                                RelativePosition::LeftDown,
                                RelativePosition::LeftTop,
                            ] {
                                if let Some(nex_pos) = dir.get_exact(pos, 1) {
                                    self.boundary_fill(
                                        nex_pos,
                                        fill_color,
                                        boundary_color.clone(),
                                        dimensional_penetration,
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn boundary_fill_color<P: ToPosition<H, W>, C1: ColorSelector, C2: ColorSelector + Clone>(
        &mut self,
        pos: P,
        fill_color: C1,
        boundary_color: C2,
        dimensional_penetration: bool,
    ) {
        self.boundary_fill(
            pos,
            fill_color.rgb().into(),
            boundary_color,
            dimensional_penetration,
        )
    }

    fn draw_straight_line<C: ColorSelector, P1: ToPosition<H, W>, P2: ToPosition<H, W>>(
        &mut self,
        color: C,
        start: P1,
        end: P2,
    ) {
        let line = StraightLine::from_color(color, start, end);
        self.draw_from_table_exact(&line)
    }

    fn draw_many_straight_lines<C: ColorSelector, P1: ToPosition<H, W>, P2: ToPosition<H, W>>(
        &mut self,
        color: C,
        start: P1,
        end: P2,
        builder: impl FnOnce(&mut StraightLine<H, W>),
    ) {
        let mut line = StraightLine::from_color(color, start, end);
        builder(&mut line);

        self.draw_from_table_exact(&line)
    }

    fn draw_free_drawing<C: ColorSelector>(
        &mut self,
        color: C,
        builder: impl FnOnce(&mut FreeDrawing<H, W>),
    ) {
        let mut free_drawing = FreeDrawing::new(color);
        builder(&mut free_drawing);

        self.draw_from_table_exact(&free_drawing)
    }

    /// Get an rectangle like selection with specified top left and bottom right edges.
    ///
    /// ## Example
    /// ```
    /// let mut selection = pixel_paper.rect_selection(LeftTopEdge, (3, 3));
    /// selection.apply_color(Black);
    /// ```
    fn rect_selection<P1: ToPosition<H, W>, P2: ToPosition<H, W>>(
        &mut self,
        top_left: P1,
        bottom_right: P2,
    ) -> RectSelection<H, W, Self>
    where
        Self: Sized,
    {
        RectSelection::new(self, top_left, bottom_right)
    }

    /// Get a copy of a fixed size ( `H1`*`W1` ) section of a [`PixelsTable`].
    ///
    /// # Example
    /// ```
    /// let copy = pixel_paper.section_copy::<3, 3>(LeftTopEdge);
    /// ```
    fn section_copy<const H1: usize, const W1: usize>(
        &mut self,
        start: impl ToPosition<H, W>,
    ) -> Canvas<H1, W1>
    where
        Self: Sized,
    {
        let mut canvas = Canvas::<H1, W1>::default();
        let start = start.get_position();
        let selection = self.rect_selection(start, (start.row() + H1, start.column() + W1));
        selection.copy_to(&mut canvas, (0, 0));

        canvas
    }

    /// **Cut, Modify, Replace**
    ///
    /// Modify a fixed size ( `H1`*`W1` ) section of this [`PixelsTable`]
    /// separately and in place.
    ///
    /// ## Example
    /// Modify a 3*3 cut starting from LeftTopEdge as a separate [`Canvas`].
    /// ```
    /// pixel_paper.modify_section::<3, 3>(LeftTopEdge, |canvas| {
    ///     canvas.change_pixel_color(RightBottomEdge, Red);
    /// });
    /// ```
    fn modify_section<const H1: usize, const W1: usize>(
        &mut self,
        start: impl ToPosition<H, W> + Clone,
        modifier: impl FnOnce(&mut Canvas<H1, W1>),
    ) where
        Self: Sized,
    {
        let mut section = self.section_copy::<H1, W1>(start.clone());
        modifier(&mut section);

        self.draw_from_table(&section, start);
    }
}
