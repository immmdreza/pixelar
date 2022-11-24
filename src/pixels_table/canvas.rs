use super::{ColorSelector, PixelDescriptor, PixelsTable};

#[derive(Debug)]
pub struct Canvas<const H: usize, const W: usize> {
    table: [[PixelDescriptor; H]; W],
}

impl<const H: usize, const W: usize> Canvas<H, W> {
    pub fn new<C: ColorSelector>(background: C) -> Self {
        Self {
            table: Self::filled_table(background),
        }
    }

    pub fn fill<C: ColorSelector>(&mut self, color: C) {
        self.table = Self::filled_table(color);
    }
}

impl<const H: usize, const W: usize> Default for Canvas<H, W> {
    fn default() -> Self {
        Self {
            table: Self::default_table(),
        }
    }
}

impl<const H: usize, const W: usize> PixelsTable<H, W> for Canvas<H, W> {
    fn table(&self) -> &[[PixelDescriptor; H]; W] {
        &self.table
    }

    fn table_mut(&mut self) -> &mut [[PixelDescriptor; H]; W] {
        &mut self.table
    }
}
