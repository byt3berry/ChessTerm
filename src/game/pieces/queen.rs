use std::collections::HashSet;

use crate::game::board::Board;
use crate::game::board::color::Color;
use crate::game::board::move_kind::MoveKind;
use crate::game::board::move_struct::Move;
use crate::game::board::position::Position;

use super::Piece;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

    fn points(&self) -> i16 {
        9i16
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

    use super::Queen;

    #[test]
    fn test_simple_moves() {
        let board: Board = BoardBuilder::new()
            .with(PieceKind::Queen(Queen::new((3isize, 3isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (0isize, 0isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (0isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (0isize, 6isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 1isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (1isize, 5isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 2isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 4isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 0isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 1isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 5isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 6isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 7isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 2isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 4isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 1isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (5isize, 5isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (6isize, 0isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (6isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (6isize, 6isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (7isize, 3isize).into(), MoveKind::Attack(None)));
        expected.insert(Move::new((3isize, 3isize).into(), (7isize, 7isize).into(), MoveKind::Attack(None)));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_no_moves() {
        let board: Board = BoardBuilder::new()
            .with(PieceKind::Queen(Queen::new((3isize, 3isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((2isize, 2isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((2isize, 3isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((2isize, 4isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((3isize, 2isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((3isize, 4isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((4isize, 2isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::Black)))
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let expected: HashSet<Move> = HashSet::new();

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }

    #[test]
    fn test_capture() {
        let pawn1: PieceKind = PieceKind::Pawn(Pawn::new((2isize, 2isize).into(), Color::White));
        let pawn2: PieceKind = PieceKind::Pawn(Pawn::new((2isize, 3isize).into(), Color::White));
        let pawn3: PieceKind = PieceKind::Pawn(Pawn::new((2isize, 4isize).into(), Color::White));
        let pawn4: PieceKind = PieceKind::Pawn(Pawn::new((3isize, 2isize).into(), Color::White));
        let pawn5: PieceKind = PieceKind::Pawn(Pawn::new((3isize, 4isize).into(), Color::White));
        let pawn6: PieceKind = PieceKind::Pawn(Pawn::new((4isize, 2isize).into(), Color::White));
        let pawn7: PieceKind = PieceKind::Pawn(Pawn::new((4isize, 3isize).into(), Color::White));
        let pawn8: PieceKind = PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::White));
        let board: Board = BoardBuilder::new()
            .with(PieceKind::Queen(Queen::new((3isize, 3isize).into(), Color::Black)))
            .with(pawn1)
            .with(pawn2)
            .with(pawn3)
            .with(pawn4)
            .with(pawn5)
            .with(pawn6)
            .with(pawn7)
            .with(pawn8)
            .build();
        let piece: &PieceKind = board
            .piece((3isize, 3isize).into(), Color::Black)
            .expect("The piece should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 2isize).into(), MoveKind::Attack(Some(pawn1))));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 3isize).into(), MoveKind::Attack(Some(pawn2))));
        expected.insert(Move::new((3isize, 3isize).into(), (2isize, 4isize).into(), MoveKind::Attack(Some(pawn3))));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 2isize).into(), MoveKind::Attack(Some(pawn4))));
        expected.insert(Move::new((3isize, 3isize).into(), (3isize, 4isize).into(), MoveKind::Attack(Some(pawn5))));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 2isize).into(), MoveKind::Attack(Some(pawn6))));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 3isize).into(), MoveKind::Attack(Some(pawn7))));
        expected.insert(Move::new((3isize, 3isize).into(), (4isize, 4isize).into(), MoveKind::Attack(Some(pawn8))));

        let possible_moves: HashSet<Move> = piece.possible_moves(&board);

        assert_eq!(expected, possible_moves);
    }
}
