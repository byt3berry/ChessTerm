#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum Color {
    White,
    Black,
    #[default]
    Any,
}

impl Color {
    pub(crate) const fn other(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
            Self::Any => panic!("Can't call other on color \"Any\"")
        }
    }
}
