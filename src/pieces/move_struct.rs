use crate::board::position::Position;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MoveKind {
    Attack,
    CastleKingSide,
    CastleQueenSide,
    EnPassant(Position),
    PawnMove,
    Promotion,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Move {
    from: Position,
    to: Position,
    kind: MoveKind,
}

impl Move {
    pub fn new(from: Position, to: Position, kind: MoveKind) -> Self {
        assert_ne!(to, from);
        assert!(from.is_valid());
        assert!(to.is_valid());

        Self {
            from,
            to,
            kind,
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
}

#[cfg(test)]
mod tests {
    use crate::board::position::Position;
    use crate::pieces::move_struct::MoveKind;

    use super::Move;

    #[test]
    #[should_panic]
    fn test_move_to_same_square() {
        let from: Position = (3isize, 3isize).into();
        let to: Position = (3isize, 3isize).into();
        let kind: MoveKind = MoveKind::Attack;

        Move::new(from, to, kind);
    }

    #[test]
    fn test_move_valid_square() {
        let from: Position = (3isize, 3isize).into();
        let to: Position = (7isize, 4isize).into();
        let kind: MoveKind = MoveKind::Attack;

        Move::new(from, to, kind);
    }

    #[test]
    #[should_panic]
    fn test_move_invalid_square() {
        let from: Position = (54isize, 65isize).into();
        let to: Position = (54isize, 66isize).into();
        let kind: MoveKind = MoveKind::Attack;

        Move::new(from, to, kind);
    }
}
