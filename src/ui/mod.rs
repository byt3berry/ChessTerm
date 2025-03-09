use anyhow::Result;
use cursor::Cursor;
use cursor::cursor_event::CursorEvent;
use drawer::{clean_screen, draw_game};

use crate::game::ChessEngine;

mod drawer;
mod cursor;

pub fn play() -> Result<()> {
    let mut chess_game: ChessEngine = ChessEngine::new();
    let mut cursor: Cursor = Cursor::new();

    clean_screen();
    Cursor::start()?;

    loop {
        draw_game(&chess_game, &cursor);
        cursor.next_event(&mut chess_game);
        chess_game.set_possible_moves(cursor.selected());

        if CursorEvent::Stop.eq(cursor.event()){
            break;
        }
    }

    Cursor::stop()?;

    print!("\r");
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!("{:?}", cursor.event());

    Ok(())
}
