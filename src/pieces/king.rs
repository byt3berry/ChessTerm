use std::collections::HashSet;

use crate::board::Board;
use crate::board::position::Position;
use crate::pieces::MoveKind;
use crate::pieces::rook::Rook;
use crate::pieces::{Color, Move, Piece, PieceKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct King {
    color: Color,
    position: Position,
    has_moved: bool,
}

impl King {
    pub const fn set_has_moved(&mut self) {
        self.has_moved = true;
    }

    fn queen_side_final_position(&self) -> Position {
        (self.position.row(), 2usize).into()
    }

    fn queen_side_rook<'a>(&self, board: &'a Board) -> Option<&'a Rook> {
        let position: Position = (self.position.row(), 0usize).into();

        if let Some(square) = board.square(position) {
            if let Some(PieceKind::ROOK(rook)) = square.piece() {
                if rook.color() == self.color() {
                    return Some(rook);
                }
            }
        };

        None
    }

    fn king_side_final_position(&self) -> Position {
        (self.position.row(), 6usize).into()
    }

    fn king_side_rook<'a>(&self, board: &'a Board) -> Option<&'a Rook> {
        let position: Position = (self.position.row(), 7usize).into();

        if let Some(square) = board.square(position) {
            if let Some(PieceKind::ROOK(rook)) = square.piece() {
                if rook.color() == self.color() {
                    return Some(rook);
                }
            }
        };

        None
    }
}

impl Piece for King {
    fn new(position: Position, color: Color) -> Self {
        Self {
            color,
            position,
            has_moved: false,
        }
    }

    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> &Position {
        &self.position
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
        let mut output: HashSet<Move> = HashSet::new();
        let mut new_position: Position;
        let mut offset: (isize, isize);
        let mut castle_final_position: Position;

        for offset in OFFSETS {
            new_position = self.position + offset;

            let Some(square) = board.square(new_position) else {
                continue;
            };
            if let Some(piece) = square.piece() {
                if piece.color() != self.color() {
                    output.insert(Move::new(self.position, new_position, MoveKind::Capture));
                }
                continue;
            }

            output.insert(Move::new(self.position, new_position, MoveKind::Move));
        }

        if self.has_moved {
            return output;
        }

        castle_final_position = self.queen_side_final_position();
        offset = (0isize, -1isize);
        new_position = self.position;

        if let Some(rook) = self.queen_side_rook(board) {
            if !rook.has_moved() {
                loop {
                    new_position = new_position + offset;

                    let Some(square) = board.square(new_position) else {
                        break;
                    };
                    if square.piece().is_some() {
                        break;
                    }

                    if new_position == castle_final_position {
                        output.insert(Move::new(self.position, new_position, MoveKind::CastleQueenSide));
                    }
                }
            }
        };

        castle_final_position = self.king_side_final_position();
        offset = (0isize, 1isize);
        new_position = self.position;

        if let Some(rook) = self.king_side_rook(board) {
            if !rook.has_moved() {
                loop {
                    new_position = new_position + offset;

                    let Some(square) = board.square(new_position) else {
                        break;
                    };
                    if square.piece().is_some() {
                        break;
                    }

                    if new_position == castle_final_position {
                        output.insert(Move::new(self.position, new_position, MoveKind::CastleKingSide));
                    }
                }
            }
        };

        output
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::board::board_builder::BoardBuilder;
    use crate::board::position::Position;
    use crate::board::Board;
    use crate::pieces::bishop::Bishop;
    use crate::pieces::pawn::Pawn;
    use crate::pieces::queen::Queen;
    use crate::pieces::rook::Rook;
    use crate::pieces::{Color, Move, MoveKind, Piece, PieceKind};

    use super::King;

    #[test]
    fn test_simple_moves() {
        let position: Position = (3usize, 3usize).into();
        let color: Color = Color::BLACK;
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new(position, color)))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new(position, (2usize, 2usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (2usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (2usize, 4usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (3usize, 2usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (3usize, 4usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (4usize, 2usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (4usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (4usize, 4usize).into(), MoveKind::Move));

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
        let position: Position = (3usize, 3usize).into();
        let color: Color = Color::BLACK;
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new(position, color)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 2isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 3isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 4isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((3isize, 2isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((3isize, 4isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 2isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 3isize).into(), color)))
            .add(PieceKind::PAWN(Pawn::new((4isize, 4isize).into(), color)))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
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
        let position: Position = (3usize, 3usize).into();
        let color: Color = Color::BLACK;
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new(position, color)))
            .add(PieceKind::PAWN(Pawn::new((2isize, 2isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((2isize, 3isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((2isize, 4isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((3isize, 2isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((3isize, 4isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((4isize, 2isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((4isize, 3isize).into(), color.other())))
            .add(PieceKind::PAWN(Pawn::new((4isize, 4isize).into(), color.other())))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new(position, (2isize, 2isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (2isize, 3isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (2isize, 4isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (3isize, 2isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (3isize, 4isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (4isize, 2isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (4isize, 3isize).into(), MoveKind::Capture));
        expected.insert(Move::new(position, (4isize, 4isize).into(), MoveKind::Capture));

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
    fn test_castle_king_side() {
        let position: Position = (0usize, 4usize).into();
        let color: Color = Color::BLACK;
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new(position, color)))
            .add(PieceKind::QUEEN(Queen::new((0isize, 3isize).into(), color)))
            .add(PieceKind::ROOK(Rook::new((0isize, 0isize).into(), color)))
            .add(PieceKind::ROOK(Rook::new((0isize, 7isize).into(), color)))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new(position, (0usize, 5usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (0usize, 6usize).into(), MoveKind::CastleKingSide));
        expected.insert(Move::new(position, (1usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (1usize, 4usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (1usize, 5usize).into(), MoveKind::Move));

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
    fn test_castle_queen_side() {
        let position: Position = (0usize, 4usize).into();
        let color: Color = Color::BLACK;
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new(position, color)))
            .add(PieceKind::BISHOP(Bishop::new((0isize, 5isize).into(), color)))
            .add(PieceKind::ROOK(Rook::new((0isize, 0isize).into(), color)))
            .add(PieceKind::ROOK(Rook::new((0isize, 7isize).into(), color)))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new(position, (0usize, 2usize).into(), MoveKind::CastleQueenSide));
        expected.insert(Move::new(position, (0usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (1usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (1usize, 4usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (1usize, 5usize).into(), MoveKind::Move));

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
    fn test_castle_king_moved() {
        let position: Position = (0usize, 4usize).into();
        let color: Color = Color::BLACK;
        let mut king: King = King::new(position, color);
        king.set_has_moved();
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(king))
            .add(PieceKind::QUEEN(Queen::new((0isize, 3isize).into(), color)))
            .add(PieceKind::ROOK(Rook::new((0isize, 0isize).into(), color)))
            .add(PieceKind::ROOK(Rook::new((0isize, 7isize).into(), color)))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new(position, (0usize, 5usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (1usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (1usize, 4usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (1usize, 5usize).into(), MoveKind::Move));

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
    fn test_castle_rook_moved() {
        let position: Position = (0usize, 4usize).into();
        let color: Color = Color::BLACK;
        let mut queen_side_rook: Rook = Rook::new((0isize, 0isize).into(), color);
        let mut king_side_rook: Rook = Rook::new((0isize, 7isize).into(), color);
        queen_side_rook.set_has_moved();
        king_side_rook.set_has_moved();
        let board: Board = BoardBuilder::new()
            .add(PieceKind::KING(King::new(position, color)))
            .add(PieceKind::QUEEN(Queen::new((0isize, 3isize).into(), color)))
            .add(PieceKind::ROOK(queen_side_rook))
            .add(PieceKind::ROOK(king_side_rook))
            .build();
        let piece: &PieceKind = board
            .piece(position)
            .expect("The piece {position} should exist");
        let mut expected: HashSet<Move> = HashSet::new();
        expected.insert(Move::new(position, (0usize, 5usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (1usize, 3usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (1usize, 4usize).into(), MoveKind::Move));
        expected.insert(Move::new(position, (1usize, 5usize).into(), MoveKind::Move));

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
