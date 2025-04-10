use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::game::board::Board;
use crate::game::board::color::Color;
use crate::game::board::move_kind::MoveKind;
use crate::game::board::move_struct::Move;
use crate::game::board::position::Position;

use super::Piece;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Rook {
    color: Color,
    position: Position,
    has_moved: bool,
}

impl Rook {
    const fn original_row(color: Color) -> usize {
        match color {
            Color::White => 7usize,
            Color::Black => 0usize,
            Color::Any => panic!("A rook of color \"Any\" has no original row"),
        }
    }

    const fn original_columns() -> [usize; 2] {
        [0usize, 7usize]
    }

    pub fn with_has_moved(mut self) -> Self {
        self.has_moved = true;
        self
    }

    pub const fn has_moved(&self) -> bool {
        self.has_moved
    }

    pub(crate) fn queen_side_castling_final_position(&self) -> Position {
        match self.color {
            Color::White => (7isize, 3isize).into(),
            Color::Black => (0isize, 3isize).into(),
            Color::Any => panic!("A rook of color \"Any\" has no castling final position"),
        }
    }

    pub(crate) fn king_side_castling_final_position(&self) -> Position {
        match self.color {
            Color::White => (7isize, 5isize).into(),
            Color::Black => (0isize, 5isize).into(),
            Color::Any => panic!("A rook of color \"Any\" has no castling final position"),
        }
    }

    pub const fn set_has_moved(&mut self) {
        self.has_moved = true;
    }
}

impl Hash for Rook {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color.hash(state);
        self.position.hash(state);
    }
}

impl Piece for Rook {
    fn new(position: Position, color: Color) -> Self {
        let rook: Self = Self {
            color,
            position,
            has_moved: false,
        };

        if position.row() != Self::original_row(color) || !Self::original_columns().contains(&position.column()) {
            rook.with_has_moved()
        } else {
            rook
        }
    }

    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> Position {
        self.position
    }

    fn points(&self) -> i16 {
        5i16
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn possible_moves(&self, board: &Board) -> HashSet<Move> {
        const OFFSETS: [(isize, isize); 4] = [
            (-1isize, 0isize),
            (0isize, -1isize),
            (0isize, 1isize),
            (1isize, 0isize),
        ];
        let mut to: Position;
        let mut output: HashSet<Move> = HashSet::new();

        for offset in OFFSETS {
            to = self.position;

            loop {
                to = to + offset;

                let Some(square) = board.square(to) else {
                    break;
                };
                if let Some(piece) = square.piece(Color::Any) {
                    if piece.color() != self.color() {
                        output.insert(Move::new(self.position, to, MoveKind::Attack(Some(*piece))));
                    }
                    break;
                }
                output.insert(Move::new(self.position, to, MoveKind::Attack(None)));
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    use crate::game::board::Board;
    use crate::game::board::board_builder::BoardBuilder;
    use crate::game::board::color::Color;
    use crate::game::board::move_kind::MoveKind;
    use crate::game::board::move_struct::Move;
    use crate::game::pieces::pawn::Pawn;
    use crate::game::pieces::piece_kind::PieceKind;
    use crate::game::pieces::Piece;

    use super::Rook;

    #[test]
    fn test_simple_moves() {
        let board: Board = BoardBuilder::new()
            .with(PieceKind::Rook(Rook::new((3isize, 3isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (0isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 0isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 1isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 5isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 6isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 7isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (6isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (7isize, 3isize).into(), MoveKind::Attack(None)));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    fn test_no_moves() {
        let board: Board = BoardBuilder::new()
            .with(PieceKind::Rook(Rook::new((3isize, 3isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((2isize, 3isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((3isize, 2isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((3isize, 4isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let expected: HashSet<Move> = HashSet::new();

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }

    #[test]
    fn test_capture() {
        let pawn1: PieceKind = PieceKind::Pawn(Pawn::new((2isize, 3isize).into(), Color::White));
        let pawn2: PieceKind = PieceKind::Pawn(Pawn::new((3isize, 2isize).into(), Color::White));
        let pawn3: PieceKind = PieceKind::Pawn(Pawn::new((3isize, 4isize).into(), Color::White));
        let pawn4: PieceKind = PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::White));
        let board: Board = BoardBuilder::new()
            .with(PieceKind::Rook(Rook::new((3isize, 3isize).into(), Color::Black)))
            .with(pawn1)
            .with(pawn2)
            .with(pawn3)
            .with(pawn4)
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack(Some(pawn1))));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack(Some(pawn2))));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack(Some(pawn3))));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack(Some(pawn4))));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(
            expected,
            possible_moves,
            "\nelements expected missing: {:?}\nelements not expected: {:?}",
            expected.difference(&possible_moves),
            possible_moves.difference(&expected),
            );
    }
}
