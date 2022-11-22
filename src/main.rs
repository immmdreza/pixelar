use pixelar::{colors::*, prelude::*};

fn main() {
    let mut pixel_paper = PixelPaper::<6, 6>::default();

    let rainbow = [
        (250, 130, 0),
        (240, 250, 0),
        (60, 250, 0),
        (0, 250, 220),
        (0, 10, 250),
        (210, 0, 250),
    ];

    for i in 0..6 {
        for j in 0..6 {
            let choose = match j.cmp(&i) {
                std::cmp::Ordering::Less => i,
                std::cmp::Ordering::Equal => j,
                std::cmp::Ordering::Greater => j,
            };

            pixel_paper.change_pixel_color((i, j), rainbow[choose])
        }
    }

    pixel_paper.draw_straight_line(Red, (0, 5), (5, 0));

    pixel_paper.save("arts/simple_2.png").unwrap()
}
