//! Contains useful enums an to identify chess pieces.

use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Piece::Pawn(color) => match color {
                Color::White => write!(f, "{{P}}"),
                Color::Black => write!(f, "(P)"),
            },
            Piece::Knight(color) => match color {
                Color::White => write!(f, "{{N}}"),
                Color::Black => write!(f, "(N)"),
            },
            Piece::Bishop(color) => match color {
                Color::White => write!(f, "{{B}}"),
                Color::Black => write!(f, "(B)"),
            },
            Piece::Rook(color) => match color {
                Color::White => write!(f, "{{R}}"),
                Color::Black => write!(f, "(R)"),
            },
            Piece::Queen(color) => match color {
                Color::White => write!(f, "{{Q}}"),
                Color::Black => write!(f, "(Q)"),
            },
            Piece::King(color) => match color {
                Color::White => write!(f, "{{K}}"),
                Color::Black => write!(f, "(K)"),
            },
        }
    }
}
