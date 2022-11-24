use crate::{
    drawings::special::FreeDrawing,
    pixels_table::DrawingContext,
    prelude::{Drawable, MoreMethodsForPixelsTable, PixelsTable},
};

pub struct Chico;

impl Default for Chico {
    fn default() -> Self {
        Self
    }
}

impl Drawable<40, 40> for Chico {
    fn setup(&self, drawing_ctx: &mut DrawingContext<40, 40>) {
        let chico = drawing_ctx.canvas_mut();

        let border_color = [79, 48, 17];
        let mut borders = FreeDrawing::new(border_color);
        borders
            .start_drawing((10, 1))
            .go_right(4)
            .go_right_down(2)
            .with_branch(|right| {
                right
                    .go_right(1)
                    .go_up(1)
                    .go_up(4)
                    .go_right_up(5)
                    .go_right(3)
                    .go_down(1)
                    .go_left_down(4)
                    .go_down(1)
                    .with_branch(|right| {
                        right
                            .go_right(1)
                            .go_right_up(1)
                            .go_right(10)
                            .go_right_down(1)
                            .go_right(1)
                            .with_branch(|top| {
                                top.go_up(2)
                                    .go_left_up(2)
                                    .go_left(2)
                                    .go_left_up(1)
                                    .go_up(1)
                                    .go_right(4)
                                    .go_right_down(1)
                                    .go_right(1)
                                    .go_right_down(3)
                                    .go_down(6)
                                    .with_branch(|right| {
                                        right
                                            .go_right(1)
                                            .go_right_up(2)
                                            .go_right(3)
                                            .go_down(2)
                                            .go_left_down(3)
                                            .go_left(2);
                                    })
                                    .with_branch(|down| {
                                        down.go_down(2);
                                    });
                            })
                            .with_branch(|right_down| {
                                right_down.go_right_down(2).go_down(1).go_right_down(1);
                            });
                    })
                    .with_branch(|left| {
                        left.go_left_down(3);
                    });
            })
            .with_branch(|down| {
                down.go_down(2).go_left(1).with_branch(|left| {
                    left.go_left(1).go_left_up(1).go_left(1).go_left_up(2);
                });
            })
            .pen_up()
            .go_down(3)
            .pen_down()
            .go_left(1)
            .go_down(3)
            .go_left_down(2)
            .go_right_down(1)
            .with_branch(|right| {
                right.go_right(1).go_right_up(1);
                // Color changes here
            })
            .go_down(1)
            .go_left_down(2)
            .go_right(1)
            .with_branch(|right| {
                right
                    .go_right(2)
                    .with_branch(|right_up| {
                        right_up.go_right_up(1).go_right(1);
                    })
                    .go_down(1)
                    .go_left_down(1)
                    .go_down(2)
                    .go_right_down(1);
            })
            .go_down(1)
            .go_left_down(1)
            .go_down(1)
            .go_left_down(2)
            .go_right_down(1)
            .go_right(4)
            .go_right_down(5)
            .go_down(2)
            .go_right_down(1)
            .with_branch(|right_up| {
                right_up
                    .go_right_up(1)
                    .go_up(2)
                    .go_left_up(1)
                    .go_up(1)
                    .go_right_up(2);
            })
            .go_right_down(1)
            .go_right(8)
            .go_right_up(2)
            .with_branch(|up| {
                up.go_up(2)
                    .with_branch(|right| {
                        right
                            .go_right(2)
                            .with_branch(|down| {
                                down.go_down(1);
                            })
                            .go_up(1)
                            .go_right_up(2)
                            .go_up(1);
                    })
                    .go_left_up(1)
                    .with_branch(|left_down| {
                        left_down.go_left_down(1).go_down(2);
                    })
                    .go_up(1)
                    .with_branch(|right_up| {
                        right_up.go_right_up(1);
                    })
                    .go_left(1)
                    .go_up(1)
                    .go_left_up(1);
            })
            .go_right(2)
            .go_right_down(2)
            .go_right(2)
            .go_right_up(1)
            .with_branch(|right_up| {
                right_up.go_right_up(1);
            })
            .go_left_up(2)
            .go_left(2)
            .go_up(1)
            .go_right_up(3)
            .go_up(1)
            .go_right_up(1)
            .with_branch(|left_up| {
                left_up.go_left_up(1);
            })
            .go_right_up(1)
            .go_right(1)
            .with_branch(|up| {
                up.go_up(3)
                    .go_left_up(3)
                    .with_branch(|up| {
                        up.go_up(1).go_right(1);
                    })
                    .go_left(3)
                    .go_left_up(1)
                    .go_left(1)
                    .with_branch(|left_up| {
                        left_up.go_left_up(1);
                    })
                    .go_down(5)
                    .go_left_down(2)
                    .go_left(1)
                    .go_left_down(1)
                    .go_left(6)
                    .go_left_up(1)
                    .go_left(2)
                    .go_left_up(2)
                    .go_up(5)
                    .go_right_up(4)
                    .go_right_down(1)
                    .go_down(1)
                    .with_branch(|left| {
                        left.go_left(2);
                    })
                    .go_down(1)
                    .with_branch(|left_down| {
                        left_down
                            .go_left_down(1)
                            .go_down(3)
                            .go_left(1)
                            .go_up(3)
                            .go_left_down(1)
                            .go_down(2);
                    })
                    .go_right(1)
                    .go_right_down(2)
                    .go_right(1)
                    .go_up(4)
                    .with_branch(|right| {
                        right
                            .go_right(1)
                            .go_right_up(1)
                            .go_right(1)
                            .go_right_down(1)
                            .go_down(1)
                            .go_left(2)
                            .go_left_down(1)
                            .go_right_down(1)
                            .go_down(3)
                            .go_right(1)
                            .go_up(3)
                            .go_right_down(1)
                            .go_down(2);
                    })
                    .go_up(1);
            })
            .go_right(4)
            .go_right_up(1)
            .go_left_up(2)
            .go_up(1)
            .go_left_up(2)
            .go_up(1)
            .go_left_up(1)
            .with_branch(|left| {
                left.go_left(1).go_left_up(2);
                // Color changes
            })
            .go_up(2)
            .go_left_up(1)
            .go_up(2);

        chico.draw_from_table_exact(&borders);

        let yellow = [255, 235, 153];
        chico.boundary_fill_color((11, 2), yellow, border_color, false);
        chico.boundary_fill_color((10, 9), yellow, border_color, false);
        chico.boundary_fill_color((3, 22), yellow, border_color, false);
        chico.boundary_fill_color((14, 31), yellow, border_color, false);

        let pink = [240, 193, 199];
        chico.boundary_fill_color((11, 10), pink, border_color, false);
        chico.boundary_fill_color((29, 2), pink, border_color, false);
        chico.boundary_fill_color((28, 6), pink, border_color, false);
        chico.boundary_fill_color((30, 23), pink, border_color, false);
        chico.boundary_fill_color((38, 29), pink, border_color, false);
        chico.boundary_fill_color((28, 33), pink, border_color, false);

        // (_, 27)
        chico.change_pixel_color((33, 27), pink);
        chico.change_pixel_color((34, 26), pink);

        let dark_pink = (154, 93, 101);
        chico.draw_free_drawing(dark_pink, |fd| {
            fd.start_drawing((19, 8))
                .go_up(1)
                .go_right_up(1)
                .go_up(1)
                .go_right_up(1);
        });

        chico.draw_free_drawing(dark_pink, |fd| {
            fd.start_drawing((17, 13))
                .go_up(3)
                .go_right_up(1)
                .go_up(1)
                .go_right_up(1);
        });

        chico.draw_free_drawing(dark_pink, |fd| {
            fd.start_drawing((17, 18)).go_up(2).go_right_up(1);
        });

        chico.draw_free_drawing(dark_pink, |fd| {
            fd.start_drawing((17, 20)).go_up(1);
        });

        chico.draw_free_drawing(dark_pink, |fd| {
            fd.start_drawing((18, 28))
                .go_up(1)
                .go_left_up(1)
                .go_up(1)
                .go_left_up(1)
                .pen_up()
                .go_up(2)
                .pen_down()
                .go_right(1)
                .go_left_up(2);
        });

        chico.draw_free_drawing(dark_pink, |fd| {
            fd.start_drawing((24, 28)).go_down(6);
        });

        chico.draw_free_drawing(dark_pink, |fd| {
            fd.start_drawing((28, 24)).go_down(3);
        });

        chico.draw_free_drawing(dark_pink, |fd| {
            fd.start_drawing((29, 10)).go_down(2).go_right_down(1);
        });

        chico.draw_free_drawing(dark_pink, |fd| {
            fd.start_drawing((24, 7)).go_down(6);
        });

        chico.change_pixel_color((25, 5), pink);

        let purple = (181, 126, 164);
        chico.draw_free_drawing(purple, |fd| {
            fd.start_drawing((26, 6)).go_down(1);
        });

        chico.change_pixel_color((28, 6), yellow);

        chico.draw_free_drawing(yellow, |fd| {
            fd.start_drawing((24, 29))
                .go_right_down(2)
                .go_down(2)
                .go_left(2)
                .go_right_down(1);
        });

        chico.draw_free_drawing(purple, |fd| {
            fd.start_drawing((25, 29)).go_down(2).go_right(1).go_up(1);
        });

        chico.draw_free_drawing(purple, |fd| {
            fd.start_drawing((32, 16))
                .go_right(2)
                .go_down(1)
                .go_left(2)
                .go_down(1)
                .go_right(2)
                .go_left_down(1);
        });

        chico.draw_free_drawing(yellow, |fd| {
            fd.start_drawing((33, 15))
                .go_down(2)
                .go_right(1)
                .go_right_down(1)
                .with_branch(|down| {
                    down.go_left_down(1).go_right(2);
                })
                .go_right_up(1)
                .go_right(1)
                .go_up(2);
        });

        // Face
        let face_color = (247, 243, 234);
        chico.boundary_fill_color((21, 11), face_color, border_color, false);

        chico.change_pixel_color((19, 13), face_color);

        chico.draw_free_drawing(face_color, |fd| {
            fd.start_drawing((20, 19)).go_right_up(1).go_right(1);
        });

        chico.change_pixel_color((25, 10), purple);
        chico.change_pixel_color((25, 23), purple);
        chico.change_pixel_color((27, 16), purple);

        let light_pink = (250, 225, 228);
        chico.draw_free_drawing(light_pink, |fd| {
            fd.start_drawing((26, 10))
                .go_down(1)
                .go_right_down(1)
                .go_up(2)
                .go_right(1)
                .go_down(2)
                .go_right_up(1);
        });

        let light_pink = (250, 225, 228);
        chico.draw_free_drawing(light_pink, |fd| {
            fd.start_drawing((26, 23))
                .go_down(1)
                .go_left_down(1)
                .go_up(2)
                .go_left(1)
                .go_down(2)
                .go_left_up(1);
        });

        // Congrats 🧁
    }
}
