//! Board representation.
//!
//! Most significant bit (MSB) represents the 'a1' square while the least significant
//! bit (LSB) represents the 'h8' square.
//!
//! TODO: Expand this section.

use crate::{bits, piece::*};
use bitflags::bitflags;
use std::ops::Add;

type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    // pieces
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_kings: u64,
    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_kings: u64,
    // position metadata
    color_to_move: Color,
    castling_rights: CastleRights,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    pub index: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub origin: Square,
    pub target: Square,
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct CastleRights: u8 {
        /// None.
        const None = 0;
        /// White pieces kingside castling.
        const WhiteKS = 0x01;
        /// White pieces queenside castling.
        const WhiteQS = 0x02;
        /// Black pieces kingside castling.
        const BlackKS = 0x04;
        /// Black pieces queenside castling.
        const BlackQS = 0x08;
        /// Both colors have all castling rights.
        const All = 0x0f;
    }
}

impl Square {
    /// Constructs a `Square` from the given string interpreted as algebraic notation.
    ///
    /// The choosen board representation maps "a1" to the square with index 0 and "h8"
    /// to the one with index 63.
    pub fn from_notation(text: &str) -> Result<Self> {
        let mut cs = text.chars();
        if let Some(letter) = cs.next() {
            let col: u32 = match letter {
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
            if let Some(digit) = cs.next() {
                let row: u32 = match digit.to_digit(10) {
                    Some(x) if 1 <= x && x <= 8 => x - 1,
                    _ => {
                        return Err(format!(
                            "Unknown rank coordinate '{}'. Expected 1, 2, 3, 4, 5, 6, 7 or 8",
                            digit
                        ))
                    }
                };
                return Ok(Self {
                    index: row * 8 + col,
                });
            }
        }

        Err(format!("Failed to parse '{}' as algebraic notation", text))
    }

    /// Returns the row index (rank) of the square.
    pub fn get_rank(&self) -> u32 {
        self.index / 8
    }

    /// Returns the column index (file) of the square.
    pub fn get_file(&self) -> u32 {
        self.index % 8
    }
}

impl Move {
    /// Constructs a `Move` from a string of text.
    ///
    /// The notation used expects 4 characters: the first two, represent the square from
    /// which the piece is moving, and the last two, represent the target square.
    /// For example, "e2e4" is one of the most common opening moves for white.
    pub fn from_notation(text: &str) -> Result<Self> {
        if text.chars().count() != 4 {
            return Err(format!("Failed to parse '{}', expected 4 characters", text));
        }
        let origin = Square::from_notation(&text[0..2])?;
        let target = Square::from_notation(&text[2..4])?;
        if origin == target {
            return Err(format!("Origin and target square cannot be the same"));
        }

        Ok(Self { origin, target })
    }

    /// Constructs a `Move` from two indices.
    ///
    /// This function does not check wether the indices are within bounds of the board,
    /// so you could end up with an impossible move.
    pub fn from_indices(origin: u32, target: u32) -> Self {
        Self {
            origin: Square { index: origin },
            target: Square { index: target },
        }
    }
}

impl Board {
    /// Constructs a new `Board` with the standard piece arrangement.
    ///
    /// To understand more about the board representation read the `bitboard` module documentation.
    #[rustfmt::skip]
    pub const fn new() -> Self {
        Self {
            white_pawns:   0x00_ff_00_00_00_00_00_00,
            white_rooks:   0x81_00_00_00_00_00_00_00,
            white_knights: 0x42_00_00_00_00_00_00_00,
            white_bishops: 0x24_00_00_00_00_00_00_00,
            white_queens:  0x10_00_00_00_00_00_00_00,
            white_kings:   0x08_00_00_00_00_00_00_00,
            black_pawns:   0x00_00_00_00_00_00_ff_00,
            black_rooks:   0x00_00_00_00_00_00_00_81,
            black_knights: 0x00_00_00_00_00_00_00_42,
            black_bishops: 0x00_00_00_00_00_00_00_24,
            black_queens:  0x00_00_00_00_00_00_00_10,
            black_kings:   0x00_00_00_00_00_00_00_08,
            castling_rights: CastleRights::All,
            color_to_move: Color::White,
        }
    }

    pub fn from_array(
        array: &[Option<Piece>; 64],
        castling_rights: CastleRights,
        color_to_move: Color,
    ) -> Self {
        let mut white_pawns: u64 = 0;
        let mut white_rooks: u64 = 0;
        let mut white_knights: u64 = 0;
        let mut white_bishops: u64 = 0;
        let mut white_queens: u64 = 0;
        let mut white_kings: u64 = 0;
        let mut black_pawns: u64 = 0;
        let mut black_rooks: u64 = 0;
        let mut black_knights: u64 = 0;
        let mut black_bishops: u64 = 0;
        let mut black_queens: u64 = 0;
        let mut black_kings: u64 = 0;

        for square in array {
            white_pawns <<= 1;
            white_rooks <<= 1;
            white_knights <<= 1;
            white_bishops <<= 1;
            white_queens <<= 1;
            white_kings <<= 1;
            black_pawns <<= 1;
            black_rooks <<= 1;
            black_knights <<= 1;
            black_bishops <<= 1;
            black_queens <<= 1;
            black_kings <<= 1;
            if let Some(piece) = square {
                match piece {
                    Piece::Pawn(c) => match c {
                        Color::White => white_pawns |= 1,
                        Color::Black => black_pawns |= 1,
                    },
                    Piece::Rook(c) => match c {
                        Color::White => white_rooks |= 1,
                        Color::Black => black_rooks |= 1,
                    },
                    Piece::Knight(c) => match c {
                        Color::White => white_knights |= 1,
                        Color::Black => black_knights |= 1,
                    },
                    Piece::Bishop(c) => match c {
                        Color::White => white_bishops |= 1,
                        Color::Black => black_bishops |= 1,
                    },
                    Piece::Queen(c) => match c {
                        Color::White => white_queens |= 1,
                        Color::Black => black_queens |= 1,
                    },
                    Piece::King(c) => match c {
                        Color::White => white_kings |= 1,
                        Color::Black => black_kings |= 1,
                    },
                }
            }
        }

        Self {
            white_pawns,
            white_rooks,
            white_knights,
            white_bishops,
            white_queens,
            white_kings,
            black_pawns,
            black_rooks,
            black_knights,
            black_bishops,
            black_queens,
            black_kings,
            castling_rights,
            color_to_move,
        }
    }

    /// Returns an array of `Option<Piece>` that represents squares of the board.
    ///
    /// TODO: Implement.
    pub fn piece_array(&self) -> [Option<Piece>; 64] {
        let mut result = [None; 64];
        result
    }

    /// Returns the bitboard resulting from the union of all the piece bitboards.
    ///
    /// The occupancy refers to the set of all squares occupied by any piece. Thus, the
    /// complement of the occupancy is the set of all empty squares.
    pub fn occupancy(&self) -> u64 {
        self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_kings
            | self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_kings
    }

    /// Checks whether the internal representation of the board is in a valid state.
    ///
    /// This function is meant for debugging and the development of the `chess::engine`
    /// crate. It checks for overlapping piece bitboards and verifies the presence of
    /// only one king of each color.
    pub fn is_valid(&self) -> bool {
        // Pieces overlap
        let overlap = Some(self.white_pawns)
            .and_then(|x| x.checked_add(self.white_knights))
            .and_then(|x| x.checked_add(self.white_bishops))
            .and_then(|x| x.checked_add(self.white_rooks))
            .and_then(|x| x.checked_add(self.white_queens))
            .and_then(|x| x.checked_add(self.white_kings))
            .and_then(|x| x.checked_add(self.black_pawns))
            .and_then(|x| x.checked_add(self.black_knights))
            .and_then(|x| x.checked_add(self.black_bishops))
            .and_then(|x| x.checked_add(self.black_rooks))
            .and_then(|x| x.checked_add(self.black_queens))
            .and_then(|x| x.checked_add(self.black_kings))
            != Some(self.occupancy());
        // Only one king of each color
        let unique_kings = bits::only_one(self.white_kings) && bits::only_one(self.black_kings);

        !overlap && unique_kings
    }

    pub fn at(&self, index: usize) -> Option<Piece> {
        if index > 63 {
            return None;
        }
        let bitmask = (1 << 63) >> index;
        if bitmask & self.white_pawns > 0 {
            Some(Piece::Pawn(Color::White))
        } else if bitmask & self.white_knights > 0 {
            Some(Piece::Knight(Color::White))
        } else if bitmask & self.white_bishops > 0 {
            Some(Piece::Bishop(Color::White))
        } else if bitmask & self.white_rooks > 0 {
            Some(Piece::Rook(Color::White))
        } else if bitmask & self.white_queens > 0 {
            Some(Piece::Queen(Color::White))
        } else if bitmask & self.white_kings > 0 {
            Some(Piece::King(Color::White))
        } else if bitmask & self.black_pawns > 0 {
            Some(Piece::Pawn(Color::Black))
        } else if bitmask & self.black_knights > 0 {
            Some(Piece::Knight(Color::Black))
        } else if bitmask & self.black_bishops > 0 {
            Some(Piece::Bishop(Color::Black))
        } else if bitmask & self.black_rooks > 0 {
            Some(Piece::Rook(Color::Black))
        } else if bitmask & self.black_queens > 0 {
            Some(Piece::Queen(Color::Black))
        } else if bitmask & self.black_kings > 0 {
            Some(Piece::King(Color::Black))
        } else {
            None
        }
    }

    pub fn get_legal_moves(&self, color_to_move: Color) -> Vec<Move> {
        todo!()
    }

    pub fn make_move(&mut self, mov: Move) {
        todo!()
    }

    pub fn undo_move(&mut self, mov: Move) {
        todo!()
    }

    /// Returns `true` if the `color` pieces have the right to castle kingside.
    pub fn can_castle_kingside(&self, color: Color) -> bool {
        match color {
            Color::White => self.castling_rights.contains(CastleRights::WhiteKS),
            Color::Black => self.castling_rights.contains(CastleRights::BlackKS),
        }
    }

    /// Returns `true` if the `color` pieces have the right to castle queenside.
    pub fn can_castle_queenside(&self, color: Color) -> bool {
        match color {
            Color::White => self.castling_rights.contains(CastleRights::WhiteQS),
            Color::Black => self.castling_rights.contains(CastleRights::BlackQS),
        }
    }
}

impl Add<(i32, i32)> for Square {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            index: self.index + (rhs.0 + rhs.1 * 8) as u32,
        }
    }
}

#[test]
fn no_overlap() {
    let board = Board::new();
    assert!(board.is_valid());
}

#[test]
fn square_struct() {
    assert!(Square::from_notation("a1").is_ok());
    assert!(Square::from_notation("h8").is_ok());
    assert!(Square::from_notation("u9").is_err());
}

#[test]
fn move_struct() {
    assert!(Move::from_notation("a1h8").is_ok());
    assert!(Move::from_notation("a1a1").is_err());
    assert!(Move::from_notation("a1h").is_err());
    assert!(Move::from_notation("ah8").is_err());
}
