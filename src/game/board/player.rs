use std::collections::HashSet;

use super::color::Color;
use super::move_struct::{Move, MoveKind};
use super::position::Position;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Player {
    color: Color,
    attacking: HashSet<Move>,
}

impl Player {
    pub(super) fn new(color: Color) -> Self {
        Self {
            color,
            attacking: HashSet::new(),
        }
    }

    pub(super) fn set_attacking(&mut self, attacking: HashSet<Move>) {
        self.attacking = attacking;
    }

    pub(crate) const fn attacking(&self) -> &HashSet<Move> {
        &self.attacking
    }

    pub(crate) fn is_attacking(&self, position: Position) -> bool {
        self.attacking.iter().any(|m| m.kind() == MoveKind::Attack && m.to() == position)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::game::board::color::Color;
    use crate::game::board::move_struct::{Move, MoveKind};

    use super::Player;

    #[test]
    fn test_is_attacking() {
        let mut player: Player = Player::new(Color::Black);
        let mut attacking: HashSet<Move> = HashSet::new();
        attacking.insert(Move::new((1isize, 4isize).into(), (5isize, 3isize).into(), MoveKind::Attack, None));
        attacking.insert(Move::new((4isize, 6isize).into(), (4isize, 4isize).into(), MoveKind::Attack, None));
        attacking.insert(Move::new((7isize, 0isize).into(), (7isize, 7isize).into(), MoveKind::Attack, None));
        player.set_attacking(attacking);

        assert!(player.is_attacking((5isize, 3isize).into()));
        assert!(player.is_attacking((4isize, 4isize).into()));
        assert!(player.is_attacking((7isize, 7isize).into()));
    }
}
