use std::collections::HashSet;

use header::{HeaderColumn, HeaderRow};

use crate::game::ChessEngine;
use crate::game::board::color::Color;
use crate::game::board::position::Position;
use crate::game::board::square::Square;
use crate::game::board::{COLUMNS, ROWS};

use super::cursor::Cursor;

mod header;
mod pieces;
mod square;

const ATTACKED_COLOR: u8 = 42u8;
const CHECKED_COLOR: u8 = 196u8;
const CURSOR_COLOR: u8 = 69u8;
const PIECE_BLACK: u8 = 235u8;
const PIECE_WHITE: u8 = 240u8;
const SQUARE_BLACK: u8 = 0u8;
const SQUARE_WHITE: u8 = 255u8;
const HEADER_BACKGROUND: u8 = 232u8;
const HEADER_FOREGROUND: u8 = 255u8;
pub(super) const SQUARE_SIZE: usize = 20usize;

const CLEAN: &str = "\x1b[2J";
const RESET: &str = "\x1b[0m";

trait Drawable {
    fn drawing(&self) -> [u8; SQUARE_SIZE*SQUARE_SIZE];
}

pub(crate) fn draw_headers() {
    let mut output: String = String::new();
    let background_color: String = terminal_color(HEADER_BACKGROUND);
    let foreground_color: String = terminal_color(HEADER_FOREGROUND);
    let (mut row, mut column): (usize, usize);
    (row, column) = (0, COLUMNS * SQUARE_SIZE * 2);

    for header in HeaderRow::values().iter().rev() {
        let drawing = header.drawing();

        for i in 0..SQUARE_SIZE {
            output += &goto(row+i, column);
            output += &background_color;

            for j in 0..SQUARE_SIZE {
                if drawing[i*SQUARE_SIZE+j] == 1 {
                    output += &foreground_color;
                    output += "  ";
                    output += &background_color;
                } else {
                    output += "  ";
                }
            }
            output += RESET;
        }

        row += SQUARE_SIZE;
    }

    (row, column) = (ROWS * SQUARE_SIZE, 0);
    for header in HeaderColumn::values().iter() {
        let drawing = header.drawing();

        for i in 0..SQUARE_SIZE {
            output += &goto(row+i, column);
            output += &background_color;

            for j in 0..SQUARE_SIZE {
                if drawing[i*SQUARE_SIZE+j] == 1 {
                    output += &foreground_color;
                    output += "  ";
                    output += &background_color;
                } else {
                    output += "  ";
                }
            }
            output += RESET;
        }

        column += SQUARE_SIZE * 2;
    }

    print!("{}", output);
}

pub(crate) fn draw_game(chess_game: &ChessEngine, cursor: &Cursor) {
    let possible_moves: Option<HashSet<Position>> = chess_game.possible_positions(cursor.selected());
    let checked_king: Option<Position> = chess_game.checked_king();
    let mut position: Position;

    for i in 0..ROWS {
        for j in 0..COLUMNS {
            position = (i, j).into();

            let Some(square) = chess_game.square(position) else {
                panic!("The square ({i}, {j}) should exist");
            };

            draw_square(possible_moves.as_ref(), checked_king, cursor, square, position);
        }
    }

    draw_headers();
}

fn colors(possible_moves: Option<&HashSet<Position>>, checked_king: Option<Position>, cursor: &Cursor, square: &Square, position: Position) -> (u8, u8) {
    let possible_moves = possible_moves.as_ref();
    let background_color: u8;

    if cursor.selected() == Some(position) {
        background_color = CURSOR_COLOR;
    } else if checked_king == Some(position) {
        background_color = CHECKED_COLOR;
    } else if possible_moves.as_ref().is_some_and(|moves| moves.contains(&position)) {
        background_color = ATTACKED_COLOR;
    } else {
        background_color = square_color(position);
    }

    let piece_color: u8 = piece_color(square, background_color);
    (background_color, piece_color)
}

fn draw_square(possible_moves: Option<&HashSet<Position>>, checked_king: Option<Position>, cursor: &Cursor, square: &Square, position: Position) {
    let mut output: String = String::new();
    let (row, column): (usize, usize) = (position.row()*SQUARE_SIZE, position.column()*SQUARE_SIZE*2);
    let drawing = square.drawing();
    let (background_color, piece_color) = colors(possible_moves, checked_king, cursor, square, position);
    let background_color: String = terminal_color(background_color);
    let piece_color: String = terminal_color(piece_color);

    for i in 0..SQUARE_SIZE {
        output += &goto(row+i, column);
        output += &background_color;

        for j in 0..SQUARE_SIZE {
            if drawing[i*SQUARE_SIZE+j] == 1 {
                output += &piece_color;
                output += "  ";
                output += &background_color;
            } else {
                output += "  ";
            }
        }
        output += RESET;
    }

    print!("{}", output);
}

fn square_color(position: Position) -> u8 {
    assert!(position.row() < ROWS, "position {position:?} is invalid");
    assert!(position.column() < COLUMNS, "position {position:?} is invalid");

    let (row, column): (usize, usize) = position.into();
    if row % 2 == 0 && column % 2 == 0 || row % 2 == 1 && column % 2 == 1 {
        SQUARE_WHITE
    } else {
        SQUARE_BLACK
    }
}

fn piece_color(square: &Square, background_color: u8) -> u8 {
    square
        .piece(Color::Any)
        .map_or(background_color, |piece|
            match piece.color() {
                Color::White => PIECE_WHITE,
                Color::Black => PIECE_BLACK,
                Color::Any => panic!("A piece with color \"Any\" can not be drawn")
            })
}

fn goto(row: usize, column: usize) -> String {
    // Don't ask why row+1
    // If i don't put +1, the first row is not printed
    format!( "\u{001b}[{};{}H", row+1, column)
}

fn terminal_color(color: u8) -> String {
    format!("\u{001b}[48;5;{}m", color)
}

pub(crate) fn clean_screen() {
    print!("{CLEAN}");
}
