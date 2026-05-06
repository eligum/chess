use crate::board::{Board, Move};
use crate::init::*;
use crate::piece::{Color, Piece};
use std::arch::x86_64::_pext_u64;

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
            // println!("{:?}", direction);
            for n in 1..=(self.squares_to_edge[index_o][direction]) {
                let index_t = index_o as i32 + direction.offset() * n as i32;
                // println!("\t---");
                // println!("\tOrigin: {index_o}");
                // println!("\tN: {n}");
                // println!("\tOffset: {}", direction.offset());
                // println!("\tTarget: {index_t}u);
                // If there is a piece on the target square...
                if let Some(piece_t) = board.at(index_t as usize) {
                    // register the move (capture) only if its color is different from the piece on the origin square.
                    if piece_o.color() != piece_t.color() {
                        moves.push(Move::from_indices(index_o, index_t as usize));
                    }
                    // Skip to next direction
                    break;
                } else {
                    // If the target square is empty, the move is always possible without taking
                    // into account pins or checks.
                    moves.push(Move::from_indices(index_o, index_t as usize));
                }
            }
        }
    }

    fn generate_pawn_moves(
        board: &Board,
        index_o: usize,
        piece_color_o: Color,
        moves: &mut Vec<Move>,
    ) {
        let current_row = index_o / 8;
        let current_col = index_o % 8;
        let mut headstart_row = 1;
        let mut adv1_offset: i32 = 8;
        let mut adv2_offset: i32 = 16;
        let mut eat_l_offset: i32 = 7;
        let mut eat_r_offset: i32 = 9;
        if piece_color_o == Color::Black {
            headstart_row = 6;
            adv1_offset = -8;
            adv2_offset = -16;
            eat_l_offset = -9;
            eat_r_offset = -7;
        }

        if 0 < current_row && current_row < 7 {
            let index_t = index_o + adv1_offset;
            if board.at(index_t).is_none() {
                moves.push(Move::from_indices(index_o, index_t));
            }
            if current_row == headstart_row {
                let index_t = index_o + adv2_offset;
                if board.at(index_t).is_none() {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            }
            // Captures
            // TODO: Check "en passant".
            if current_col > 0 {
                let index_t = index_o + eat_l_offset;
                if board.at(index_t).is_some_and(|p| p.color() != piece_color_o) {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            }
            if current_col < 7 {
                let index_t = index_o + eat_r_offset;
                if board.at(index_t).is_some_and(|p| p.color() != piece_color_o) {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            }
        }
    }

    fn generate_knight_moves(
        board: &Board,
        index_o: usize,
        piece_color_o: Color,
        moves: &mut Vec<Move>,
    ) {
        let current_row = index_o / 8;
        let current_col = index_o % 8;

        // north-east
        if current_row < 6 && current_col < 7 {
            let index_t = index_o + 17;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
        // north-west
        if current_row < 6 && current_col > 0 {
            let index_t = index_o + 15;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
        // east-north
        if current_row < 7 && current_col < 6 {
            let index_t = index_o + 10;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
        // east-south
        if current_row > 0 && current_col < 6 {
            let index_t = index_o - 6;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
        // south-east
        if current_row > 1 && current_col < 7 {
            let index_t = index_o - 15;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
        // south-west
        if current_row > 1 && current_col > 0 {
            let index_t = index_o - 17;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
        // west-south
        if current_row > 0 && current_col > 1 {
            let index_t = index_o - 10;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
        // west-north
        if current_row < 7 && current_col > 1 {
            let index_t = index_o + 6;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
    }

    fn generate_king_moves(
        board: &Board,
        index_o: usize,
        piece_color_o: Color,
        moves: &mut Vec<Move>,
    ) {
        let current_row = index_o / 8;
        let current_col = index_o % 8;

        // Castling
        // TODO: Implement.

        // north
        if current_row < 7 {
            let index_t = index_o + 8;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
        // north-east
        if current_row < 7 && current_col < 7 {
            let index_t = index_o + 9;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
        // east
        if current_col < 7 {
            let index_t = index_o + 1;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
        // south-east
        if current_row > 0 && current_col < 7 {
            let index_t = index_o - 7;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
        // south
        if current_row > 0 {
            let index_t = index_o - 8;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
        // south-west
        if current_row > 0 && current_col > 0 {
            let index_t = index_o - 9;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
        // west
        if current_col > 0 {
            let index_t = index_o - 1;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
        // north-west
        if current_row < 7 && current_col > 0 {
            let index_t = index_o + 7;
            if let Some(piece) = board.at(index_t) {
                if piece.color() != piece_color_o {
                    moves.push(Move::from_indices(index_o, index_t));
                }
            } else {
                moves.push(Move::from_indices(index_o, index_t));
            }
        }
    }
}

impl MoveGen for Naive {
    /// Generates pseudolegal moves for the current board position.
    fn generate_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        for (index, square) in board.iter().enumerate() {
            if let Some(piece) = square {
                if piece.color() == board.color_to_move {
                    if piece.is_slider() {
                        self.generate_sliding_moves(board, index, piece, &mut moves);
                    } else {
                        match piece {
                            Piece::Pawn(color) => Self::generate_pawn_moves(board, index, color, &mut moves),
                            Piece::Knight(color) => Self::generate_knight_moves(board, index, color, &mut moves),
                            Piece::King(color) => Self::generate_king_moves(board, index, color, &mut moves),
                            _ => unreachable!(),
                        };
                    }
                }
            }
        }

        moves
    }
}

pub fn extract_target_indices(moves: &[Move], origin_index: usize) -> Vec<usize> {
    moves
        .iter()
        .filter_map(|mov| {
            if mov.origin.index as usize == origin_index {
                Some(mov.target.index.into())
            } else {
                None
            }
        })
        .collect()
}
