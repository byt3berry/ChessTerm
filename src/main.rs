use anyhow::Result;

use game::ChessEngine;
use ui::cursor::Cursor;
use ui::cursor::cursor_event::CursorEvent;
use ui::drawer::{clean_screen, draw_game};

mod ui;
mod game;

fn main() -> Result<()> {
    let mut chess_game: ChessEngine = ChessEngine::new();
    let mut cursor: Cursor = Cursor::new();

    clean_screen();
    Cursor::start()?;
    draw_game(&chess_game, &cursor);

    loop {
        cursor.next_event(&mut chess_game);
        draw_game(&chess_game, &cursor);

        if CursorEvent::Stop.eq(cursor.event()) || chess_game.is_end() {
            break;
        }
    }

    Cursor::stop()?;

    Ok(())
}
