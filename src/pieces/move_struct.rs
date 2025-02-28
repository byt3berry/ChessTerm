use crate::board::position::Position;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MoveKind {
    Attack,
    CastleKingSide,
    CastleQueenSide,
    EnPassant,
    PawnMove,
    Promotion,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Move {
    from: Position,
    to: Position,
    kind: MoveKind,
}

impl Move {
    pub const fn new(from: Position, to: Position, kind: MoveKind) -> Self {
        Self {
            from,
            to,
            kind,
        }
    }

    pub const fn kind(&self) -> MoveKind {
        self.kind
    }

    pub const fn from(&self) -> &Position {
        &self.from
    }

    pub const fn to(&self) -> &Position {
        &self.to
    }
}
