use crate::game::pieces::piece_kind::PieceKind;

use super::color::Color;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct Square {
    color: Color,
    piece: Option<PieceKind>,
}

impl Square {
    pub(super) const fn new(color: Color, piece: Option<PieceKind>) -> Self {
        Self {
            color,
            piece,
        }
    }

    pub(super) const fn set_piece(&mut self, piece: PieceKind) {
        self.piece = Some(piece);
    }

    pub(super) const fn piece_unset(&mut self) -> PieceKind {
        assert!(self.piece.is_some());

        let piece: PieceKind = self.piece.unwrap();
        self.piece = None;
        piece
    }

    pub(crate) const fn piece(&self) -> Option<&PieceKind> {
        self.piece.as_ref()
    }

    pub(super) const fn piece_mut(&mut self) -> Option<&mut PieceKind> {
        self.piece.as_mut()
    }
}
