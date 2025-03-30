use std::ops::{Add, Sub};

use super::{ROWS, COLUMNS};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Position {
    row: isize,
    column: isize,
}

impl Position {
    pub(super) const fn to_index(self) -> Option<usize> {
        if self.row < 0 || self.row as usize >= ROWS || self.column < 0 || self.column as usize >= COLUMNS {
            return None;
        }

        Some(self.row as usize * ROWS + self.column as usize)
    }

    pub(crate) fn row(&self) -> usize {
        assert!(self.row >= 0, "position {self:?} is invalid (row is negative)");

        self.row as usize
    }

    pub(crate) fn column(&self) -> usize {
        assert!(self.column >= 0, "position {self:?} is invalid (column is negative)");

        self.column as usize
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.row() < ROWS && self.column() < COLUMNS
    }

    pub fn from_notation(notation: &str) -> Option<Self> {
        assert_eq!(2, notation.len());
        let notation: Vec<char> = notation.chars().collect();

        let column: isize = match notation[0] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return None,
        };

        let row: isize = match notation[1] {
            '1'..='8' => 8 - notation[1].to_digit(10)? as isize,
            _ => return None,
        };

        Some((row, column).into())
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        (value.0 as isize, value.1 as isize).into()
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

impl Add<(isize, isize)> for Position {
    type Output = Self;

    fn add(self, value: (isize, isize)) -> Self::Output {
        (self.row + value.0, self.column + value.1).into()
    }
}

impl Sub<(isize, isize)> for Position {
    type Output = Self;

    fn sub(self, value: (isize, isize)) -> Self::Output {
        (self.row - value.0, self.column - value.1).into()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::Position;

    #[rstest]
    #[case("b1", Some((7isize, 1isize).into()))]
    #[case("b4", Some((4isize, 1isize).into()))]
    #[case("c3", Some((5isize, 2isize).into()))]
    #[case("c8", Some((0isize, 2isize).into()))]
    #[case("d3", Some((5isize, 3isize).into()))]
    #[case("d7", Some((1isize, 3isize).into()))]
    #[case("e4", Some((4isize, 4isize).into()))]
    #[case("f3", Some((5isize, 5isize).into()))]
    #[case("h3", Some((5isize, 7isize).into()))]
    #[case("h4", Some((4isize, 7isize).into()))]
    fn test_from_notation(
        #[case]
        notation: &str,
        #[case]
        expected: Option<Position>
    ) {
        let position = Position::from_notation(notation);

        assert_eq!(expected, position);
    }

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

    #[rstest]
    #[case((-2isize, 3isize).into())]
    #[should_panic]
    fn test_position_row_panic(
        #[case]
        position: Position
    ) {
        position.row();
    }

    #[rstest]
    #[case((2isize, -3isize).into())]
    #[should_panic]
    fn test_position_column_panic(
        #[case]
        position: Position
    ) {
        position.column();
    }

    #[rstest]
    #[case((2isize, 3isize).into(), Some(19usize))]
    #[case((10isize, 12isize).into(), None)]
    fn test_position_index(
        #[case()]
        position: Position,
        #[case()]
        expected: Option<usize>
        ) {
        let index: Option<usize> = position.to_index();

        assert_eq!(expected, index);
    }
}
