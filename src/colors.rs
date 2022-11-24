use crate::prelude::ColorSelector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Black;

impl ColorSelector for Black {
    fn rgb_codes(&self) -> [u8; 3] {
        [0, 0, 0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct White;

impl ColorSelector for White {
    fn rgb_codes(&self) -> [u8; 3] {
        [255, 255, 255]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Red;

impl ColorSelector for Red {
    fn rgb_codes(&self) -> [u8; 3] {
        [255, 0, 0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Green;

impl ColorSelector for Green {
    fn rgb_codes(&self) -> [u8; 3] {
        [0, 255, 0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Blue;

impl ColorSelector for Blue {
    fn rgb_codes(&self) -> [u8; 3] {
        [0, 0, 255]
    }
}
