use crate::board::position::Position;
use crate::board::square::Square;
use crate::board::{Board, COLUMNS, ROWS};
use crate::pieces::Color;

mod pieces;
mod square;

const PIECE_BLACK: u8 = 235;
const PIECE_WHITE: u8 = 240;
const SQUARE_BLACK: u8 = 0;
const SQUARE_WHITE: u8 = 255;
const SQUARE_SIZE: usize = 20;

pub trait Drawable {
    fn drawing(&self) -> [u8; SQUARE_SIZE*SQUARE_SIZE];
}

pub fn draw(board: &Board) {
    let mut position: Position;
    clean();
    println!();

    for i in 0..ROWS {
        for j in 0..COLUMNS {
            position = (i, j).into();

            let Some(square) = board.square(position) else {
                panic!("The square ({i}, {j}) should exist");
            };

            draw_square(square, position);
        }
    }
}

pub fn square_color(position: Position) -> Color {
    assert!(position.row() < ROWS, "position {position:?} is invalid");
    assert!(position.column() < COLUMNS, "position {position:?} is invalid");

    let (row, column): (usize, usize) = position.into();
    if row % 2 == 0 && column % 2 == 0 || row % 2 == 1 && column % 2 == 1 {
        Color::WHITE
    } else {
        Color::BLACK
    }
}

fn draw_square(square: &Square, position: Position) {
    let (row, column): (usize, usize) = position.into();
    let drawing = square.drawing();
    let bg_color: u8 = match square_color(position) {
        Color::WHITE => SQUARE_WHITE,
        Color::BLACK => SQUARE_BLACK,
    };
    let piece_color: u8 = square
        .piece()
        .map_or(bg_color, |piece|
            match piece.color() {
                Color::WHITE => PIECE_WHITE,
                Color::BLACK => PIECE_BLACK,
            });

    for i in 0..SQUARE_SIZE {
        goto(row*SQUARE_SIZE+i, column*SQUARE_SIZE*2);
        background(bg_color);

        for j in 0..SQUARE_SIZE {
            if drawing[i*SQUARE_SIZE+j] == 1 {
                background(piece_color);
                print!("  ");
                background(bg_color);
            } else {
                print!("  ");
            }
        }
        reset();
    }
}

fn goto(row: usize, column: usize) {
    // Don't ask why row+1
    // If i don't put +1, the first row is not printed
    print!("\u{001b}[{};{}H", row+1, column);
}

fn background(color: u8) {
    print!("\u{001b}[48;5;{color}m");
}

fn clean() {
    print!("\u{001b}[2J");
}

fn reset() {
    print!("\u{001b}[0m");
}
