use crate::prelude::{ColorSelector, PixelDescriptor, PixelsTable, Position, ToPosition};

pub struct StraightLine<const H: usize, const W: usize> {
    table: [[PixelDescriptor; H]; W],
    start: Position,
    end: Position,
    color: PixelDescriptor,
}

impl<const H: usize, const W: usize> StraightLine<H, W> {
    pub fn new<P1: ToPosition<H, W>, P2: ToPosition<H, W>>(
        color: PixelDescriptor,
        start: P1,
        end: P2,
    ) -> Self {
        let mut t = Self {
            start: start.get_position(),
            end: end.get_position(),
            table: Self::default_table(),
            color,
        };

        for pos in t.get_dda_line() {
            let _ = t.change_pixel_at(pos, t.color);
        }

        t
    }

    pub fn from_color<C: ColorSelector, P1: ToPosition<H, W>, P2: ToPosition<H, W>>(
        color: C,
        start: P1,
        end: P2,
    ) -> Self {
        Self::new(color.rgb().into(), start, end)
    }

    pub fn get_dda_line(&self) -> DDALine {
        DDALine::new(self.start, self.end)
    }

    pub fn continue_with<P: ToPosition<H, W> + Clone>(
        &mut self,
        other_end: P,
    ) -> &mut StraightLine<H, W> {
        self.draw_from_table_exact(&StraightLine::new(self.color, self.end, other_end.clone()));
        self.end = other_end.get_position();

        self
    }
}

impl<const H: usize, const W: usize> PixelsTable<H, W> for StraightLine<H, W> {
    fn table(&self) -> &[[PixelDescriptor; H]; W] {
        &self.table
    }

    fn table_mut(&mut self) -> &mut [[PixelDescriptor; H]; W] {
        &mut self.table
    }
}

pub struct DDALine {
    x: f64,
    y: f64,
    x_increment: f64,
    y_increment: f64,
    cur_step: usize,
    max_step: usize,
}

impl DDALine {
    pub fn new(start: Position, end: Position) -> Self {
        let x1 = start.row() as isize;
        let y1 = start.column() as isize;
        let x2 = end.row() as isize;
        let y2 = end.column() as isize;

        let dx = x2 - x1;
        let dy = y2 - y1;

        let dx_abs = dx.abs();
        let dy_abs = dy.abs();

        let steps = (if dx_abs > dy_abs { dx_abs } else { dy_abs }) as usize;

        let x_increment = dx as f64 / steps as f64;
        let y_increment = dy as f64 / steps as f64;

        let x = x1 as f64;
        let y = y1 as f64;

        Self {
            x,
            y,
            x_increment,
            y_increment,
            cur_step: 0,
            max_step: steps,
        }
    }
}

impl Iterator for DDALine {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_step > self.max_step {
            None
        } else {
            let res = Some(Position::from_signed(
                self.x.round() as isize,
                self.y.round() as isize,
            )?);

            self.x += self.x_increment;
            self.y += self.y_increment;
            self.cur_step += 1;
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        for pos in DDALine::new((0, 4).into(), (0, 0).into()) {
            println!("{:?}", pos);
        }
    }
}
