use crate::pieces::color::Color;
use crate::pieces::piece_kind::PieceKind;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Square {
    color: Color,
    piece: Option<PieceKind>,
}

impl Square {
    pub const fn new(color: Color, piece: Option<PieceKind>) -> Self {
        Self {
            color,
            piece,
        }
    }

    pub const fn set_piece(&mut self, piece: PieceKind) {
        self.piece = Some(piece);
    }

    pub const fn color(&self) -> Color {
        self.color
    }

    pub const fn piece(&self) -> Option<&PieceKind> {
        self.piece.as_ref()
    }

    pub const fn piece_mut(&mut self) -> Option<&mut PieceKind> {
        self.piece.as_mut()
    }
}
