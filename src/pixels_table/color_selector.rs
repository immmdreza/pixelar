use image::Rgb;

pub trait ColorSelector {
    fn get_rgb_codes(&self) -> [u8; 3];

    fn get_rgb(&self) -> Rgb<u8> {
        self.get_rgb_codes().into()
    }
}

impl ColorSelector for u8 {
    fn get_rgb_codes(&self) -> [u8; 3] {
        [*self, *self, *self]
    }
}

impl ColorSelector for [u8; 3] {
    fn get_rgb_codes(&self) -> [u8; 3] {
        self.to_owned()
    }
}

impl ColorSelector for (u8, u8, u8) {
    fn get_rgb_codes(&self) -> [u8; 3] {
        [self.0, self.1, self.2]
    }
}

impl ColorSelector for Rgb<u8> {
    fn get_rgb_codes(&self) -> [u8; 3] {
        self.0.to_owned()
    }
}

pub struct RgbColor(pub u8, pub u8, pub u8);

impl ColorSelector for RgbColor {
    fn get_rgb_codes(&self) -> [u8; 3] {
        [self.0, self.1, self.2]
    }
}
