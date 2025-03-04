use std::collections::HashSet;

use crate::game::board::Board;
use crate::game::board::color::Color;
use crate::game::board::move_struct::{Move, MoveKind};
use crate::game::board::pin_kind::PinKind;
use crate::game::board::position::Position;

use super::Piece;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct Queen {
    color: Color,
    position: Position,
}

impl Piece for Queen {
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
            (-1isize, -1isize),
            (-1isize, 0isize),
            (-1isize, 1isize),
            (0isize, -1isize),
            (0isize, 1isize),
            (1isize, -1isize),
            (1isize, 0isize),
            (1isize, 1isize),
        ];
        let mut to: Position;
        let mut king_found: bool;
        let mut pin_kind: Option<PinKind> = None;
        let mut output: HashSet<Move> = HashSet::new();

        for offset in OFFSETS {
            if !self.pinned_but_movable(board, offset.into()) {
                    continue;
            }

            to = self.position;

            loop {
                to = to + offset;

                let Some(square) = board.square(to) else {
                    break;
                };
                if let Some(piece) = square.piece() {
                    if piece.color() != self.color() {
                        king_found = self.find_king(board, to, offset);

                        if king_found { 
                            pin_kind = Some(offset.into());
                        }

                        output.insert(Move::new(self.position, to, MoveKind::Attack, pin_kind));
                    }
                    break;
                }
                output.insert(Move::new(self.position, to, MoveKind::Attack, None));
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
    use crate::game::board::move_struct::{Move, MoveKind};
    use crate::game::board::pin_kind::PinKind;
    use crate::game::pieces::bishop::Bishop;
    use crate::game::pieces::rook::Rook;
    use crate::game::pieces::Piece;
    use crate::game::pieces::king::King;
    use crate::game::pieces::pawn::Pawn;
    use crate::game::pieces::piece_kind::PieceKind;

    use super::Queen;

    #[test]
    fn test_simple_moves() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Queen(Queen::new((3isize, 3isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (0isize, 0isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (0isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (0isize, 6isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 1isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 5isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 0isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 1isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 5isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 6isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 7isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 1isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 5isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (6isize, 0isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (6isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (6isize, 6isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (7isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (7isize, 7isize).into(), MoveKind::Attack, None));

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
    fn test_move_pin_diagonal_tlbr() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Queen(Queen::new((3isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 5isize).into(), Color::White)))
            .add(PieceKind::King(King::new((5isize, 6isize).into(), Color::White)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 4isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 4isize).into(), (4isize, 5isize).into(), MoveKind::Attack, Some(PinKind::DiagonalTopLeftBottomRight)));

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
    fn test_move_pin_diagonal_trbl() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Queen(Queen::new((3isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::White)))
            .add(PieceKind::King(King::new((5isize, 2isize).into(), Color::White)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 4isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 4isize).into(), (4isize, 3isize).into(), MoveKind::Attack, Some(PinKind::DiagonalTopRightBottomLeft)));

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
    fn test_move_pin_vertical() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Queen(Queen::new((3isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::White)))
            .add(PieceKind::King(King::new((5isize, 4isize).into(), Color::White)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 4isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 4isize).into(), (4isize, 4isize).into(), MoveKind::Attack, Some(PinKind::Vertical)));

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
    fn test_move_pin_horizontal() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Queen(Queen::new((3isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 5isize).into(), Color::White)))
            .add(PieceKind::King(King::new((3isize, 6isize).into(), Color::White)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 4isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 4isize).into(), (3isize, 5isize).into(), MoveKind::Attack, Some(PinKind::Horizontal)));

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
            .add(PieceKind::Queen(Queen::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::Black)))
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
            .add(PieceKind::Queen(Queen::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 2isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 3isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 4isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 2isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 4isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 2isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::White)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 4isize).into(), MoveKind::Attack, None));

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
    fn test_pinned_can_move_straight() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::Queen(Queen::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((5isize, 3isize).into(), Color::White)))
            .build();
        board.set_attacking(Color::White);
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 3isize).into(), MoveKind::Attack, None));

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
    fn test_pinned_can_move_diagonal() {
        let mut board: Board = BoardBuilder::new()
            .add(PieceKind::Queen(Queen::new((1isize, 1isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((6isize, 6isize).into(), Color::White)))
            .build();
        board.set_attacking(Color::White);
        let piece: &PieceKind = board
            .piece((1isize, 1isize).into())
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((1isize, 1isize).into(), (2isize, 2isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((1isize, 1isize).into(), (3isize, 3isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((1isize, 1isize).into(), (4isize, 4isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((1isize, 1isize).into(), (5isize, 5isize).into(), MoveKind::Attack, None));
        expected.insert(Move::new((1isize, 1isize).into(), (6isize, 6isize).into(), MoveKind::Attack, None));

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
