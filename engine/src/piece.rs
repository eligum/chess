//! Useful enums for distinguishing chess pieces.

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl Piece {
    /// Returns `true` if the piece is a slider, meaning its moves have _infinite_ range.
    pub fn is_slider(self) -> bool {
        match self {
            Piece::Bishop(_) | Piece::Rook(_) | Piece::Queen(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Piece::Pawn(color) => match color {
                Color::White => write!(f, "P"),
                Color::Black => write!(f, "p"),
            },
            Piece::Knight(color) => match color {
                Color::White => write!(f, "N"),
                Color::Black => write!(f, "n"),
            },
            Piece::Bishop(color) => match color {
                Color::White => write!(f, "B"),
                Color::Black => write!(f, "b"),
            },
            Piece::Rook(color) => match color {
                Color::White => write!(f, "R"),
                Color::Black => write!(f, "r"),
            },
            Piece::Queen(color) => match color {
                Color::White => write!(f, "Q"),
                Color::Black => write!(f, "q"),
            },
            Piece::King(color) => match color {
                Color::White => write!(f, "K"),
                Color::Black => write!(f, "k"),
            },
        }
    }
}
