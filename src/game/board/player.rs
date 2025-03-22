use std::collections::HashSet;

use super::color::Color;
use super::move_kind::MoveKind;
use super::move_struct::Move;
use super::position::Position;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Player {
    color: Color,
    possible_moves: HashSet<Move>,
}

impl Player {
    pub(super) fn new(color: Color) -> Self {
        Self {
            color,
            possible_moves: HashSet::new(),
        }
    }

    pub(super) fn set_possible_moves(&mut self, possible_moves: HashSet<Move>) {
        self.possible_moves = possible_moves;
    }

    pub(crate) const fn possible_moves(&self) -> &HashSet<Move> {
        &self.possible_moves
    }

    pub(crate) fn is_attacking(&self, position: Position) -> bool {
        self
            .possible_moves
            .iter()
            .any(|m| matches!(m.kind(), MoveKind::Attack(_)) && m.to() == position)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::game::board::color::Color;
    use crate::game::board::move_kind::MoveKind;
    use crate::game::board::move_struct::Move;

    use super::Player;

    #[test]
    fn test_is_attacking() {
        let mut player: Player = Player::new(Color::Black);
        let mut possible_moves: HashSet<Move> = HashSet::new();
        possible_moves.insert(Move::new((1isize, 4isize).into(), (5isize, 3isize).into(), MoveKind::Attack(None)));
        possible_moves.insert(Move::new((4isize, 6isize).into(), (4isize, 4isize).into(), MoveKind::Attack(None)));
        possible_moves.insert(Move::new((7isize, 0isize).into(), (7isize, 7isize).into(), MoveKind::Attack(None)));
        player.set_possible_moves(possible_moves);

        assert!(player.is_attacking((5isize, 3isize).into()));
        assert!(player.is_attacking((4isize, 4isize).into()));
        assert!(player.is_attacking((7isize, 7isize).into()));
    }
}
