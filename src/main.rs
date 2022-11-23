use pixelar::{colors::*, drawings::normal::Point, positions::*, prelude::*};

fn main() {
    let mut pixel_paper = PixelPaper::<3, 3>::default();

    pixel_paper.draw(Point::new(Black), LeftTopEdge);

    pixel_paper.save("arts/simple_3.png").unwrap()
}
