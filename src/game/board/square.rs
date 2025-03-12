use crate::game::pieces::piece_kind::PieceKind;

use super::color::Color;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct Square {
    piece: Option<PieceKind>,
}

impl Square {
    pub(super) const fn set_piece(&mut self, piece: PieceKind) {
        self.piece = Some(piece);
    }

    pub(super) const fn piece_unset(&mut self) -> PieceKind {
        assert!(self.piece.is_some());

        let piece: PieceKind = self.piece.unwrap();
        self.piece = None;
        piece
    }

    pub(crate) fn piece(&self, color: Color) -> Option<&PieceKind> {
        if color == Color::Any || self.piece.is_some_and(|piece| piece.color() == color) {
            return self.piece.as_ref();
        }
        
        None
    }

    pub(super) fn piece_mut(&mut self, color: Color) -> Option<&mut PieceKind> {
        if color == Color::Any || self.piece.is_some_and(|piece| piece.color() == color) {
            return self.piece.as_mut();
        }
        
        None
    }
}

#[cfg(test)]
impl Square {
    pub(super) const fn new(piece: Option<PieceKind>) -> Self {
        Self {
            piece,
        }
    }
}
