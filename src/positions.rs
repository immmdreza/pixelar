use crate::ToPosition;

pub struct LeftTopEdge;

impl<const H: usize, const W: usize> ToPosition<H, W> for LeftTopEdge {
    fn get_position(self) -> crate::Position {
        (0, 0).into()
    }
}

pub struct LeftBottomEdge;

impl<const H: usize, const W: usize> ToPosition<H, W> for LeftBottomEdge {
    fn get_position(self) -> crate::Position {
        (H - 1, 0).into()
    }
}

pub struct RightTopEdge;

impl<const H: usize, const W: usize> ToPosition<H, W> for RightTopEdge {
    fn get_position(self) -> crate::Position {
        (0, W - 1).into()
    }
}

pub struct RightBottomEdge;

impl<const H: usize, const W: usize> ToPosition<H, W> for RightBottomEdge {
    fn get_position(self) -> crate::Position {
        (H - 1, W - 1).into()
    }
}

pub struct Mid;

impl<const H: usize, const W: usize> ToPosition<H, W> for Mid {
    fn get_position(self) -> crate::Position {
        (H / 2, W / 2).into()
    }
}

pub struct TopMid;

impl<const H: usize, const W: usize> ToPosition<H, W> for TopMid {
    fn get_position(self) -> crate::Position {
        (0, W / 2).into()
    }
}

pub struct BottomMid;

impl<const H: usize, const W: usize> ToPosition<H, W> for BottomMid {
    fn get_position(self) -> crate::Position {
        (H, W / 2).into()
    }
}

pub struct LeftMid;

impl<const H: usize, const W: usize> ToPosition<H, W> for LeftMid {
    fn get_position(self) -> crate::Position {
        (H / 2, 0).into()
    }
}

pub struct RightMid;

impl<const H: usize, const W: usize> ToPosition<H, W> for RightMid {
    fn get_position(self) -> crate::Position {
        (H / 2, W).into()
    }
}
