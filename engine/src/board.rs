//! Board representation.
//!
//! Least significant bit (LSB) represents the 'a1' square while the most significant
//! bit (MSB) represents the 'h8' square.
//!
//! TODO: Expand this section.

use crate::{bits, piece::*};
use bitflags::bitflags;
use std::ops::Add;

type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    // pieces
    piece_bitboards: [u64; 12],
    // position information
    pub(crate) color_to_move: Color,
    pub(crate) castling_rights: CastleRights,
    pub(crate) en_passant_square: Option<usize>,
    // auxiliary structures
    squares: [Option<Piece>; 64],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    pub index: u8,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
        let mut characters = text.chars();
        if let Some(letter) = characters.next() {
            let col: u8 = match letter {
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
            if let Some(digit) = characters.next() {
                let row: u8 = match digit.to_digit(10) {
                    Some(x) if 1 <= x && x <= 8 => x as u8 - 1,
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
    #[inline]
    pub fn get_rank(&self) -> usize {
        (self.index / 8).into()
    }

    /// Returns the column index (file) of the square.
    #[inline]
    pub fn get_file(&self) -> usize {
        (self.index % 8).into()
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
    pub fn from_indices(origin: usize, target: usize) -> Self {
        // TODO: Perhaps force `origin` and `target` to be u8 too.
        Self {
            origin: Square {
                index: origin.try_into().unwrap(),
            },
            target: Square {
                index: target.try_into().unwrap(),
            },
        }
    }
}

macro_rules! board {
    (wP) => {Some(Piece::Pawn(Color::White))};
    (wN) => {Some(Piece::Knight(Color::White))};
    (wB) => {Some(Piece::Bishop(Color::White))};
    (wR) => {Some(Piece::Rook(Color::White))};
    (wQ) => {Some(Piece::Queen(Color::White))};
    (wK) => {Some(Piece::King(Color::White))};
    (bP) => {Some(Piece::Pawn(Color::Black))};
    (bN) => {Some(Piece::Knight(Color::Black))};
    (bB) => {Some(Piece::Bishop(Color::Black))};
    (bR) => {Some(Piece::Rook(Color::Black))};
    (bQ) => {Some(Piece::Queen(Color::Black))};
    (bK) => {Some(Piece::King(Color::Black))};
    (.)  => {None};
    ($($p:tt)*) => {[
        $(board!($p)),+
    ]}
}

impl Board {
    /// Constructs a new `Board` with the standard piece arrangement.
    ///
    /// To understand more about the board representation read the `bitboard` module documentation.
    pub const fn new() -> Self {
        Self {
            piece_bitboards: [
                0x00_00_00_00_00_00_ff_00, // white pawns
                0x00_00_00_00_00_00_00_42, // white knights
                0x00_00_00_00_00_00_00_24, // white bishops
                0x00_00_00_00_00_00_00_81, // white rooks
                0x00_00_00_00_00_00_00_08, // white queens
                0x00_00_00_00_00_00_00_10, // white kings
                0x00_ff_00_00_00_00_00_00, // black pawns
                0x42_00_00_00_00_00_00_00, // black knights
                0x24_00_00_00_00_00_00_00, // black bishops
                0x81_00_00_00_00_00_00_00, // black rooks
                0x08_00_00_00_00_00_00_00, // black queens
                0x10_00_00_00_00_00_00_00, // black kings
            ],
            castling_rights: CastleRights::All,
            color_to_move: Color::White,
            en_passant_square: None,
            squares: board![
                wR wN wB wQ wK wB wN wR
                wP wP wP wP wP wP wP wP
                .  .  .  .  .  .  .  .
                .  .  .  .  .  .  .  .
                .  .  .  .  .  .  .  .
                .  .  .  .  .  .  .  .
                bP bP bP bP bP bP bP bP
                bR bN bB bQ bK bB bN bR
            ],
        }
    }

    /// Constructs a new empty `Board`.
    pub const fn empty() -> Self {
        Self {
            piece_bitboards: [0; 12],
            color_to_move: Color::White,
            castling_rights: CastleRights::None,
            en_passant_square: None,
            squares: [None; 64],
        }
    }

    #[inline]
    fn bitboard_index(piece: Piece) -> usize {
        match piece {
            Piece::Pawn(color) => match color {
                Color::White => 0,
                Color::Black => 6,
            },
            Piece::Knight(color) => match color {
                Color::White => 1,
                Color::Black => 7,
            },
            Piece::Bishop(color) => match color {
                Color::White => 2,
                Color::Black => 8,
            },
            Piece::Rook(color) => match color {
                Color::White => 3,
                Color::Black => 9,
            },
            Piece::Queen(color) => match color {
                Color::White => 4,
                Color::Black => 10,
            },
            Piece::King(color) => match color {
                Color::White => 5,
                Color::Black => 11,
            },
        }
    }

    /// Returns the bitboard for `piece`.
    ///
    /// These bitboards contain informaton of the position of each piece on the board.
    #[inline]
    pub fn get_bitboard(&self, piece: Piece) -> u64 {
        self.piece_bitboards[Self::bitboard_index(piece)]
    }

    pub(crate) fn from_array(
        squares: [Option<Piece>; 64],
        castling_rights: CastleRights,
        color_to_move: Color,
        en_passant_square: Option<usize>,
    ) -> Self {
        let mut board = Self {
            piece_bitboards: [0; 12],
            castling_rights,
            color_to_move,
            en_passant_square,
            squares,
        };
        for (index, square) in board.squares.into_iter().enumerate() {
            if let Some(piece) = square {
                board.piece_bitboards[Self::bitboard_index(piece)] |= 1 << index;
            }
        }

        board
    }

    /// Returns an array of `Option<Piece>` that represents squares of the board.
    pub fn piece_array(&self) -> &[Option<Piece>; 64] {
        &self.squares
    }

    /// Returns the bitboard resulting from the union of all the piece bitboards.
    ///
    /// The occupancy refers to the set of all squares occupied by any piece. Thus, the
    /// complement of the occupancy is the set of all empty squares.
    pub fn occupancy(&self) -> u64 {
        self.piece_bitboards.iter().fold(0, |acc, x| acc | x)
    }

    /// Checks whether the internal representation of the board is in a valid state.
    ///
    /// This function is meant for debugging and the development of the `chess::engine`
    /// crate. It checks for overlapping piece bitboards and verifies the presence of
    /// only one king of each color.
    pub fn is_valid(&self) -> bool {
        // Pieces overlap
        let overlap = self
            .piece_bitboards
            .iter()
            .try_fold(0, |acc: u64, &x| acc.checked_add(x))
            != Some(self.occupancy());
        // Only one king of each color
        let unique_kings = bits::only_one(self.get_bitboard(Piece::King(Color::White)))
            && bits::only_one(self.get_bitboard(Piece::King(Color::Black)));

        !overlap && unique_kings
    }

    pub fn at(&self, index: usize) -> Option<Piece> {
        assert!(index < 64, "Board index {index} is out of bounds!");

        // if index > 63 {
        //     None
        // } else {
        //     self.squares[index]
        // }

        // self.piece_bitboards
        //     .iter()
        //     .find(|&bitboard| bitboard & (1 << index) > 0)
        //     .and_then(|piece| Some(piece));

        let mask = 1 << index;

        if mask & self.get_bitboard(Piece::Pawn(Color::White)) > 0 {
            Some(Piece::Pawn(Color::White))
        } else if mask & self.get_bitboard(Piece::Knight(Color::White)) > 0 {
            Some(Piece::Knight(Color::White))
        } else if mask & self.get_bitboard(Piece::Bishop(Color::White)) > 0 {
            Some(Piece::Bishop(Color::White))
        } else if mask & self.get_bitboard(Piece::Rook(Color::White)) > 0 {
            Some(Piece::Rook(Color::White))
        } else if mask & self.get_bitboard(Piece::Queen(Color::White)) > 0 {
            Some(Piece::Queen(Color::White))
        } else if mask & self.get_bitboard(Piece::King(Color::White)) > 0 {
            Some(Piece::King(Color::White))
        } else if mask & self.get_bitboard(Piece::Pawn(Color::Black)) > 0 {
            Some(Piece::Pawn(Color::Black))
        } else if mask & self.get_bitboard(Piece::Knight(Color::Black)) > 0 {
            Some(Piece::Knight(Color::Black))
        } else if mask & self.get_bitboard(Piece::Bishop(Color::Black)) > 0 {
            Some(Piece::Bishop(Color::Black))
        } else if mask & self.get_bitboard(Piece::Rook(Color::Black)) > 0 {
            Some(Piece::Rook(Color::Black))
        } else if mask & self.get_bitboard(Piece::Queen(Color::Black)) > 0 {
            Some(Piece::Queen(Color::Black))
        } else if mask & self.get_bitboard(Piece::King(Color::Black)) > 0 {
            Some(Piece::King(Color::Black))
        } else {
            None
        }
    }

    /// Returns the color of the pieces whose turn is to play.
    #[inline]
    pub fn color_to_move(&self) -> Color {
        self.color_to_move
    }

    pub fn compute_legal_moves_for(&self, index: usize) -> Vec<Move> {
        todo!()
    }

    pub fn get_legal_moves(&self, color_to_move: Color) -> Vec<Move> {
        todo!()
    }

    /// Applies a move and returns `true` if the move updated the state of the board.
    #[rustfmt::skip]
    pub fn make_move(&mut self, mov: Move) -> bool {
        if mov.origin == mov.target {
            return false;
        }
        let origin = mov.origin.index as usize;
        let target = mov.target.index as usize;
        if let Some(piece_o) = self.at(origin) {
            if let Some(piece_t) = self.at(target) {
                if piece_o.color() == piece_t.color() {
                    return false;
                }
                self.piece_bitboards[Self::bitboard_index(piece_o)] ^= (1 << origin) | (1 << target);
                self.piece_bitboards[Self::bitboard_index(piece_t)] ^= 1 << target;

                self.squares[origin] = None;
                self.squares[target] = Some(piece_o);
            } else {
                self.piece_bitboards[Self::bitboard_index(piece_o)] ^= (1 << origin) | (1 << target);

                self.squares[origin] = None;
                self.squares[target] = Some(piece_o);
            }
            // Change whose turn it is since a move has been successfully applied
            self.color_to_move = match self.color_to_move {
                Color::White => Color::Black,
                Color::Black => Color::White,
            };
            println!("{:?} bitboard: {:08x}", piece_o, self.get_bitboard(piece_o));
        } else {
            return false;
        }
        true
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

    /// Returns an iterator over the squares of the board.
    ///
    /// The iterator yields all squares ([`Option<Piece>`](crate::piece::Piece)) from 'a1' to 'h8'.
    pub fn iter(&self) -> BoardIter<'_> {
        BoardIter {
            board: self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = Option<Piece>;
    type IntoIter = BoardIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct BoardIter<'a> {
    board: &'a Board,
    index: usize,
}

impl<'a> Iterator for BoardIter<'a> {
    type Item = Option<Piece>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 63 {
            None
        } else {
            let item = Some(self.board.at(self.index));
            self.index += 1;
            item
        }
    }
}

impl Add<(i32, i32)> for Square {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            index: self.index + (rhs.0 + rhs.1 * 8) as u8,
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
    assert!(Square::from_notation("b0").is_err());
}

#[test]
fn move_struct() {
    assert!(Move::from_notation("a1h8").is_ok());
    assert!(Move::from_notation("a1a1").is_err());
    assert!(Move::from_notation("a1h").is_err());
    assert!(Move::from_notation("ah8").is_err());
}
