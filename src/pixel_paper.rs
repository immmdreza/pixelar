use std::path::Path;

use image::{ImageBuffer, Rgb};
use imageproc::{drawing::draw_filled_rect_mut, rect::Rect};

use crate::prelude::{ColorSelector, PixelDescriptor, PixelsTable};

pub struct PixelPaper<const H: usize, const W: usize> {
    pixels_table: [[PixelDescriptor; H]; W],
    block_width: usize,
    separator_width: usize,
    background: Rgb<u8>,
    separator_color: Rgb<u8>,
}

impl<const H: usize, const W: usize> Default for PixelPaper<H, W> {
    fn default() -> Self {
        Self::new(10, 1, [255, 255, 255], [0, 0, 0])
    }
}

impl<const H: usize, const W: usize> PixelPaper<H, W> {
    pub fn new<C1, C2>(
        block_width: usize,
        separator_width: usize,
        background: C1,
        separator_color: C2,
    ) -> Self
    where
        C1: ColorSelector,
        C2: ColorSelector,
    {
        Self {
            pixels_table: Self::get_default_table(),
            block_width,
            background: background.get_rgb(),
            separator_width,
            separator_color: separator_color.get_rgb(),
        }
    }

    fn get_pixel_paper_image(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let separator_pixel_length = self.separator_width;

        let blocks_pixel_in_height = H * self.block_width;
        let separators_count_in_height = H + 1;
        let separators_pixel_in_height = separators_count_in_height * separator_pixel_length;
        let height = blocks_pixel_in_height + separators_pixel_in_height;

        let blocks_pixel_in_width = W * self.block_width;
        let separators_count_in_width = W + 1;
        let separators_pixel_in_width = separators_count_in_width * separator_pixel_length;
        let width = blocks_pixel_in_width + separators_pixel_in_width;

        let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::new(width as u32, height as u32);

        draw_filled_rect_mut(
            &mut image,
            Rect::at(0, 0).of_size(width as u32, height as u32),
            self.background.get_rgb(),
        );

        for i in 0..separators_count_in_width as i32 {
            draw_filled_rect_mut(
                &mut image,
                Rect::at(i * ((separator_pixel_length + self.block_width) as i32), 0)
                    .of_size(separator_pixel_length as u32, height as u32),
                self.separator_color.get_rgb(),
            )
        }

        for i in 0..separators_count_in_height as i32 {
            draw_filled_rect_mut(
                &mut image,
                Rect::at(0, i * ((separator_pixel_length + self.block_width) as i32))
                    .of_size(width as u32, separator_pixel_length as u32),
                self.separator_color.get_rgb(),
            )
        }

        image
    }

    fn draw_to_image_pixels(
        &self,
        image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
        color: Rgb<u8>,
        row: usize,
        column: usize,
    ) {
        // Find out start pixel ...
        // 0, 0 -> 1sp + 0bp, 1sp + 0bp
        // 1, 1 -> 2sp + 1bp, 2sp + 1bp
        // 2, 2 -> 3sp + 2bp, 3sp + 2bp
        // i, j -> (i + 1)sp + ibp, (j + 1)sp + jbp
        let start_x_pixel = ((column + 1) * self.separator_width) + (column * self.block_width);
        let start_y_pixel = ((row + 1) * self.separator_width) + (row * self.block_width);

        for i in 0..self.block_width {
            for j in 0..self.block_width {
                image.put_pixel(
                    (i + start_y_pixel) as u32,
                    (j + start_x_pixel) as u32,
                    color,
                )
            }
        }
    }

    fn draw_pixels_table(&self, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let other_table = *self.get_table();

        for (i, row) in other_table.iter().enumerate() {
            for (j, pixel) in row.iter().enumerate() {
                if let PixelDescriptor::Pixel(pixel) = pixel {
                    self.draw_to_image_pixels(image, pixel.color(), j, i)
                }
            }
        }
    }

    pub fn save(&self, path: &str) -> Result<(), image::ImageError> {
        let mut image = self.get_pixel_paper_image();
        self.draw_pixels_table(&mut image);
        image.save(Path::new(path))
    }
}

impl<const H: usize, const W: usize> PixelsTable<H, W> for PixelPaper<H, W> {
    fn get_table(&self) -> &[[PixelDescriptor; H]; W] {
        &self.pixels_table
    }

    fn get_mut_table(&mut self) -> &mut [[PixelDescriptor; H]; W] {
        &mut self.pixels_table
    }
}
