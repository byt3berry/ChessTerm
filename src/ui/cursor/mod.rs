use anyhow::{Error, Result};
use crossterm::event::{read, Event, MouseEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use cursor_event::CursorEvent;
use std::iter::{self, RepeatWith};

use crate::game::board::position::Position;
use crate::game::board::{COLUMNS, ROWS};
use crate::game::ChessEngine;

use super::drawer::SQUARE_SIZE;

pub(crate) mod cursor_event;

pub(crate) struct Cursor {
    event: CursorEvent,
    event_iterator: RepeatWith<fn() -> CursorEvent>,
}

impl Cursor {
    pub(crate) fn new() -> Self {
        Self {
            event: CursorEvent::None,
            event_iterator: Self::event_iter(),
        }
    }

    pub(crate) fn event(&self) -> &CursorEvent {
        &self.event
    }

    pub(crate) fn next_event(&mut self, chess_game: &mut ChessEngine) {
        if let Some(event) = self.event_iterator.next() {
            let current_position: Option<Position> = Self::to_board_position(&self.event);
            let new_position: Option<Position> = Self::to_board_position(&event);

            if chess_game.try_move(current_position, new_position) {
                self.event = CursorEvent::None;
            } else {
                self.event = event;
            }
        } else {
            self.event = CursorEvent::None;
        }
    }

    pub(crate) fn selected(&self) -> Option<Position> {
        Self::to_board_position(&self.event)
    }

    fn to_board_position(cursor_event: &CursorEvent) -> Option<Position> {
        let (row, column) = cursor_event.position()?;
        let chess_row: usize = row as usize / SQUARE_SIZE;
        let chess_column: usize = column as usize / SQUARE_SIZE / 2;
        if chess_row > ROWS || chess_column > COLUMNS {
            return None
        }

        Some((chess_row, chess_column).into())
    }

    fn event_iter() -> RepeatWith<fn() -> CursorEvent> {
        iter::repeat_with(|| {
            loop {
                let new_event: Result<Event> = read().map_err(Error::from);

                if let Ok(Event::Mouse(event)) = new_event {
                    if let MouseEventKind::Down(_) = event.kind {
                        return CursorEvent::Event(event)
                    }
                } else if let Ok(Event::Key(_)) = new_event {
                    return CursorEvent::Stop;
                }
            }
        })
    }

    pub(crate) fn start() -> Result<()> {
        enable_raw_mode()?;
        execute!(
            std::io::stderr(),
            crossterm::event::EnableMouseCapture,
        ).map_err(Error::from)
    }

    pub(crate) fn stop() -> Result<()> {
        execute!(
            std::io::stderr(),
            crossterm::event::DisableMouseCapture,
        )?;
        disable_raw_mode().map_err(Error::from)
    }
}
