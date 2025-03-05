#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum Color {
    #[default]
    White,
    Black,
}

impl Color {
    pub(crate) const fn other(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
