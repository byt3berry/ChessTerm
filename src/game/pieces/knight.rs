use std::collections::HashSet;

use crate::game::board::Board;
use crate::game::board::color::Color;
use crate::game::board::move_struct::{Move, MoveKind};
use crate::game::board::position::Position;

use super::Piece;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Knight {
    color: Color,
    position: Position,
}

impl Piece for Knight {
    fn new(position: Position, color: Color) -> Self {
        Self {
            color,
            position,
        }
    }

    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn possible_moves(&self, board: &Board) -> HashSet<Move> {
        const OFFSETS: [(isize, isize); 8] = [
            (-1isize, -2isize),
            (-1isize, 2isize),
            (-2isize, -1isize),
            (-2isize, 1isize),
            (1isize, -2isize),
            (1isize, 2isize),
            (2isize, -1isize),
            (2isize, 1isize),
        ];
        let mut to: Position;
        let mut output: HashSet<Move> = HashSet::new();

        if self.pinned(board) {
            return output;
        }

        for offset in OFFSETS {
            to = self.position + offset;

            let Some(square) = board.square(to) else {
                continue;
            };
            if let Some(piece) = square.piece() {
                if piece.color() != self.color() {
                    output.insert(Move::new(self.position, to, MoveKind::Attack, None));
                }
                continue;
            }
            output.insert(Move::new(self.position, to, MoveKind::Attack, None));
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
    use crate::game::board::move_struct::{Move, MoveKind};
    use crate::game::pieces::Piece;
    use crate::game::pieces::bishop::Bishop;
    use crate::game::pieces::king::King;
    use crate::game::pieces::pawn::Pawn;
    use crate::game::pieces::piece_kind::PieceKind;

    use super::Knight;

    #[test]
    fn test_simple_moves() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Knight(Knight::new((3isize, 3isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 1isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 5isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 1isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 5isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 4isize).into(), MoveKind::Attack, None));

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
            .add(PieceKind::Knight(Knight::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 1isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 1isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((5isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((5isize, 4isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
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
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Knight(Knight::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 2isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 4isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 1isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 5isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 1isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 5isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((5isize, 2isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((5isize, 4isize).into(), Color::White)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 1isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 5isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 1isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 5isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 4isize).into(), MoveKind::Attack, None));

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
    fn test_pinned_no_move() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::Knight(Knight::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((6isize, 6isize).into(), Color::White)))
            .build();
        board.set_attacking(Color::White);
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
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
}
