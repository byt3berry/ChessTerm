use super::{Color, Piece};

#[derive(Clone, Copy, Debug)]
pub struct Knight {
    color: Color,
}

impl Piece for Knight {
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
