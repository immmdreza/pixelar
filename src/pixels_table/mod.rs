mod color_selector;
mod more_methods_for_table;
mod pixel_descriptor;
pub mod simple_table;
pub mod surrounding_pixels;
mod table;

pub use color_selector::{ColorSelector, RgbColor};
pub use more_methods_for_table::MoreMethodsForPixelsTable;
pub use pixel_descriptor::PixelDescriptor;
pub use table::{PixelNotFound, PixelsTable};
