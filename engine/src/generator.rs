use crate::board::{Board, Move};
use crate::init::*;
use crate::piece::Piece;

pub trait MoveGen {
    fn generate_moves(&self) -> Vec<Move>;
}

pub struct MoveGenerator<'a> {
    board: &'a Board,
    squares_to_edge: [[usize; 8]; 64],
}

impl<'a> MoveGenerator<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self {
            board,
            squares_to_edge: compute_squares_to_edge(),
        }
    }

    /// Public becuase of benchmarking.
    pub(crate) fn generate_sliding_moves(
        &self,
        index_o: usize,
        piece_o: Piece,
        moves: &mut Vec<Move>,
    ) {
        let offsets = match piece_o {
            Piece::Bishop(_) => &DIRECTION_OFFSETS[4..8],
            Piece::Rook(_) => &DIRECTION_OFFSETS[0..4],
            Piece::Queen(_) => &DIRECTION_OFFSETS[0..8],
            _ => {
                println!("ERROR: Piece is not slider.");
                return;
            }
        };

        for (i, &offset) in offsets.iter().enumerate() {
            for n in 1..=(self.squares_to_edge[index_o][i]) {
                let index_t = index_o as i32 + offset * n as i32;
                if let Some(piece_t) = self.board.at(index_t as usize) {
                    // If there is a piece on the target square and it is of the opposite
                    // color, add the move (capture) and skip to the next direction.
                    if piece_o.color() != piece_t.color() {
                        moves.push(Move::from_indices(index_o, index_t as usize));
                    }
                } else {
                    // If the square is empty, the move is always possible without taking
                    // into account pins or checks.
                    moves.push(Move::from_indices(index_o, index_t as usize));
                }
            }
        }
    }
}

impl<'a> MoveGen for MoveGenerator<'a> {
    /// Generates pseudolegal moves for the current board position.
    fn generate_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for (index, square) in self.board.iter().enumerate() {
            if let Some(piece) = square {
                if piece.color() == self.board.color_to_move {
                    if piece.is_slider() {
                        self.generate_sliding_moves(index, piece, &mut moves);
                    } else {
                        match piece {
                            Piece::Pawn(_) => {},
                            Piece::Knight(_) => {},
                            Piece::King(_) => {},
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }

        moves
    }
}
