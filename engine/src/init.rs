use std::cmp::min;

pub const DIRECTION_OFFSETS: &[i32] = &[8, -8, 1, -1, 9, -9, 7, -7];

pub enum DirectionOffset {
    North,
    South,
    East,
    West,
    NorthEast,
    SouthWest,
    NorthWest,
    SouthEast,
}

/// Computes the minimum distance to the edge of the board from each square for every direction.
#[rustfmt::skip]
pub fn compute_squares_to_edge() -> [[usize; 8]; 64] {
    let mut squares_to_edge = [[0; 8]; 64];
    for rank in 0..8 {
        for file in 0..8 {
            let n = 7 - rank;
            let s = rank;
            let e = 7 - file;
            let w = file;
            let index = rank * 8 + file;
            squares_to_edge[index] = [
                n,         //  8
                s,         // -8
                e,         //  1
                w,         // -1
                min(n, e), //  9
                min(s, w), // -9
                min(n, w), //  7
                min(s, e), // -7
            ];
        }
    }

    squares_to_edge
}
