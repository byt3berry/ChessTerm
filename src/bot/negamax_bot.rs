use std::collections::HashSet;

use crate::game::{ChessEngine, Result};
use crate::game::board::color::Color;
use crate::game::board::position::Position;

struct NegaMaxBot {
    chess_game: ChessEngine,
}

impl NegaMaxBot {
    pub fn new(chess_game: ChessEngine) -> Self {
        Self {
            chess_game,
        }
    }

    pub fn run(&mut self, depth: i16) -> (Position, Position) {
        if let (_, Some(predicted)) = self.negamax(1, depth) {
            return predicted;
        }

        todo!("no no no");
    }

    fn negamax(&mut self, color_score: i16, depth: i16) -> (i16, Option<(Position, Position)>) {
        if depth == 0 {
            return (self.pieces_score() * color_score, None);
        }

        let mut max: (i16, Option<(Position, Position)>) = (-100, None);
        let mut score: (i16, Option<(Position, Position)>);
        let possible_moves: HashSet<(Position, Position)> = self
            .chess_game
            .possible_moves()
            .iter()
            .flat_map(|(_, moves)| moves)
            .map(|m| (m.from(), m.to()))
            .collect();

        for (from, to) in possible_moves {
            self.chess_game.try_move(Some(from), Some(to));

            println!("depth: {depth}: ({from:?}, {to:?}) ({:?})", self.chess_game.result());
            score = match self.chess_game.result() {
                Result::Checkmate => return (100 * color_score, Some((from, to))),
                Result::Draw | Result::Stalemate => (0, Some((from, to))),
                _ => self.negamax(-color_score, depth-1),
            };

            score.0 *= -1;
            println!("depth: {depth}: {score:?} ({max:?})");

            if score.0 > max.0 {
                max = (score.0, Some((from, to)));
            }

            self.chess_game.undo_move();
        }

        max
    }

    fn pieces_score(&self) -> i16 {
        let white_sum = self.chess_game.points(Color::White);
        let black_sum = self.chess_game.points(Color::Black);

        if self.chess_game.current_player() == Color::White {
            black_sum - white_sum
        } else {
            white_sum - black_sum
        }
    }
}

// https://s1.static-clubeo.com/uploads/roirouge/Medias/Mats_1_%20coup_page1__o2750y.gif
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use pretty_assertions::assert_eq;

    use crate::game::ChessEngine;
    use crate::game::board::Board;
    use crate::game::board::board_builder::BoardBuilder;
    use crate::game::board::color::Color;
    use crate::game::board::position::Position;
    use crate::game::pieces::Piece;
    use crate::game::pieces::bishop::Bishop;
    use crate::game::pieces::king::King;
    use crate::game::pieces::pawn::Pawn;
    use crate::game::pieces::piece_kind::PieceKind;
    use crate::game::pieces::queen::Queen;
    use crate::game::pieces::rook::Rook;

    use super::NegaMaxBot;

    #[test]
    fn test_negamax_depth_1_checkmate_1() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((7isize, 6isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((1isize, 0isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((1isize, 7isize).into(), Color::White)))
            .add(PieceKind::King(King::new((0isize, 6isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 5isize).into(), Color::Black)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::White);
        let mut bot: NegaMaxBot = NegaMaxBot::new(chess_game);
        let expected: (Position, Position) = ((1isize, 0isize).into(), (1isize, 6isize).into());

        let predicted_move: (Position, Position) = bot.run(1);

        assert_eq!(expected, predicted_move);
    }

    #[test]
    fn test_negamax_depth_1_checkmate_2() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((7isize, 6isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((2isize, 0isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((3isize, 6isize).into(), Color::White)))
            .add(PieceKind::King(King::new((0isize, 7isize).into(), Color::Black)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::White);
        let mut bot: NegaMaxBot = NegaMaxBot::new(chess_game);
        let expected: (Position, Position) = ((2isize, 0isize).into(), (2isize, 7isize).into());

        let predicted_move: (Position, Position) = bot.run(1);

        assert_eq!(expected, predicted_move);
    }

    #[test]
    fn test_negamax_depth_1_checkmate_3() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((7isize, 6isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((7isize, 4isize).into(), Color::White)))
            .add(PieceKind::King(King::new((0isize, 6isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 6isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 7isize).into(), Color::Black)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::White);
        let mut bot: NegaMaxBot = NegaMaxBot::new(chess_game);
        let expected: (Position, Position) = ((7isize, 4isize).into(), (0isize, 4isize).into());

        let predicted_move: (Position, Position) = bot.run(1);

        assert_eq!(expected, predicted_move);
    }

    #[test]
    fn test_negamax_depth_1_checkmate_4() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((0isize, 4isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 6isize).into(), Color::White)))
            .add(PieceKind::King(King::new((0isize, 6isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 6isize).into(), Color::Black)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::White);
        let mut bot: NegaMaxBot = NegaMaxBot::new(chess_game);
        let mut expected: HashSet<(Position, Position)> = HashSet::new();
        expected.insert(((0isize, 4isize).into(), (1isize, 3isize).into()));
        expected.insert(((0isize, 4isize).into(), (1isize, 4isize).into()));
        expected.insert(((0isize, 4isize).into(), (0isize, 2isize).into()));

        let predicted_move: (Position, Position) = bot.run(1);

        assert!(
            expected.contains(&predicted_move),
            "expected:\n{expected:?}\npredicted: {predicted_move:?}"
            );
    }

    #[test]
    fn test_negamax_depth_1_checkmate_5() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((6isize, 6isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((2isize, 2isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 6isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 7isize).into(), Color::White)))
            .add(PieceKind::King(King::new((0isize, 6isize).into(), Color::Black)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::White);
        let mut bot: NegaMaxBot = NegaMaxBot::new(chess_game);
        let expected: (Position, Position) = ((2isize, 2isize).into(), (0isize, 2isize).into());

        let predicted_move: (Position, Position) = bot.run(1);

        assert_eq!(expected, predicted_move);
    }

    #[test]
    fn test_negamax_depth_1_checkmate_6() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((6isize, 6isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((2isize, 2isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 6isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((2isize, 7isize).into(), Color::White)))
            .add(PieceKind::King(King::new((0isize, 6isize).into(), Color::Black)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::White);
        let mut bot: NegaMaxBot = NegaMaxBot::new(chess_game);
        let expected: (Position, Position) = ((2isize, 2isize).into(), (0isize, 2isize).into());

        let predicted_move: (Position, Position) = bot.run(1);

        assert_eq!(expected, predicted_move);
    }

    #[test]
    fn test_negamax_depth_1_checkmate_7() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((6isize, 6isize).into(), Color::White)))
            .add(PieceKind::Queen(Queen::new((5isize, 5isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 7isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 7isize).into(), Color::Black)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::White);
        let mut bot: NegaMaxBot = NegaMaxBot::new(chess_game);
        let expected: (Position, Position) = ((5isize, 5isize).into(), (0isize, 5isize).into());

        let predicted_move: (Position, Position) = bot.run(1);

        assert_eq!(expected, predicted_move);
    }

    #[test]
    fn test_negamax_depth_1_checkmate_8() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((7isize, 6isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((7isize, 5isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((7isize, 7isize).into(), Color::White)))
            .add(PieceKind::King(King::new((1isize, 6isize).into(), Color::Black)))
            .add(PieceKind::Queen(Queen::new((2isize, 3isize).into(), Color::Black)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let mut bot: NegaMaxBot = NegaMaxBot::new(chess_game);
        let expected: (Position, Position) = ((2isize, 3isize).into(), (5isize, 6isize).into());

        let predicted_move: (Position, Position) = bot.run(1);

        assert_eq!(expected, predicted_move);
    }

    #[test]
    fn test_negamax_depth_1_checkmate_9() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((6isize, 6isize).into(), Color::White)))
            .add(PieceKind::Queen(Queen::new((4isize, 7isize).into(), Color::White)))
            .add(PieceKind::King(King::new((0isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((1isize, 6isize).into(), Color::Black)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::White);
        let mut bot: NegaMaxBot = NegaMaxBot::new(chess_game);
        let expected: (Position, Position) = ((4isize, 7isize).into(), (0isize, 3isize).into());

        let predicted_move: (Position, Position) = bot.run(1);

        assert_eq!(expected, predicted_move);
    }

    #[test]
    fn test_negamax_depth_1_checkmate_10() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((5isize, 1isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((5isize, 0isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 1isize).into(), Color::White)))
            .add(PieceKind::King(King::new((1isize, 6isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((6isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Queen(Queen::new((3isize, 5isize).into(), Color::Black)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        let mut bot: NegaMaxBot = NegaMaxBot::new(chess_game);
        let expected: (Position, Position) = ((3isize, 5isize).into(), (6isize, 2isize).into());

        let predicted_move: (Position, Position) = bot.run(1);

        assert_eq!(expected, predicted_move);
    }

    #[test]
    fn test_negamax_depth_1_checkmate_11() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((6isize, 6isize).into(), Color::White)))
            .add(PieceKind::Queen(Queen::new((2isize, 5isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((4isize, 7isize).into(), Color::White)))
            .add(PieceKind::King(King::new((0isize, 6isize).into(), Color::Black)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::White);
        let mut bot: NegaMaxBot = NegaMaxBot::new(chess_game);
        let expected: (Position, Position) = ((4isize, 7isize).into(), (0isize, 7isize).into());

        let predicted_move: (Position, Position) = bot.run(1);

        assert_eq!(expected, predicted_move);
    }

    #[test]
    fn test_negamax_depth_1_checkmate_12() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((6isize, 6isize).into(), Color::White)))
            .add(PieceKind::Queen(Queen::new((3isize, 7isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((3isize, 5isize).into(), Color::White)))
            .add(PieceKind::King(King::new((1isize, 6isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((0isize, 5isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black)))
            .build();
        let chess_game: ChessEngine = ChessEngine::from_board(board, Color::White);
        let mut bot: NegaMaxBot = NegaMaxBot::new(chess_game);
        let expected: (Position, Position) = ((3isize, 7isize).into(), (2isize, 6isize).into());

        let predicted_move: (Position, Position) = bot.run(1);

        assert_eq!(expected, predicted_move);
    }
}
