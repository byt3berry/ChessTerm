use anyhow::{anyhow, bail, Result};

use crate::game::board::Board;
use crate::game::board::board_builder::BoardBuilder;
use crate::game::board::color::Color;
use crate::game::board::position::Position;
use crate::game::pieces::Piece;
use crate::game::pieces::bishop::Bishop;
use crate::game::pieces::king::King;
use crate::game::pieces::knight::Knight;
use crate::game::pieces::pawn::Pawn;
use crate::game::pieces::piece_kind::PieceKind;
use crate::game::pieces::queen::Queen;
use crate::game::pieces::rook::Rook;

use super::ChessEngine;

pub struct FenParser;

impl FenParser {
    pub(crate) fn parse(fen: &str) -> Result<ChessEngine> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        assert_eq!(6, parts.len());

        let starting_player: Color = FenParser::part2(parts[1])?;
        let en_passantable: Option<Position> = FenParser::part4(parts[3]);

        let mut board: Board = FenParser::part1(parts[0], en_passantable)?;
        FenParser::part3(parts[2], &mut board)?;

        Ok(ChessEngine::from_board(board, starting_player))
    }

    fn part1(part: &str, en_passantable: Option<Position>) -> Result<Board> {
        let mut builder: BoardBuilder = BoardBuilder::new();
        let mut row: usize = 0;
        let mut column: usize = 0;

        for part in part.splitn(8, '/') {
            for c in part.chars() {
                let position: Position = (row, column).into();

                match c {
                    'B' => {
                        builder.add(PieceKind::Bishop(Bishop::new(position, Color::White)));
                        column += 1;
                    } 
                    'b' => {
                        builder.add(PieceKind::Bishop(Bishop::new(position, Color::Black)));
                        column += 1;
                    } 
                    'K' => {
                        builder.add(PieceKind::King(King::new(position, Color::White)));
                        column += 1;
                    } 
                    'k' => {
                        builder.add(PieceKind::King(King::new(position, Color::Black)));
                        column += 1;
                    } 
                    'N' => {
                        builder.add(PieceKind::Knight(Knight::new(position, Color::White)));
                        column += 1;
                    } 
                    'n' => {
                        builder.add(PieceKind::Knight(Knight::new(position, Color::Black)));
                        column += 1;
                    } 
                    'P' => {
                        let mut piece: Pawn = Pawn::new(position, Color::White);
                        let direction = piece.direction();

                        if Some(position - direction) == en_passantable {
                            piece.set_en_passant_possible();
                        }
                        builder.add(PieceKind::Pawn(piece));
                        column += 1;
                    } 
                    'p' => {
                        let mut piece: Pawn = Pawn::new(position, Color::Black);
                        let direction = piece.direction();

                        if Some(position - direction) == en_passantable {
                            piece.set_en_passant_possible();
                        }
                        builder.add(PieceKind::Pawn(piece));
                        column += 1;
                    } 
                    'Q' => {
                        builder.add(PieceKind::Queen(Queen::new(position, Color::White)));
                        column += 1;
                    } 
                    'q' => {
                        builder.add(PieceKind::Queen(Queen::new(position, Color::Black)));
                        column += 1;
                    } 
                    'R' => {
                        builder.add(PieceKind::Rook(Rook::new(position, Color::White)));
                        column += 1;
                    } 
                    'r' => {
                        builder.add(PieceKind::Rook(Rook::new(position, Color::Black)));
                        column += 1;
                    } 
                    '0'..='8' => {
                        let parsed: usize = c
                            .to_digit(10)
                            .ok_or(anyhow!("\'{c}\' should be parseable"))? as usize;
                        column += parsed;
                    }
                    _ => bail!("Unknown character in FEN part1: \'{c}\'"),
                }
            }

            if column != 8 {
                bail!("Invalid rows count: {column}");
            }
            row += 1;
            column = 0;
        }

        if row != 8 {
            bail!("Invalid rows count: {row}");
        }

        Ok(builder.build())
    }

    fn part2(part: &str) -> Result<Color> {
        assert_eq!(1, part.len());

        match part.chars().next().expect("Unknown characters in FEN part2") {
            'w' => Ok(Color::White),
            'b' => Ok(Color::Black),
            c => bail!("Unknown character in FEN part2: \'{c}\'"),
        }
    }

    fn part3(part: &str, board: &mut Board) -> Result<()> {
        let mut no_castle: bool = false;
        if part == "-" {
            no_castle = true;
        }

        let castles: Vec<char> = part.chars().collect();
        println!("castles: {castles:?}");

        if no_castle || !castles.contains(&'K') {
            if let Some(PieceKind::Rook(rook)) = board.king_side_rook_mut(Color::White) {
                println!("No king side castle for white");
                rook.set_has_moved();
            };
        }

        if no_castle || !castles.contains(&'k') {
            if let Some(PieceKind::Rook(rook)) = board.king_side_rook_mut(Color::Black) {
                println!("No king side castle for black");
                rook.set_has_moved();
            };
        }

        if no_castle || !castles.contains(&'Q') {
            if let Some(PieceKind::Rook(rook)) = board.queen_side_rook_mut(Color::White) {
                println!("No queen side castle for white");
                rook.set_has_moved();
            };
        }

        if no_castle || !castles.contains(&'q') {
            if let Some(PieceKind::Rook(rook)) = board.queen_side_rook_mut(Color::Black) {
                println!("No queen side castle for black");
                rook.set_has_moved();
            };
        }

        Ok(())
    }

    fn part4(part: &str) -> Option<Position> {
        if part == "-" {
            None
        } else {
            Position::from_notation(part)
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    use crate::game::ChessEngine;
    use crate::game::board::Board;
    use crate::game::board::board_builder::BoardBuilder;
    use crate::game::board::color::Color;
    use crate::game::pieces::Piece;
    use crate::game::pieces::piece_kind::PieceKind;
    use crate::game::pieces::bishop::Bishop;
    use crate::game::pieces::king::King;
    use crate::game::pieces::knight::Knight;
    use crate::game::pieces::pawn::Pawn;
    use crate::game::pieces::queen::Queen;
    use crate::game::pieces::rook::Rook;

    use super::FenParser;

    #[test]
    fn test_from_fen_1() -> Result<()> {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let expected_board: Board = Board::init();
        let expected: ChessEngine = ChessEngine::from_board(expected_board, Color::White);

        let chess_game: ChessEngine = FenParser::parse(fen)?;

        assert_eq!(expected, chess_game);
        Ok(())
    }

    #[test]
    fn test_from_fen_2() -> Result<()> {
        let fen: &str = "r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1 b - - 1 1";
        let expected_board: Board = BoardBuilder::new()
            .with(PieceKind::Bishop(Bishop::new((0isize, 2isize).into(), Color::Black)))
            .with(PieceKind::Bishop(Bishop::new((1isize, 4isize).into(), Color::White)))
            .with(PieceKind::Bishop(Bishop::new((7isize, 6isize).into(), Color::Black)))
            .with(PieceKind::King(King::new((0isize, 3isize).into(), Color::Black)))
            .with(PieceKind::King(King::new((6isize, 4isize).into(), Color::White)))
            .with(PieceKind::Knight(Knight::new((1isize, 6isize).into(), Color::White)))
            .with(PieceKind::Knight(Knight::new((2isize, 0isize).into(), Color::Black)))
            .with(PieceKind::Knight(Knight::new((2isize, 5isize).into(), Color::Black)))
            .with(PieceKind::Knight(Knight::new((3isize, 3isize).into(), Color::White)))
            .with(PieceKind::Pawn(Pawn::new((1isize, 0isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((1isize, 3isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((1isize, 5isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((1isize, 7isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((3isize, 1isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((3isize, 4isize).into(), Color::White)))
            .with(PieceKind::Pawn(Pawn::new((3isize, 7isize).into(), Color::White)))
            .with(PieceKind::Pawn(Pawn::new((4isize, 6isize).into(), Color::White)))
            .with(PieceKind::Pawn(Pawn::new((5isize, 3isize).into(), Color::White)))
            .with(PieceKind::Pawn(Pawn::new((6isize, 0isize).into(), Color::White)))
            .with(PieceKind::Pawn(Pawn::new((6isize, 2isize).into(), Color::White)))
            .with(PieceKind::Queen(Queen::new((7isize, 0isize).into(), Color::Black)))
            .with(PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black).with_has_moved()))
            .with(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black).with_has_moved()))
            .build();
        let expected: ChessEngine = ChessEngine::from_board(expected_board, Color::Black);

        let chess_game: ChessEngine = FenParser::parse(fen)?;

        assert_eq!(expected, chess_game);
        Ok(())
    }

    #[test]
    fn test_from_fen_3() -> Result<()> {
        let fen: &str = "8/8/8/4p1K1/2k1P3/8/8/8 b - - 0 1";
        let expected_board: Board = BoardBuilder::new()
            .with(PieceKind::King(King::new((3isize, 6isize).into(), Color::White)))
            .with(PieceKind::King(King::new((4isize, 2isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((3isize, 4isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::White)))
            .build();
        let expected: ChessEngine = ChessEngine::from_board(expected_board, Color::Black);

        let chess_game: ChessEngine = FenParser::parse(fen)?;

        assert_eq!(expected, chess_game);
        Ok(())
    }

    #[test]
    fn test_from_fen_4() -> Result<()> {
        let fen: &str = "4k2r/6r1/8/8/8/8/3R4/R3K3 w Qk - 0 1";
        let expected_board: Board = BoardBuilder::new()
            .with(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .with(PieceKind::King(King::new((7isize, 4isize).into(), Color::White)))
            .with(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black)))
            .with(PieceKind::Rook(Rook::new((1isize, 6isize).into(), Color::Black).with_has_moved()))
            .with(PieceKind::Rook(Rook::new((6isize, 3isize).into(), Color::White).with_has_moved()))
            .with(PieceKind::Rook(Rook::new((7isize, 0isize).into(), Color::White)))
            .build();
        let expected: ChessEngine = ChessEngine::from_board(expected_board, Color::White);

        let chess_game: ChessEngine = FenParser::parse(fen)?;

        assert_eq!(expected, chess_game);
        Ok(())
    }

    #[test]
    fn test_from_fen_5() -> Result<()> {
        let fen: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let expected_board: Board = BoardBuilder::new()
            .with(PieceKind::Bishop(Bishop::new((0isize, 2isize).into(), Color::Black)))
            .with(PieceKind::Bishop(Bishop::new((0isize, 5isize).into(), Color::Black)))
            .with(PieceKind::Bishop(Bishop::new((7isize, 2isize).into(), Color::White)))
            .with(PieceKind::Bishop(Bishop::new((7isize, 5isize).into(), Color::White)))
            .with(PieceKind::King(King::new((0isize, 4isize).into(), Color::Black)))
            .with(PieceKind::King(King::new((7isize, 4isize).into(), Color::White)))
            .with(PieceKind::Knight(Knight::new((0isize, 1isize).into(), Color::Black)))
            .with(PieceKind::Knight(Knight::new((0isize, 6isize).into(), Color::Black)))
            .with(PieceKind::Knight(Knight::new((7isize, 1isize).into(), Color::White)))
            .with(PieceKind::Knight(Knight::new((7isize, 6isize).into(), Color::White)))
            .with(PieceKind::Pawn(Pawn::new((1isize, 0isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((1isize, 1isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((1isize, 2isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((1isize, 3isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((1isize, 4isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((1isize, 5isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((1isize, 6isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((1isize, 7isize).into(), Color::Black)))
            .with(PieceKind::Pawn(Pawn::new((4isize, 4isize).into(), Color::White).with_en_passant_possible()))
            .with(PieceKind::Pawn(Pawn::new((6isize, 0isize).into(), Color::White)))
            .with(PieceKind::Pawn(Pawn::new((6isize, 1isize).into(), Color::White)))
            .with(PieceKind::Pawn(Pawn::new((6isize, 2isize).into(), Color::White)))
            .with(PieceKind::Pawn(Pawn::new((6isize, 3isize).into(), Color::White)))
            .with(PieceKind::Pawn(Pawn::new((6isize, 5isize).into(), Color::White)))
            .with(PieceKind::Pawn(Pawn::new((6isize, 6isize).into(), Color::White)))
            .with(PieceKind::Pawn(Pawn::new((6isize, 7isize).into(), Color::White)))
            .with(PieceKind::Queen(Queen::new((0isize, 3isize).into(), Color::Black)))
            .with(PieceKind::Queen(Queen::new((7isize, 3isize).into(), Color::White)))
            .with(PieceKind::Rook(Rook::new((0isize, 0isize).into(), Color::Black)))
            .with(PieceKind::Rook(Rook::new((0isize, 7isize).into(), Color::Black)))
            .with(PieceKind::Rook(Rook::new((7isize, 0isize).into(), Color::White)))
            .with(PieceKind::Rook(Rook::new((7isize, 7isize).into(), Color::White)))
            .build();
        let expected: ChessEngine = ChessEngine::from_board(expected_board, Color::Black);

        let chess_game: ChessEngine = FenParser::parse(fen)?;

        assert_eq!(expected, chess_game);
        Ok(())
    }
}
