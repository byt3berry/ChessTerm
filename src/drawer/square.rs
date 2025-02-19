use crate::board::square::Square;
use super::{Drawable, SQUARE_SIZE};

impl Drawable for Square {
    fn get_drawing(&self) -> [u8; SQUARE_SIZE*SQUARE_SIZE] {
        // [0; SQUARE_SIZE*SQUARE_SIZE]
        match self.piece() {
            Some(piece) => piece.get_drawing(),
            None => [0; SQUARE_SIZE*SQUARE_SIZE],
        }
    }

}
