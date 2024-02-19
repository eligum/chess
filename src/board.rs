use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
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
                    Some(Piece::Rook),
                    Some(Piece::Knight),
                    Some(Piece::Bishop),
                    Some(Piece::Queen),
                    Some(Piece::King),
                    Some(Piece::Bishop),
                    Some(Piece::Knight),
                    Some(Piece::Rook),
                ],
                [
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                    Some(Piece::Pawn),
                ],
                [
                    Some(Piece::Rook),
                    Some(Piece::Knight),
                    Some(Piece::Bishop),
                    Some(Piece::Queen),
                    Some(Piece::King),
                    Some(Piece::Bishop),
                    Some(Piece::Knight),
                    Some(Piece::Rook),
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
            Piece::Pawn => write!(f, "P"),
            Piece::Knight => write!(f, "N"),
            Piece::Bishop => write!(f, "B"),
            Piece::Rook => write!(f, "R"),
            Piece::Queen => write!(f, "Q"),
            Piece::King => write!(f, "K"),
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
                    write!(f, ". ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
