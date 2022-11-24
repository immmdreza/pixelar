use crate::{
    drawings::special::{FreeDrawing, StraightLine},
    prelude::{Position, RelativePosition, ToPosition},
};

use super::{
    surrounding_pixels::SurroundingPixels, ColorSelector, PixelDescriptor, PixelsTable,
    RectSelection,
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

    fn rect_selection<P1: ToPosition<H, W>, P2: ToPosition<H, W>>(
        &mut self,
        start: P1,
        end: P2,
    ) -> RectSelection<H, W, Self>
    where
        Self: Sized,
    {
        RectSelection::new(self, start, end)
    }
}
