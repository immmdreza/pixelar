use crate::prelude::ColorSelector;

pub struct Black;

impl ColorSelector for Black {
    fn get_rgb_codes(&self) -> [u8; 3] {
        [0, 0, 0]
    }
}

pub struct White;

impl ColorSelector for White {
    fn get_rgb_codes(&self) -> [u8; 3] {
        [255, 255, 255]
    }
}

pub struct Red;

impl ColorSelector for Red {
    fn get_rgb_codes(&self) -> [u8; 3] {
        [255, 0, 0]
    }
}

pub struct Green;

impl ColorSelector for Green {
    fn get_rgb_codes(&self) -> [u8; 3] {
        [0, 255, 0]
    }
}

pub struct Blue;

impl ColorSelector for Blue {
    fn get_rgb_codes(&self) -> [u8; 3] {
        [0, 0, 255]
    }
}
