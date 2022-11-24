use image::Rgb;

use super::color_selector::ColorSelector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    color: Rgb<u8>,
}

impl Pixel {
    pub fn new(color: Rgb<u8>) -> Self {
        Self { color }
    }

    pub fn from_rgb_code(code: [u8; 3]) -> Self {
        Self { color: Rgb(code) }
    }

    pub fn color(&self) -> Rgb<u8> {
        self.color
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            color: Rgb([255, 255, 255]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelDescriptor {
    Nothing,
    Pixel(Pixel),
}

impl PixelDescriptor {
    pub fn is_pixel(&self) -> bool {
        matches!(self, PixelDescriptor::Pixel(_))
    }

    pub fn is_nothing(&self) -> bool {
        matches!(self, PixelDescriptor::Nothing)
    }

    pub fn from_color<C: ColorSelector>(color: C) -> Self {
        color.into()
    }
}

impl Default for PixelDescriptor {
    fn default() -> Self {
        PixelDescriptor::Nothing
    }
}

impl<T> From<T> for Pixel
where
    T: ColorSelector,
{
    fn from(cs: T) -> Self {
        Self::new(cs.rgb())
    }
}

impl From<Pixel> for PixelDescriptor {
    fn from(target: Pixel) -> Self {
        Self::Pixel(target)
    }
}

impl<T> From<T> for PixelDescriptor
where
    T: ColorSelector,
{
    fn from(cs: T) -> Self {
        Pixel::new(cs.rgb()).into()
    }
}
