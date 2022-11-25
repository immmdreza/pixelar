use pixelar::{
    colors::*, drawings::normal::Point, pixels_table::Selection, positions::*, prelude::*,
};

fn main() {
    let mut pixel_paper = PixelPaper::<5, 5>::default();

    let mut selection = pixel_paper.rect_selection(LeftTopEdge, (3, 3));
    selection.apply_color(Black);

    pixel_paper.save("arts/simple_3.png").unwrap()
}
