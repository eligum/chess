//! Todo.

use crate::piece::*;
use std::ops::Add;

type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    index: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    origin: Square,
    target: Square,
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
                        "Unknown column coordinate '{}'. Expected a, b, c, d, e, f, g or h",
                        letter
                    ))
                }
            };
            if let Some(digit) = cs.next() {
                let row: u32 = match digit.to_digit(10) {
                    Some(x) if 1 <= x && x <= 8 => x - 1,
                    _ => {
                        return Err(format!(
                            "Unknown row coordinate '{}'. Expected 1, 2, 3, 4, 5, 6, 7 or 8",
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

    /// Returns the row/rank index of the square in the chess board.
    pub fn get_rank(&self) -> u32 {
        self.index / 8
    }

    /// Returns the column/file index of the square in the chess board.
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
    /// so you could end up with an illegal move.
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
    pub fn new() -> Self {
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
        }
    }

    pub fn from_array(array: &[Option<Piece>; 64]) -> Self {
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
        }
    }

    /// Returns an array of `Option<Piece>` that represents the current position of the board.
    pub fn extract_array(&self) -> [Option<Piece>; 64] {
        let mut result = [None; 64];
        
        result
    }

    pub fn is_legal_position(&self) -> bool {
        // Pieces overlap
        let overlap = self.white_pawns
            & self.white_knights
            & self.white_bishops
            & self.white_rooks
            & self.white_queens
            & self.white_kings
            & self.black_pawns
            & self.black_knights
            & self.black_bishops
            & self.black_rooks
            & self.black_queens
            & self.black_kings
            > 0;
        // Only one king of each color
        let unique_white_king =
            self.white_kings > 0 && (self.white_kings & (self.white_kings - 1) == 0);
        let unique_black_king =
            self.black_kings > 0 && (self.black_kings & (self.black_kings - 1) == 0);
        let unique_kings = unique_white_king && unique_black_king;

        // self.black_pawns.count_ones();

        !overlap && unique_kings
    }

    pub fn get_legal_moves(&self, white_to_play: bool) -> Vec<Move> {
        todo!()
    }

    pub fn make_move(&mut self, mov: Move) {
        todo!()
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
    assert!(board.is_legal_position());
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
