use crate::board::{Board, Move};
use crate::init::*;
use crate::piece::Piece;

pub trait MoveGen {
    fn generate_moves(&self, board: &Board) -> Vec<Move>;
}

pub struct Naive {
    squares_to_edge: SquaresToEdge,
}

impl Naive {
    pub fn new() -> Self {
        Self {
            squares_to_edge: compute_squares_to_edge(),
        }
    }

    /// Public because of benchmarking.
    pub(crate) fn generate_sliding_moves(
        &self,
        board: &Board,
        index_o: usize,
        piece_o: Piece,
        moves: &mut Vec<Move>,
    ) {
        let directions = match piece_o {
            Piece::Bishop(_) => &MOVE_DIRECTIONS[4..8],
            Piece::Rook(_) => &MOVE_DIRECTIONS[0..4],
            Piece::Queen(_) => &MOVE_DIRECTIONS[0..8],
            _ => {
                println!("ERROR: Piece is not slider.");
                return;
            }
        };

        for &direction in directions {
            println!("{direction:?}");
            for n in 1..=(self.squares_to_edge[index_o][direction]) {
                let index_t = index_o as i32 + direction.offset() * n as i32;
                println!("\t---");
                println!("\tOrigin: {index_o}");
                println!("\tN: {n}");
                println!("\tOffset: {}", direction.offset());
                println!("\tTarget: {index_t}");
                // If there is a piece on the target square...
                if let Some(piece_t) = board.at(index_t as usize) {
                    // and its color is different from the piece on the origin square,
                    if piece_o.color() != piece_t.color() {
                        // add the move (capture) and skip to the next direction.
                        moves.push(Move::from_indices(index_o, index_t as usize));
                        // break;
                    }
                } else {
                    // If the target square is empty, the move is always possible without taking
                    // into account pins or checks.
                    moves.push(Move::from_indices(index_o, index_t as usize));
                }
            }
            println!("---");
        }
    }
}

impl MoveGen for Naive {
    /// Generates pseudolegal moves for the current board position.
    fn generate_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        // println!("{}", self.squares_to_edge);
        // return moves;

        for (index, square) in board.iter().enumerate() {
            if let Some(piece) = square {
                if piece.color() == board.color_to_move {
                    if piece.is_slider() {
                        println!("{index}");
                        self.generate_sliding_moves(board, index, piece, &mut moves);
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
