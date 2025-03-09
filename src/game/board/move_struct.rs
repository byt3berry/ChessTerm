use crate::game::board::position::Position;

use super::{move_kind::MoveKind, pin_kind::PinKind};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Move {
    from: Position,
    to: Position,
    kind: MoveKind,
    pin: Option<PinKind>,
}

impl Move {
    pub(crate) fn new(from: Position, to: Position, kind: MoveKind, pin: Option<PinKind>) -> Self {
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

    pub(super) const fn kind(&self) -> MoveKind {
        self.kind
    }

    pub(crate) const fn from(&self) -> Position {
        self.from
    }

    pub(crate) const fn to(&self) -> Position {
        self.to
    }

    pub(crate) const fn pin(&self) -> Option<PinKind> {
        self.pin
    }
}

#[cfg(test)]
mod tests {
    use crate::game::board::move_struct::MoveKind;
    use crate::game::board::pin_kind::PinKind;
    use crate::game::board::position::Position;

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
