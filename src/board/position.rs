use super::ROWS;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    row: usize,
    column: usize,
}

impl Position {
    pub fn to_index(&self) -> usize {
        self.row * ROWS + self.column
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            row: value.0,
            column: value.1,
        }
    }
}

impl From<Position> for (usize, usize) {
    fn from(value: Position) -> Self {
        (value.row, value.column)
    }
}
