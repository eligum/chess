use std::fmt;
use std::ops::Add;

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

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

pub enum Move {
    GoTo { row: u8, col: u8 },
    Take { row: u8, col: u8 },
}

pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
}

impl Piece {
    pub fn get_color(self) -> Color {
        match self {
            Piece::Pawn(c) => c,
            Piece::Knight(c) => c,
            Piece::Bishop(c) => c,
            Piece::Rook(c) => c,
            Piece::Queen(c) => c,
            Piece::King(c) => c,
        }
    }

    #[rustfmt::skip]
    pub fn get_possible_moves(self, board: &Board, coords: (i32, i32)) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        match self {
            Piece::Knight(color) => {
                let offsets: [(i8, i8); 8] = [
                    (1, 2),
                    (2, 1),
                    (2, -1),
                    (1, -2),
                    (-1, -2),
                    (-2, -1),
                    (-2, 1),
                    (-1, 2),
                ];
                let (row, col) = coords;
                for (row_offset, col_offset) in offsets {
                    let r = row as i8 + row_offset;
                    let c = col as i8 + col_offset;
                    if r >= 0 && c >= 0 {
                        let r = r as usize;
                        let c = c as usize;
                        if r < board.num_rows() && c < board.num_cols() {
                            match color {
                                Color::White => {
                                    if let Some(piece) = board.squares[r][c] {
                                        match piece {
                                            Piece::King(_) => (),
                                            _ => {
                                                if let Color::Black = piece.get_color() {
                                                    moves.push(Move::Take { row: r as u8, col: c as u8, });
                                                }
                                            }
                                        }
                                    } else {
                                        moves.push(Move::GoTo { row: r as u8, col: c as u8, });
                                    }
                                }
                                Color::Black => {
                                    if let Some(piece) = board.squares[r][c] {
                                        match piece {
                                            Piece::King(_) => (),
                                            _ => {
                                                if let Color::White = piece.get_color() {
                                                    moves.push(Move::Take { row: r as u8, col: c as u8, });
                                                }
                                            }
                                        }
                                    } else {
                                        moves.push(Move::GoTo { row: r as u8, col: c as u8, });
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Piece::Pawn(color) => {
                match color {
                    Color::White => {
                        let (r, c) = coords;
                        // Movement
                        if board.within_bounds(r + 1, c) {}
                        // Captures
                    }
                    Color::Black => todo!(),
                }
            }
            Piece::Bishop(_) => todo!(),
            Piece::Rook(_) => todo!(),
            Piece::Queen(_) => todo!(),
            Piece::King(_) => todo!(),
        }

        moves
    }
}

impl Board {
    /// Creates a new board with the standard arrangement of chess pieces.
    /// The position (8, 0) of the array corresponds with the a1 square of a
    /// real chess board in algebraic notation.
    pub fn new() -> Self {
        Board {
            squares: [
                [
                    Some(Piece::Rook(Color::Black)),
                    Some(Piece::Knight(Color::Black)),
                    Some(Piece::Bishop(Color::Black)),
                    Some(Piece::Queen(Color::Black)),
                    Some(Piece::King(Color::Black)),
                    Some(Piece::Bishop(Color::Black)),
                    Some(Piece::Knight(Color::Black)),
                    Some(Piece::Rook(Color::Black)),
                ],
                [
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                    Some(Piece::Pawn(Color::Black)),
                ],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                    Some(Piece::Pawn(Color::White)),
                ],
                [
                    Some(Piece::Rook(Color::White)),
                    Some(Piece::Knight(Color::White)),
                    Some(Piece::Bishop(Color::White)),
                    Some(Piece::Queen(Color::White)),
                    Some(Piece::King(Color::White)),
                    Some(Piece::Bishop(Color::White)),
                    Some(Piece::Knight(Color::White)),
                    Some(Piece::Rook(Color::White)),
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

    pub fn num_rows(&self) -> usize {
        8
    }

    pub fn num_cols(&self) -> usize {
        8
    }

    pub fn within_bounds(&self, row: i32, col: i32) -> bool {
        0 <= row && (row as usize) < self.num_rows() && 0 <= col && (col as usize) < self.num_cols()
    }
}

impl Position {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }

    pub fn from_indices(row_index: usize, col_index: usize) -> Self {
        Self {
            row: row_index as i32,
            col: col_index as i32,
        }
    }

    pub fn from_notation(text: &str) -> Result<Self, String> {
        let mut cs = text.chars();
        if let Some(letter) = cs.next() {
            let col: i32 = match letter {
                'a' | 'A' => 0,
                'b' | 'B' => 1,
                'c' | 'C' => 2,
                'd' | 'D' => 3,
                'e' | 'E' => 4,
                'f' | 'F' => 5,
                'g' | 'G' => 6,
                'h' | 'H' => 7,
                _ => return Err(format!("Unknown column coordinate '{}'", letter)),
            };
            if let Some(digit) = cs.next() {
                let row: i32 = match digit.to_digit(10) {
                    Some(x) if 1 <= x && x <= 8 => (8 - x) as i32,
                    _ => return Err(format!("Row coordinate out of range '{}'", digit)),
                };
                return Ok(Self { row, col });
            }
        }

        Err(format!("Failed to parse '{}' as algebraic notation", text))
    }
}

impl Add<(i32, i32)> for Position {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            row: self.row + rhs.0,
            col: self.col + rhs.1,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Piece::Pawn(color) => match color {
                Color::White => write!(f, "wP"),
                Color::Black => write!(f, "bP"),
            },
            Piece::Knight(color) => match color {
                Color::White => write!(f, "wN"),
                Color::Black => write!(f, "bN"),
            },
            Piece::Bishop(color) => match color {
                Color::White => write!(f, "wB"),
                Color::Black => write!(f, "bB"),
            },
            Piece::Rook(color) => match color {
                Color::White => write!(f, "wR"),
                Color::Black => write!(f, "bR"),
            },
            Piece::Queen(color) => match color {
                Color::White => write!(f, "wQ"),
                Color::Black => write!(f, "bQ"),
            },
            Piece::King(color) => match color {
                Color::White => write!(f, "wK"),
                Color::Black => write!(f, "bK"),
            },
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
