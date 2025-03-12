use std::collections::HashSet;

use board::Board;
use board::color::Color;
use board::move_struct::Move;
use board::position::Position;
use board::square::Square;

pub(super) mod pieces;
pub(super) mod board;

pub struct ChessEngine {
    board: Board,
    current_player: Color,
    possible_moves: Option<HashSet<Move>>,
}

impl ChessEngine {
    pub fn new() -> Self {
        let starting_player: Color = Color::White;
        let mut board = Board::init();
        board.set_possible_moves(starting_player);
        board.set_possible_moves(starting_player.other());

        Self {
            board,
            current_player: starting_player,
            possible_moves: None,
        }
    }

    pub fn square(&self, position: Position) -> Option<&Square> {
        self.board.square(position)
    }

    pub fn try_move(&mut self, from: Option<Position>, to: Option<Position>) -> bool {
        let Some(from) = from else {
            return false;
        };
        let Some(to) = to else {
            return false;
        };
        let Some(possible_moves) = self.possible_moves.as_ref() else {
            return false;
        };
        let Some(try_move) = possible_moves
            .iter()
            .find(|m| m.from() == from && m.to() == to) else {
                return false;
            };

        self.board.make_move(try_move, self.current_player);
        self.board.set_possible_moves(self.current_player);
        self.board.set_possible_moves(self.current_player.other());
        self.current_player = self.current_player.other();
        true
    }

    pub fn checked_king(&self) -> Option<Position> {
        if self.board.checked(self.current_player) {
            self.board.king(self.current_player)
        } else {
            None
        }
    }

    pub fn set_possible_moves(&mut self, position: Option<Position>) {
        let Some(position) = position else {
            return;
        };

        if let Some(piece) = self.board.piece(position, self.current_player) {
            let mut possible_moves: HashSet<Move> = piece.possible_moves(&self.board);
            self.filter_check_block(&mut possible_moves);
            self.possible_moves = Some(possible_moves);

            return;
        }

        self.possible_moves = None;
    }

    fn filter_check_block(&self, possible_moves: &mut HashSet<Move>) {
        possible_moves
            .retain(|simulated_move| {
                let mut simulated_board: Board = self
                    .board
                    .simulate_move(simulated_move, self.current_player);
                simulated_board.set_possible_moves(self.current_player.other());
                !simulated_board
                    .checked(self.current_player)
            });
    }

    pub fn possible_positions(&self, position: Option<Position>) -> Option<HashSet<Position>> {
        let possible_moves: &HashSet<Move> = self.possible_moves.as_ref()?;
        let position = position?;

        if self.board.piece(position, self.current_player).is_some() {
            return Some(possible_moves
                .iter()
                .map(|m| m.to())
                .collect());
        }

        None
    }
}

#[cfg(test)]
impl ChessEngine {
    pub fn from_board(mut board: Board, starting_player: Color) -> Self {
        board.set_possible_moves(starting_player);
        board.set_possible_moves(starting_player.other());

        Self {
            board,
            current_player: starting_player,
            possible_moves: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::collections::HashSet;

    use crate::game::board::Board;
    use crate::game::board::board_builder::BoardBuilder;
    use crate::game::board::color::Color;
    use crate::game::board::position::Position;
    use crate::game::pieces::bishop::Bishop;
    use crate::game::pieces::king::King;
    use crate::game::pieces::knight::Knight;
    use crate::game::pieces::pawn::Pawn;
    use crate::game::pieces::piece_kind::PieceKind;
    use crate::game::pieces::queen::Queen;
    use crate::game::pieces::rook::Rook;

    use super::ChessEngine;

    #[test]
    fn test_filter_check_block() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((7isize, 4isize).into(), Color::White)))
            .add(PieceKind::Pawn(Pawn::new((6isize, 6isize).into(), Color::White)))
            .add(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .add(PieceKind::Queen(Queen::new((4isize, 6isize).into(), Color::Black)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.set_possible_moves(Some((4isize, 6isize).into()));
        chess_game.try_move(Some((4isize, 6isize).into()), Some((4isize, 7isize).into()));
        chess_game.set_possible_moves(Some((6isize, 6isize).into()));
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((5isize, 6isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((6isize, 6isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_bishop_pinned_no_move() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Bishop(Bishop::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((5isize, 3isize).into(), Color::White)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.set_possible_moves(Some((3isize, 3isize).into()));
        let expected: HashSet<Position> = HashSet::new();

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_bishop_pinned_can_move() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Bishop(Bishop::new((1isize, 1isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((6isize, 6isize).into(), Color::White)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.set_possible_moves(Some((1isize, 1isize).into()));
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((2isize, 2isize).into());
        expected.insert((3isize, 3isize).into());
        expected.insert((4isize, 4isize).into());
        expected.insert((5isize, 5isize).into());
        expected.insert((6isize, 6isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((1isize, 1isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_king_moves_attacked_square_other_color() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((2isize, 0isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((4isize, 0isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((0isize, 2isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((0isize, 4isize).into(), Color::White)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.set_possible_moves(Some((3isize, 3isize).into()));
        let expected: HashSet<Position> = HashSet::new();

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_king_moves_attacked_square_same_color() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::King(King::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((2isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((4isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 2isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((0isize, 4isize).into(), Color::Black)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.set_possible_moves(Some((3isize, 3isize).into()));
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((2isize, 2isize).into());
        expected.insert((2isize, 3isize).into());
        expected.insert((2isize, 4isize).into());
        expected.insert((3isize, 2isize).into());
        expected.insert((3isize, 4isize).into());
        expected.insert((4isize, 2isize).into());
        expected.insert((4isize, 3isize).into());
        expected.insert((4isize, 4isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_knight_pinned_no_move() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Knight(Knight::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((6isize, 6isize).into(), Color::White)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.set_possible_moves(Some((3isize, 3isize).into()));
        let expected: HashSet<Position> = HashSet::new();

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_pawn_pinned_no_move() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((6isize, 6isize).into(), Color::White)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.set_possible_moves(Some((3isize, 3isize).into()));
        let expected: HashSet<Position> = HashSet::new();

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_pawn_pinned_no_attack() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::White)))
            .add(PieceKind::Rook(Rook::new((4isize, 3isize).into(), Color::White)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.set_possible_moves(Some((3isize, 3isize).into()));
        let expected: HashSet<Position> = HashSet::new();

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_pawn_pinned_can_move() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((5isize, 3isize).into(), Color::White)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.set_possible_moves(Some((3isize, 3isize).into()));
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((4isize, 3isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_pawn_pinned_can_en_passant() {
        let mut pawn: Pawn = Pawn::new((4isize, 3isize).into(), Color::White);
        pawn.set_en_passant_possible();
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((7isize, 1isize).into(), Color::Black)))
            .add(PieceKind::Pawn(Pawn::new((5isize, 4isize).into(), Color::White)))
            .add(PieceKind::Pawn(pawn))
            .add(PieceKind::Bishop(Bishop::new((6isize, 7isize).into(), Color::White)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.set_possible_moves(Some((4isize, 4isize).into()));
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((5isize, 3isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((4isize, 4isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_queen_pinned_can_move_straight() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Queen(Queen::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((5isize, 3isize).into(), Color::White)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.set_possible_moves(Some((3isize, 3isize).into()));
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((1isize, 3isize).into());
        expected.insert((2isize, 3isize).into());
        expected.insert((4isize, 3isize).into());
        expected.insert((5isize, 3isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_queen_pinned_can_move_diagonal() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Queen(Queen::new((1isize, 1isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((6isize, 6isize).into(), Color::White)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.set_possible_moves(Some((1isize, 1isize).into()));
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((2isize, 2isize).into());
        expected.insert((3isize, 3isize).into());
        expected.insert((4isize, 4isize).into());
        expected.insert((5isize, 5isize).into());
        expected.insert((6isize, 6isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((1isize, 1isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_rook_pinned_no_move() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Rook(Rook::new((1isize, 1isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 0isize).into(), Color::Black)))
            .add(PieceKind::Bishop(Bishop::new((6isize, 6isize).into(), Color::White)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.set_possible_moves(Some((1isize, 1isize).into()));
        let expected: HashSet<Position> = HashSet::new();

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((1isize, 1isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }

    #[test]
    fn test_rook_pinned_can_move() {
        let board: Board = BoardBuilder::new()
            .add(PieceKind::Rook(Rook::new((3isize, 3isize).into(), Color::Black)))
            .add(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .add(PieceKind::Rook(Rook::new((5isize, 3isize).into(), Color::White)))
            .build();
        let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
        chess_game.set_possible_moves(Some((3isize, 3isize).into()));
        let mut expected: HashSet<Position> = HashSet::new();
        expected.insert((1isize, 3isize).into());
        expected.insert((2isize, 3isize).into());
        expected.insert((4isize, 3isize).into());
        expected.insert((5isize, 3isize).into());

        let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(Some((3isize, 3isize).into()));

        assert_eq!(Some(expected), possible_moves);
    }
}
