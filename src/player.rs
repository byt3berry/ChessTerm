use crate::pieces::Color;

#[derive(Debug)]
pub struct Player {
    color: Color,
}

impl Player {
    pub fn new(color: Color) -> Self {
        Self {
            color,
        }
    }
}
