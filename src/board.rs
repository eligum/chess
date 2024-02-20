use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Pawn(PieceColor),
    Knight(PieceColor),
    Bishop(PieceColor),
    Rook(PieceColor),
    Queen(PieceColor),
    King(PieceColor),
}

pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    /// Creates a new board with the standard arrangement of chess pieces.
    /// The position (8, 0) of the array corresponds with the a1 square of a
    /// real chess board in algebraic notation.
    pub fn new() -> Self {
        Board {
            squares: [
                [
                    Some(Piece::Rook(PieceColor::Black)),
                    Some(Piece::Knight(PieceColor::Black)),
                    Some(Piece::Bishop(PieceColor::Black)),
                    Some(Piece::Queen(PieceColor::Black)),
                    Some(Piece::King(PieceColor::Black)),
                    Some(Piece::Bishop(PieceColor::Black)),
                    Some(Piece::Knight(PieceColor::Black)),
                    Some(Piece::Rook(PieceColor::Black)),
                ],
                [
                    Some(Piece::Pawn(PieceColor::Black)),
                    Some(Piece::Pawn(PieceColor::Black)),
                    Some(Piece::Pawn(PieceColor::Black)),
                    Some(Piece::Pawn(PieceColor::Black)),
                    Some(Piece::Pawn(PieceColor::Black)),
                    Some(Piece::Pawn(PieceColor::Black)),
                    Some(Piece::Pawn(PieceColor::Black)),
                    Some(Piece::Pawn(PieceColor::Black)),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(Piece::Pawn(PieceColor::White)),
                    Some(Piece::Pawn(PieceColor::White)),
                    Some(Piece::Pawn(PieceColor::White)),
                    Some(Piece::Pawn(PieceColor::White)),
                    Some(Piece::Pawn(PieceColor::White)),
                    Some(Piece::Pawn(PieceColor::White)),
                    Some(Piece::Pawn(PieceColor::White)),
                    Some(Piece::Pawn(PieceColor::White)),
                ],
                [
                    Some(Piece::Rook(PieceColor::White)),
                    Some(Piece::Knight(PieceColor::White)),
                    Some(Piece::Bishop(PieceColor::White)),
                    Some(Piece::Queen(PieceColor::White)),
                    Some(Piece::King(PieceColor::White)),
                    Some(Piece::Bishop(PieceColor::White)),
                    Some(Piece::Knight(PieceColor::White)),
                    Some(Piece::Rook(PieceColor::White)),
                ],
            ],
        }
    }

    pub fn empty() -> Self {
        Board {
            squares: [
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
            ],
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Piece::Pawn(color) => match color {
                PieceColor::White => write!(f, "wP"),
                PieceColor::Black => write!(f, "bP"),
            },
            Piece::Knight(color) => match color {
                PieceColor::White => write!(f, "wN"),
                PieceColor::Black => write!(f, "bN"),
            },
            Piece::Bishop(color) => match color {
                PieceColor::White => write!(f, "wB"),
                PieceColor::Black => write!(f, "bB"),
            },
            Piece::Rook(color) => match color {
                PieceColor::White => write!(f, "wR"),
                PieceColor::Black => write!(f, "bR"),
            },
            Piece::Queen(color) => match color {
                PieceColor::White => write!(f, "wQ"),
                PieceColor::Black => write!(f, "bQ"),
            },
            Piece::King(color) => match color {
                PieceColor::White => write!(f, "wK"),
                PieceColor::Black => write!(f, "bK"),
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = self.squares[i][j] {
                    write!(f, "{} ", piece)?;
                } else {
                    write!(f, " . ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
