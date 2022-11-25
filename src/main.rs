use pixelar::{colors::*, prelude::*};

fn main() {
    let mut paper = PixelPaper::<5, 5>::default();
    let mut animation = PixelBook::new(Repeat::Infinite);

    animation.add_paper(&paper);

    for i in 0..5 {
        paper.change_pixel_color((i, i), Red);
        paper.change_pixel_color((i, 4 - i), Blue);

        animation.add_paper(&paper);
    }

    animation.save("arts/animated_1.gif").unwrap();
}
