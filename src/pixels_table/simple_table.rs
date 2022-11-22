use super::{PixelDescriptor, PixelsTable};

#[derive(Debug)]
pub struct SimpleTable<const H: usize, const W: usize>([[PixelDescriptor; H]; W]);

impl<const H: usize, const W: usize> Default for SimpleTable<H, W> {
    fn default() -> Self {
        Self(Self::get_default_table())
    }
}

impl<const H: usize, const W: usize> PixelsTable<H, W> for SimpleTable<H, W> {
    fn get_table(&self) -> &[[PixelDescriptor; H]; W] {
        &self.0
    }

    fn get_mut_table(&mut self) -> &mut [[PixelDescriptor; H]; W] {
        &mut self.0
    }
}
