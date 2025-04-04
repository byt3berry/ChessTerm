use crate::game::board::{color::Color, square::Square};

use super::{Drawable, SQUARE_SIZE};

impl Drawable for Square {
    fn drawing(&self) -> [u8; SQUARE_SIZE*SQUARE_SIZE] {
        self
            .piece(Color::Any)
            .map_or([0; SQUARE_SIZE*SQUARE_SIZE], Drawable::drawing)
    }

}
