use crate::board::square::Square;
use crate::drawer::{Drawable, SQUARE_SIZE};

impl Drawable for Square {
    fn drawing(&self) -> [u8; SQUARE_SIZE*SQUARE_SIZE] {
        self
            .piece()
            .map_or([0; SQUARE_SIZE*SQUARE_SIZE], Drawable::drawing)
    }

}
