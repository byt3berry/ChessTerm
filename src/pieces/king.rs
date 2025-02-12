use super::{Color, Piece};

#[derive(Clone, Copy, Debug)]
pub struct King {
    color: Color,
}

impl Piece for King {
    fn new(color: Color) -> Self {
        Self {
            color,
        }
    }

    fn move_to(&self, square: crate::board::square::Square) {
        todo!()
    }

    fn color(&self) -> Color {
        self.color
    }
}
