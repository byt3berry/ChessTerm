use crate::game::pieces::piece_kind::PieceKind;

use super::color::Color;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) struct Square (Option<PieceKind>);

impl Square {
    pub(super) const fn set_piece(&mut self, piece: PieceKind) {
        self.0 = Some(piece);
    }

    pub(super) const fn piece_unset(&mut self) -> Option<PieceKind> {
        if let Some(piece) = self.0 {
            self.0 = None;
            return Some(piece);
        }

        None
    }

    pub(crate) fn piece(&self, color: Color) -> Option<&PieceKind> {
        if color == Color::Any || self.0.is_some_and(|piece| piece.color() == color) {
            return self.0.as_ref();
        }
        
        None
    }

    pub(super) fn piece_mut(&mut self, color: Color) -> Option<&mut PieceKind> {
        if color == Color::Any || self.0.is_some_and(|piece| piece.color() == color) {
            return self.0.as_mut();
        }
        
        None
    }
}

#[cfg(test)]
impl Square {
    pub(super) const fn new(piece: Option<PieceKind>) -> Self {
        Self(piece)
    }
}
