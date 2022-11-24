use crate::position::{Position, RelativePosition};

use super::PixelsTable;

pub struct SurroundingPixels<'t, const H: usize, const W: usize, P: PixelsTable<H, W>> {
    table: &'t P,
    requested_point: Position,
    iterated: u8,
    cur_rel_position: RelativePosition,
}

impl<'t, const H: usize, const W: usize, P: PixelsTable<H, W>> SurroundingPixels<'t, H, W, P> {
    pub fn new(table: &'t P, requested_point: Position) -> Self {
        Self {
            table,
            requested_point,
            iterated: 0,
            cur_rel_position: RelativePosition::LeftTop,
        }
    }
}

impl<'t, const H: usize, const W: usize, P: PixelsTable<H, W>> Iterator
    for SurroundingPixels<'t, H, W, P>
{
    type Item = (RelativePosition, Position);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterated += 1;
        if self.iterated > 8 {
            None
        } else {
            self.cur_rel_position = self.cur_rel_position.next()?;
            let abs_pos = self.cur_rel_position.get_exact(self.requested_point, 1);
            if let Some(abs_pos) = abs_pos {
                if let Ok(pix) = self.table.pixel_at(abs_pos) {
                    if pix.is_pixel() {
                        Some((self.cur_rel_position, abs_pos))
                    } else {
                        self.next()
                    }
                } else {
                    self.next()
                }
            } else {
                self.next()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pixels_table::canvas::Canvas;

    use super::*;

    #[test]
    fn test_name() {
        let mut table = Canvas::<5, 5>::default();
        let it = table.table_mut();

        it[1][1] = [1, 1, 1].into();
        it[3][3] = [1, 1, 1].into();

        let sur: Vec<(RelativePosition, Position)> =
            SurroundingPixels::new(&table, (2, 2).into()).collect();

        assert_eq!(
            vec![
                (RelativePosition::RightDown, (3, 3).into()),
                (RelativePosition::LeftTop, (1, 1).into())
            ],
            sur
        );
    }
}
