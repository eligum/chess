use std::{
    cmp::min,
    fmt,
    ops::{Index, IndexMut},
};

pub const MOVE_DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
    Direction::NorthEast,
    Direction::SouthEast,
    Direction::NorthWest,
    Direction::SouthWest,
];

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Direction {
    North = 0,
    South,
    East,
    West,
    NorthEast,
    SouthWest,
    NorthWest,
    SouthEast,
}

impl Direction {
    /// The index offset of moving one square into this direction.
    pub fn offset(self) -> i32 {
        match self {
            Self::North => 8,
            Self::South => -8,
            Self::East => 1,
            Self::West => -1,
            Self::NorthEast => 9,
            Self::SouthWest => -9,
            Self::NorthWest => 7,
            Self::SouthEast => -7,
        }
    }
}

impl<T> Index<Direction> for [T; MOVE_DIRECTIONS.len()] {
    type Output = T;

    fn index(&self, index: Direction) -> &Self::Output {
        self.index(index as usize)
    }
}

impl<T> IndexMut<Direction> for [T; MOVE_DIRECTIONS.len()] {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        self.index_mut(index as usize)
    }
}

/// Computes the minimum distance to the edge of the board from each square for every direction.
// #[rustfmt::skip]
pub fn compute_squares_to_edge() -> SquaresToEdge {
    let mut squares_to_edge = [[0; 8]; 64];
    for rank in 0..8 {
        for file in 0..8 {
            let n = 7 - rank;
            let s = rank;
            let e = 7 - file;
            let w = file;
            let index = rank * 8 + file;
            squares_to_edge[index][Direction::North] = n;
            squares_to_edge[index][Direction::South] = s;
            squares_to_edge[index][Direction::East] = e;
            squares_to_edge[index][Direction::West] = w;
            squares_to_edge[index][Direction::NorthEast] = min(n, e);
            squares_to_edge[index][Direction::SouthEast] = min(s, e);
            squares_to_edge[index][Direction::NorthWest] = min(n, w);
            squares_to_edge[index][Direction::SouthWest] = min(s, w);
        }
    }

    SquaresToEdge(squares_to_edge)
}

pub struct SquaresToEdge([[usize; 8]; 64]);

impl Index<usize> for SquaresToEdge {
    type Output = [usize; 8];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl fmt::Display for SquaresToEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..8 {
            for col in 0..8 {
                write!(f, "(index {:>2}) [{}][{}]", row * 8 + col, row, col)?;
                for &direction in MOVE_DIRECTIONS {
                    write!(f, " {}: {}", direction, self.0[8 * row + col][direction])?;
                }
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => write!(f, "N"),
            Direction::South => write!(f, "S"),
            Direction::East => write!(f, "E"),
            Direction::West => write!(f, "W"),
            Direction::NorthEast => write!(f, "NE"),
            Direction::SouthEast => write!(f, "SE"),
            Direction::NorthWest => write!(f, "NW"),
            Direction::SouthWest => write!(f, "SW"),
        }
    }
}

//
// Tests
//

#[test]
#[rustfmt::skip]
fn squares_to_edge() {
    let t = MOVE_DIRECTIONS;
    assert_eq!(
        compute_squares_to_edge().0,
        [
            [7, 0, 7, 0, 7, 0, 0, 0]; 64
        ],
    );
}
