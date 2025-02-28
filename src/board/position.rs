use std::ops::Add;

use crate::board::{ROWS, COLUMNS};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    row: isize,
    column: isize,
}

impl Position {
    pub const fn to_index(self) -> Option<usize> {
        if self.row < 0 || self.row as usize >= ROWS || self.column < 0 || self.column as usize >= COLUMNS {
            return None;
        }

        Some(self.row as usize * ROWS + self.column as usize)
    }

    pub fn row(&self) -> usize {
        assert!(self.row >= 0, "position {self:?} is invalid");
        assert!(self.row < ROWS as isize, "position {self:?} is invalid");

        self.row as usize
    }

    pub fn column(&self) -> usize {
        assert!(self.column >= 0, "position {self:?} is invalid");
        assert!(self.column < COLUMNS as isize, "position {self:?} is invalid");

        self.column as usize
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            row: value.0 as isize,
            column: value.1 as isize,
        }
    }
}

impl From<(isize, isize)> for Position {
    fn from(value: (isize, isize)) -> Self {
        Self {
            row: value.0,
            column: value.1,
        }
    }
}

impl From<Position> for (usize, usize) {
    fn from(value: Position) -> Self {
        assert!(value.row >= 0, "position {value:?} is invalid");
        assert!(value.column >= 0, "position {value:?} is invalid");

        (value.row as usize, value.column as usize)
    }
}

impl Add<Self> for Position {
    type Output = Self;

    fn add(self, value: Self) -> Self::Output {
        (self.row + value.row, self.column + value.column).into()
    }
}

impl Add<Self> for &Position {
    type Output = Position;

    fn add(self, value: Self) -> Self::Output {
        (self.row + value.row, self.column + value.column).into()
    }
}

impl Add<(isize, isize)> for Position {
    type Output = Self;

    fn add(self, value: (isize, isize)) -> Self::Output {
        (self.row + value.0, self.column + value.1).into()
    }
}

impl Add<(isize, isize)> for &Position {
    type Output = Position;

    fn add(self, value: (isize, isize)) -> Self::Output {
        (self.row + value.0, self.column + value.1).into()
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn test_position_add_position() {
        let position1: Position = (2isize, 3isize).into();
        let position2: Position = (4isize, 5isize).into();
        let expected: Position = (6isize, 8isize).into();

        let position3: Position = position1 + position2;

        assert_eq!(expected, position3);
    }

    #[test]
    fn test_position_add_isize() {
        let position1: Position = (2isize, 3isize).into();
        let position2: (isize, isize) = (4isize, 5isize);
        let expected: Position = (6isize, 8isize).into();

        let position3: Position = position1 + position2;

        assert_eq!(expected, position3);
    }

    #[test]
    fn test_position_row_valid() {
        let position: Position = (2isize, 3isize).into();
        let expected: usize = 2usize;

        let row: usize = position.row();

        assert_eq!(expected, row);
    }

    #[test]
    #[should_panic]
    fn test_position_row_invalid_negative() {
        let position: Position = (-2isize, 3isize).into();

        position.row();
    }

    #[test]
    #[should_panic]
    fn test_position_row_invalid_large() {
        let position: Position = (isize::MAX, 3isize).into();

        position.row();
    }

    #[test]
    fn test_position_index_valid() {
        let position: Position = (2isize, 3isize).into();
        let expected: Option<usize> = Some(19usize);

        let index: Option<usize> = position.to_index();

        assert_eq!(expected, index);
    }

    #[test]
    fn test_position_index_invalid() {
        let position: Position = (10isize, 12isize).into();
        let expected: Option<usize> = None;

        let index: Option<usize> = position.to_index();

        assert_eq!(expected, index);
    }
}
