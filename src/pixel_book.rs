use std::fs::File;

use image::{
    codecs::gif::{GifEncoder, Repeat},
    Frame, ImageResult,
};

use crate::PixelPaper;

pub struct PixelBook<const H: usize, const W: usize> {
    repeat: Repeat,
    papers: Vec<PixelPaper<H, W>>,
}

impl<const H: usize, const W: usize> PixelBook<H, W> {
    pub fn new(repeat: Repeat) -> Self {
        Self {
            repeat,
            papers: vec![],
        }
    }

    pub fn save(&self, path: &str) -> ImageResult<()> {
        let mut encoder = GifEncoder::new(File::create(path).unwrap());
        encoder.set_repeat(self.repeat)?;
        let frames = self.papers.iter().map(|f| Frame::new(f.get_image_buffer()));
        encoder.encode_frames(frames)?;
        Ok(())
    }

    pub fn add_paper(&mut self, paper: &PixelPaper<H, W>) {
        self.papers.push(paper.clone());
    }

    pub fn add_papers<P>(&mut self, papers: P)
    where
        P: IntoIterator<Item = PixelPaper<H, W>>,
    {
        for paper in papers {
            self.add_paper(&paper);
        }
    }
}
