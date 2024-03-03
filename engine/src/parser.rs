//! Load games or positions from a variety of formats.
//!
//! TODO: Implement PGN and possibly EPD

use crate::bitboard::Board;
use crate::piece::{Color, Piece};

pub const STARTING_POSITION_FEN: &'static str =
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// Parses a string in [FEN](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
/// and returns a new instance of `bitboard::Board` that represents the position if the notation is valid.
pub fn load_position_from_fen(fen: &str) -> Result<Board, String> {
    let mut file: usize = 0;
    let mut rank: usize = 7;
    let mut squares: [Option<Piece>; 64] = [None; 64];

    let piece_placement = fen
        .split_ascii_whitespace()
        .next()
        .ok_or(format!("Invalid FEN"))?;

    for symbol in piece_placement.chars() {
        if symbol == '/' {
            file = 0;
            rank -= 1;
        } else if symbol.is_digit(10) {
            file += symbol.to_digit(10).expect("Character is a digit") as usize;
        } else {
            let color = if symbol.is_uppercase() {
                Color::White
            } else {
                Color::Black
            };
            squares[rank * 8 + file] = match symbol.to_ascii_lowercase() {
                'p' => Some(Piece::Pawn(color)),
                'n' => Some(Piece::Knight(color)),
                'b' => Some(Piece::Bishop(color)),
                'r' => Some(Piece::Rook(color)),
                'q' => Some(Piece::Queen(color)),
                'k' => Some(Piece::King(color)),
                _ => return Err(format!("Unrecognized symbol '{}'", symbol)),
            };
            file += 1;
        }
    }

    Ok(Board::from_array(&squares))
}

#[test]
fn fen_parser() {
    assert_eq!(
        load_position_from_fen(STARTING_POSITION_FEN).unwrap(),
        Board::new()
    )
}
