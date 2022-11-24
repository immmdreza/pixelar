use pixelar::{
    colors::*, drawings::normal::Point, pixels_table::Selection, positions::*, prelude::*,
};

fn main() {
    let mut pixel_paper = PixelPaper::<3, 3>::default();

    pixel_paper.rect_selection(0, 1).apply_color(Red);

    pixel_paper.save("arts/simple_3.png").unwrap()
}
