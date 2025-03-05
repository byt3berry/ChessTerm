#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PinKind {
    Horizontal,
    Vertical,
    DiagonalTopLeftBottomRight,
    DiagonalTopRightBottomLeft,
}

impl From<(isize, isize)> for PinKind {
    fn from(value: (isize, isize)) -> Self {
        match value {
            (-1isize, -1isize) | (1isize, 1isize)  => PinKind::DiagonalTopLeftBottomRight,
            (-1isize, 0isize)  | (1isize, 0isize)  => PinKind::Vertical,
            (-1isize, 1isize)  | (1isize, -1isize) => PinKind::DiagonalTopRightBottomLeft,
            (0isize, -1isize)  | (0isize, 1isize)  => PinKind::Horizontal,
            _                                      => panic!("Invalid pin offset {value:?}"),
        }
    }
}
