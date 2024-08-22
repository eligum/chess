//! Load and store games or positions from a variety of formats.
//!
//! TODO: Implement PGN and possibly EPD

use crate::board::{Board, CastleRights};
use crate::piece::{Color, Piece};

pub const STARTING_POSITION_FEN: &'static str =
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// Parses a string in [FEN](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
/// and returns a new instance of `bitboard::Board` that represents the position if the notation is valid.
pub fn load_position_from_fen(fen: &str) -> Result<Board, String> {
    let mut file: usize = 0;
    let mut rank: usize = 7;
    let mut pieces: [Option<Piece>; 64] = [None; 64];
    let mut fen_fields = fen.split_ascii_whitespace();

    for symbol in fen_fields
        .next()
        .ok_or(format!("Missing first field of FEN"))?
        .chars()
    {
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
            pieces[rank * 8 + file] = match symbol.to_ascii_lowercase() {
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

    let color_to_move = fen_fields
        .next()
        .ok_or(format!("Missing second filed of FEN"))?;
    let color_to_move = match color_to_move {
        "w" => Color::White,
        "b" => Color::Black,
        _ => {
            return Err(format!(
                "Unexpected symbol '{}' in second field of FEN",
                color_to_move
            ))
        }
    };

    let mut castling_rights = CastleRights::None;
    for symbol in fen_fields
        .next()
        .ok_or(format!("Missing third field of FEN"))?
        .chars()
    {
        match symbol {
            '-' => break,
            'K' => castling_rights |= CastleRights::WhiteKS,
            'Q' => castling_rights |= CastleRights::WhiteQS,
            'k' => castling_rights |= CastleRights::BlackKS,
            'q' => castling_rights |= CastleRights::BlackQS,
            _ => {
                return Err(format!(
                    "Unexpected symbol '{}' in third field of FEN",
                    symbol
                ))
            }
        }
    }

    let mut en_passant_square = None;
    let mut en_passant_field = fen_fields
        .next()
        .ok_or(format!("Missing fourth field of FEN"))?
        .chars();
    if let Some(letter) = en_passant_field.next() {
        if letter != '-' {
            let file = match letter {
                'a' | 'A' => 0,
                'b' | 'B' => 1,
                'c' | 'C' => 2,
                'd' | 'D' => 3,
                'e' | 'E' => 4,
                'f' | 'F' => 5,
                'g' | 'G' => 6,
                'h' | 'H' => 7,
                _ => {
                    return Err(format!(
                        "Unknown file coordinate '{}'. Expected a, b, c, d, e, f, g or h",
                        letter
                    ))
                }
            };
            if let Some(digit) = en_passant_field.next() {
                let rank = match digit.to_digit(10) {
                    Some(x) if 1 <= x && x <= 8 => (x - 1) as usize,
                    _ => {
                        return Err(format!(
                            "Unknown rank coordinate '{}'. Expected 1, 2, 3, 4, 5, 6, 7 or 8",
                            digit
                        ))
                    }
                };
                en_passant_square = Some(rank * 8 + file);
            }
        }
    }

    let halfmove_clock = fen_fields
        .next()
        .ok_or(format!("Missing fifth field of FEN"))?
        .parse::<u32>()
        .unwrap(); // TODO: Convert parsing error to String.

    let fullmove_number = fen_fields
        .next()
        .ok_or(format!("Missing sixth field of FEN"))?
        .parse::<u32>()
        .unwrap(); // TODO: Convert parsing error to String.

    Ok(Board::from_array(
        pieces,
        castling_rights,
        color_to_move,
        en_passant_square,
    ))
}

pub fn store_position_as_fen(_board: &Board) -> Result<String, String> {
    todo!();
}

#[test]
fn fen_parser_load() {
    assert_eq!(
        load_position_from_fen(STARTING_POSITION_FEN).unwrap(),
        Board::new(),
    );
}

#[test]
#[ignore = "function not implemented yet"]
fn fen_parser_store() {
    assert_eq!(
        store_position_as_fen(&Board::new()).unwrap(),
        STARTING_POSITION_FEN,
    );
}
