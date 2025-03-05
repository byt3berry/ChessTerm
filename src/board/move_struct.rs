use crate::board::position::Position;

use super::pin_kind::PinKind;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MoveKind {
    Attack, // A move that can take a piece
    CastleKingSide,
    CastleQueenSide,
    EnPassant(Position),
    PawnMove, // A pawn move that can not take a piece (vertical pawn move)
    Promotion,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Move {
    from: Position,
    to: Position,
    kind: MoveKind,
    pin: Option<PinKind>,
}

impl Move {
    pub fn new(from: Position, to: Position, kind: MoveKind, pin: Option<PinKind>) -> Self {
        assert_ne!(to, from);
        assert!(from.is_valid());
        assert!(to.is_valid());

        Self {
            from,
            to,
            kind,
            pin,
        }
    }

    pub const fn kind(&self) -> MoveKind {
        self.kind
    }

    pub const fn from(&self) -> Position {
        self.from
    }

    pub const fn to(&self) -> Position {
        self.to
    }

    pub const fn pin(&self) -> Option<PinKind> {
        self.pin
    }
}

#[cfg(test)]
mod tests {
    use crate::board::move_struct::MoveKind;
    use crate::board::pin_kind::PinKind;
    use crate::board::position::Position;

    use super::Move;

    #[test]
    #[should_panic]
    fn test_move_to_same_square() {
        let from: Position = (3isize, 3isize).into();
        let to: Position = (3isize, 3isize).into();
        let kind: MoveKind = MoveKind::Attack;
        let pin: Option<PinKind> = None;

        Move::new(from, to, kind, pin);
    }

    #[test]
    fn test_move_valid_square() {
        let from: Position = (3isize, 3isize).into();
        let to: Position = (7isize, 4isize).into();
        let kind: MoveKind = MoveKind::Attack;
        let pin: Option<PinKind> = None;

        Move::new(from, to, kind, pin);
    }

    #[test]
    #[should_panic]
    fn test_move_invalid_square() {
        let from: Position = (54isize, 65isize).into();
        let to: Position = (54isize, 66isize).into();
        let kind: MoveKind = MoveKind::Attack;
        let pin: Option<PinKind> = None;

        Move::new(from, to, kind, pin);
    }
}
