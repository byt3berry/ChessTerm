use std::panic;

use anyhow::Result;

use game::board::board_builder::BoardBuilder;
use game::board::color::Color;
use game::board::Board;
use game::pieces::king::King;
use game::pieces::piece_kind::PieceKind;
use game::pieces::rook::Rook;
use game::ChessEngine;
use ui::cursor::Cursor;
use ui::cursor::cursor_event::CursorEvent;
use ui::drawer::{clean_screen, draw_game};

mod bot;
mod game;
mod ui;

fn main() -> Result<()> {
    panic::set_hook(Box::new(|p| {
        let _ = Cursor::stop();
        panic!("{p}");
    }));

    let mut chess_game: ChessEngine = ChessEngine::new();
    // let board: Board = BoardBuilder::new()
    //     .add(PieceKind::King(King::new((7isize, 6isize).into(), Color::Black)))
    //     .add(PieceKind::Rook(Rook::new((1isize, 0isize).into(), Color::Black)))
    //     .add(PieceKind::Rook(Rook::new((1isize, 7isize).into(), Color::Black)))
    //     .add(PieceKind::King(King::new((0isize, 6isize).into(), Color::White)))
    //     .add(PieceKind::Rook(Rook::new((0isize, 5isize).into(), Color::White)))
    //     .build();
    // let mut chess_game: ChessEngine = ChessEngine::from_board(board, Color::Black);
    let mut cursor: Cursor = Cursor::new();

    clean_screen();
    Cursor::start()?;
    // draw_game(&chess_game, &cursor);

    loop {
        cursor.next_event(&mut chess_game);
        // draw_game(&chess_game, &cursor);
        
        println!("event: {:?}", cursor.event());

        if CursorEvent::Stop.eq(cursor.event()) || chess_game.is_end() {
            break;
        }
    }

    Cursor::stop()?;

    Ok(())
}
