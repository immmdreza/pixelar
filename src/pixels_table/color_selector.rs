use image::Rgb;

pub trait ColorSelector {
    fn rgb_codes(&self) -> [u8; 3];

    fn rgb(&self) -> Rgb<u8> {
        self.rgb_codes().into()
    }

    fn rgb_color(&self) -> RgbColor {
        let codes = self.rgb_codes();
        RgbColor(codes[0], codes[1], codes[2])
    }
}

impl ColorSelector for u8 {
    fn rgb_codes(&self) -> [u8; 3] {
        [*self, *self, *self]
    }
}

impl ColorSelector for [u8; 3] {
    fn rgb_codes(&self) -> [u8; 3] {
        self.to_owned()
    }
}

impl ColorSelector for (u8, u8, u8) {
    fn rgb_codes(&self) -> [u8; 3] {
        [self.0, self.1, self.2]
    }
}

impl ColorSelector for Rgb<u8> {
    fn rgb_codes(&self) -> [u8; 3] {
        self.0.to_owned()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColor(pub u8, pub u8, pub u8);

impl ColorSelector for RgbColor {
    fn rgb_codes(&self) -> [u8; 3] {
        [self.0, self.1, self.2]
    }
}
