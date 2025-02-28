#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Color {
    #[default]
    WHITE,
    BLACK,
}

impl Color {
    pub const fn other(self) -> Self {
        match self {
            Self::WHITE => Self::BLACK,
            Self::BLACK => Self::WHITE,
        }
    }
}
