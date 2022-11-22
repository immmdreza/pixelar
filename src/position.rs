use std::cmp::Ordering;

pub trait ToPosition<const H: usize, const W: usize> {
    fn get_position(self) -> Position;
}

impl<const H: usize, const W: usize> ToPosition<H, W> for Position {
    fn get_position(self) -> Position {
        self
    }
}

impl<const H: usize, const W: usize> ToPosition<H, W> for [usize; 2] {
    fn get_position(self) -> Position {
        (self[0], self[1]).into()
    }
}

impl<const H: usize, const W: usize> ToPosition<H, W> for (usize, usize) {
    fn get_position(self) -> Position {
        self.into()
    }
}

impl<const H: usize, const W: usize> ToPosition<H, W> for usize {
    fn get_position(self) -> Position {
        (self, self).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Position {
    row: usize,
    column: usize,
}

impl Position {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn from_signed(row: isize, column: isize) -> Option<Self> {
        if row < 0 || column < 0 {
            None
        } else {
            Some(Self {
                row: row as usize,
                column: column as usize,
            })
        }
    }

    pub fn new_row(&self, row: usize) -> Self {
        Self {
            row,
            column: self.column,
        }
    }

    pub fn new_column(&self, column: usize) -> Self {
        Self {
            row: self.row,
            column,
        }
    }

    pub fn is_equal_to(&self, row: usize, column: usize) -> bool {
        self.row == row && self.column == column
    }

    pub fn left(&self, how_many: usize) -> Option<Position> {
        Some(Position {
            column: self.column.checked_sub(how_many)?,
            ..*self
        })
    }

    pub fn right(&self, how_many: usize) -> Option<Position> {
        Some(Position {
            column: self.column.checked_add(how_many)?,
            ..*self
        })
    }

    pub fn top(&self, how_many: usize) -> Option<Position> {
        Some(Position {
            row: self.row.checked_sub(how_many)?,
            ..*self
        })
    }

    pub fn down(&self, how_many: usize) -> Option<Position> {
        Some(Position {
            row: self.row.checked_add(how_many)?,
            ..*self
        })
    }

    pub fn left_top(&self, how_many: usize) -> Option<Position> {
        self.left(how_many)?.top(how_many)
    }

    pub fn right_top(&self, how_many: usize) -> Option<Position> {
        self.right(how_many)?.top(how_many)
    }

    pub fn left_down(&self, how_many: usize) -> Option<Position> {
        self.left(how_many)?.down(how_many)
    }
    pub fn right_down(&self, how_many: usize) -> Option<Position> {
        self.right(how_many)?.down(how_many)
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn check_pos<const H: usize, const W: usize>(&self) -> bool {
        self.row < H || self.column < W
    }
}

impl From<(usize, usize)> for Position {
    fn from(pos: (usize, usize)) -> Self {
        Position {
            row: pos.0,
            column: pos.1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativePosition {
    Same,
    Right,
    RightDown,
    Down,
    LeftDown,
    Left,
    LeftTop,
    Top,
    RightTop,
}

impl Default for RelativePosition {
    fn default() -> Self {
        RelativePosition::Top
    }
}

impl RelativePosition {
    pub fn get_exact(&self, pos: Position, scale: usize) -> Option<Position> {
        match self {
            RelativePosition::Right => pos.right(scale),
            RelativePosition::RightDown => pos.right_down(scale),
            RelativePosition::Down => pos.down(scale),
            RelativePosition::LeftDown => pos.left_down(scale),
            RelativePosition::Left => pos.left(scale),
            RelativePosition::LeftTop => pos.left_top(scale),
            RelativePosition::Top => pos.top(scale),
            RelativePosition::RightTop => pos.right_top(scale),
            RelativePosition::Same => Some(pos),
        }
    }

    pub fn get_relative_pos(pos1: Position, pos2: Position) -> RelativePosition {
        if pos2.row > pos1.row {
            match pos2.column.cmp(&pos1.column) {
                Ordering::Less => RelativePosition::LeftDown,
                Ordering::Equal => RelativePosition::Down,
                Ordering::Greater => RelativePosition::RightDown,
            }
        } else if pos2.row < pos1.row {
            match pos2.column.cmp(&pos1.column) {
                Ordering::Less => RelativePosition::LeftTop,
                Ordering::Equal => RelativePosition::Top,
                Ordering::Greater => RelativePosition::RightTop,
            }
        } else if pos2.column > pos1.column {
            RelativePosition::Right
        } else if pos2.column < pos1.column {
            RelativePosition::Left
        } else {
            RelativePosition::Same
        }
    }

    pub fn get_surrounding_relative_pos(
        pos1: Position,
        pos2: Position,
    ) -> Option<RelativePosition> {
        use RelativePosition::*;

        if pos2.row == pos1.row {
            if pos2.column + 1 == pos1.column {
                Left.into()
            } else if pos1.column + 1 == pos2.column {
                Right.into()
            } else if pos2.column == pos1.column {
                Same.into()
            } else {
                None
            }
        } else if pos2.column == pos1.column {
            if pos2.row + 1 == pos1.row {
                Top.into()
            } else if pos1.row + 1 == pos2.row {
                Down.into()
            } else if pos2.row == pos1.row {
                Same.into()
            } else {
                None
            }
        } else if pos2.column + 1 == pos1.column {
            if pos2.row + 1 == pos1.row {
                LeftTop.into()
            } else if pos1.row + 1 == pos2.row {
                LeftDown.into()
            } else {
                None
            }
        } else if pos1.column + 1 == pos2.column {
            if pos2.row + 1 == pos1.row {
                RightTop.into()
            } else if pos1.row + 1 == pos2.row {
                RightDown.into()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn next_clock_wise(&self) -> Option<RelativePosition> {
        use RelativePosition::*;

        match self {
            Same => None,
            Right => Some(RightDown),
            RightDown => Some(Down),
            Down => Some(LeftDown),
            LeftDown => Some(Left),
            Left => Some(LeftTop),
            LeftTop => Some(Top),
            Top => Some(RightTop),
            RightTop => Some(Right),
        }
    }
}

impl Iterator for RelativePosition {
    type Item = RelativePosition;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_clock_wise()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let center_point: Position = (1, 1).into();
        assert_eq!(
            RelativePosition::get_relative_pos(center_point, center_point.top(1).unwrap()),
            RelativePosition::Top
        );
        assert_eq!(
            RelativePosition::get_relative_pos(center_point, center_point.down(1).unwrap()),
            RelativePosition::Down
        );
        assert_eq!(
            RelativePosition::get_relative_pos(center_point, center_point.left(1).unwrap()),
            RelativePosition::Left
        );
        assert_eq!(
            RelativePosition::get_relative_pos(center_point, center_point.right(1).unwrap()),
            RelativePosition::Right
        );
        assert_eq!(
            RelativePosition::get_relative_pos(center_point, center_point.right_top(1).unwrap()),
            RelativePosition::RightTop
        );
        assert_eq!(
            RelativePosition::get_relative_pos(center_point, center_point.right_down(1).unwrap()),
            RelativePosition::RightDown
        );
        assert_eq!(
            RelativePosition::get_relative_pos(center_point, center_point.left_top(1).unwrap()),
            RelativePosition::LeftTop
        );
        assert_eq!(
            RelativePosition::get_relative_pos(center_point, center_point.left_down(1).unwrap()),
            RelativePosition::LeftDown
        );
    }

    #[test]
    fn test_name_1() {
        let center_point: Position = (1, 1).into();
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.top(1).unwrap()
            ),
            Some(RelativePosition::Top)
        );
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.down(1).unwrap()
            ),
            Some(RelativePosition::Down)
        );
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.left(1).unwrap()
            ),
            Some(RelativePosition::Left)
        );
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.right(1).unwrap()
            ),
            Some(RelativePosition::Right)
        );
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.right_top(1).unwrap()
            ),
            Some(RelativePosition::RightTop)
        );
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.right_down(1).unwrap()
            ),
            Some(RelativePosition::RightDown)
        );
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.left_top(1).unwrap()
            ),
            Some(RelativePosition::LeftTop)
        );
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.left_down(1).unwrap()
            ),
            Some(RelativePosition::LeftDown)
        );
    }

    #[test]
    fn test_name_2() {
        let center_point: Position = (2, 2).into();
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.top(2).unwrap()
            ),
            None
        );
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.down(2).unwrap()
            ),
            None
        );
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.left(2).unwrap()
            ),
            None
        );
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.right(2).unwrap()
            ),
            None
        );
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.right_top(2).unwrap()
            ),
            None
        );
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.right_down(2).unwrap()
            ),
            None
        );
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.left_top(2).unwrap()
            ),
            None
        );
        assert_eq!(
            RelativePosition::get_surrounding_relative_pos(
                center_point,
                center_point.left_down(2).unwrap()
            ),
            None
        );
    }
}
