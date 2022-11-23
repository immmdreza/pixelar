use pixelar::{colors::*, drawings::drawable::Point, positions::*, prelude::*};

fn main() {
    let mut pixel_paper = PixelPaper::<6, 6>::default();

    pixel_paper.draw_from_drawable(Point::new(Black, LeftBottomEdge));

    pixel_paper.save("arts/simple_3.png").unwrap()
}
