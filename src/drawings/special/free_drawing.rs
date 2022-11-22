use crate::{
    pixels_table::PixelNotFound,
    position::Position,
    prelude::{ColorSelector, PixelDescriptor, PixelsTable, ToPosition},
};

pub struct FreeDrawing<const H: usize, const W: usize> {
    table: [[PixelDescriptor; H]; W],
    color: PixelDescriptor,
    pen_down: bool,
    cursor_position: Position,
}

impl<const H: usize, const W: usize> FreeDrawing<H, W> {
    pub fn new<C: ColorSelector>(color: C) -> FreeDrawing<H, W> {
        Self {
            table: Self::get_default_table(),
            color: color.get_rgb().into(),
            pen_down: true,
            cursor_position: Default::default(),
        }
    }

    pub fn cursor_position(&self) -> Position {
        self.cursor_position
    }
}

impl<const H: usize, const W: usize> FreeDrawing<H, W> {
    fn mark_cursor_position(&mut self) -> Result<(), PixelNotFound> {
        if self.pen_down {
            self.change_pixel_at(self.cursor_position, self.color)
        } else {
            Ok(())
        }
    }

    pub fn pen_up(&mut self) -> &mut FreeDrawing<H, W> {
        self.pen_down = false;
        self
    }

    pub fn pen_down(&mut self) -> &mut FreeDrawing<H, W> {
        self.pen_down = true;
        self
    }

    pub fn start_drawing<P: ToPosition<H, W>>(&mut self, pos: P) -> &mut FreeDrawing<H, W> {
        self.cursor_position = pos.get_position();
        self.mark_cursor_position().unwrap();
        self
    }

    pub fn go_up(&mut self, how_many: usize) -> &mut FreeDrawing<H, W> {
        for _ in 0..how_many {
            self.cursor_position = self.cursor_position.top(1).unwrap();
            self.mark_cursor_position().unwrap();
        }

        self
    }

    pub fn go_down(&mut self, how_many: usize) -> &mut FreeDrawing<H, W> {
        for _ in 0..how_many {
            self.cursor_position = self.cursor_position.down(1).unwrap();
            self.mark_cursor_position().unwrap();
        }

        self
    }

    pub fn go_right(&mut self, how_many: usize) -> &mut FreeDrawing<H, W> {
        for _ in 0..how_many {
            self.cursor_position = self.cursor_position.right(1).unwrap();
            self.mark_cursor_position().unwrap();
        }

        self
    }

    pub fn go_left(&mut self, how_many: usize) -> &mut FreeDrawing<H, W> {
        for _ in 0..how_many {
            self.cursor_position = self.cursor_position.left(1).unwrap();
            self.mark_cursor_position().unwrap();
        }

        self
    }

    pub fn go_left_up(&mut self, how_many: usize) -> &mut FreeDrawing<H, W> {
        for _ in 0..how_many {
            self.cursor_position = self.cursor_position.left_top(1).unwrap();
            self.mark_cursor_position().unwrap();
        }

        self
    }

    pub fn go_left_down(&mut self, how_many: usize) -> &mut FreeDrawing<H, W> {
        for _ in 0..how_many {
            self.cursor_position = self.cursor_position.left_down(1).unwrap();
            self.mark_cursor_position().unwrap();
        }

        self
    }

    pub fn go_right_up(&mut self, how_many: usize) -> &mut FreeDrawing<H, W> {
        for _ in 0..how_many {
            self.cursor_position = self.cursor_position.right_top(1).unwrap();
            self.mark_cursor_position().unwrap();
        }

        self
    }

    pub fn go_right_down(&mut self, how_many: usize) -> &mut FreeDrawing<H, W> {
        for _ in 0..how_many {
            self.cursor_position = self.cursor_position.right_down(1).unwrap();
            self.mark_cursor_position().unwrap();
        }

        self
    }

    pub fn get_branch(&self) -> FreeDrawing<H, W> {
        let mut fd = FreeDrawing::<H, W>::new(match self.color {
            PixelDescriptor::Pixel(p) => p.color(),
            _ => unreachable!(),
        });
        fd.start_drawing(self.cursor_position);

        fd
    }

    pub fn with_branch(
        &mut self,
        setup_branch: impl FnOnce(&mut FreeDrawing<H, W>),
    ) -> &mut FreeDrawing<H, W> {
        let mut branch = self.get_branch();
        setup_branch(&mut branch);

        self.draw_exactly_from(&branch);
        self
    }
}

impl<const H: usize, const W: usize> PixelsTable<H, W> for FreeDrawing<H, W> {
    fn get_table(&self) -> &[[PixelDescriptor; H]; W] {
        &self.table
    }

    fn get_mut_table(&mut self) -> &mut [[PixelDescriptor; H]; W] {
        &mut self.table
    }
}
