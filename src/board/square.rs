use crate::pieces::{Color, PieceKind};

#[derive(Clone, Copy, Debug, Default)]
pub struct Square {
    color: Color,
    piece: Option<PieceKind>,
}

impl Square {
    pub fn new(color: Color, piece: Option<PieceKind>) -> Self {
        Self {
            color,
            piece,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn piece(&self) -> Option<&PieceKind> {
        self.piece.as_ref()
    }
}
