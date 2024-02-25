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

pub struct Move {
    origin: u32,
    target: u32,
}

impl Move {
    pub fn from_notation(text: &str) -> Result<Self, String> {
        todo!()
    }

    pub fn from_indices(origin: u32, target: u32) -> Self {
        todo!()
    }
}

impl Board {
    #[rustfmt::skip]
    pub fn new() -> Self {
        Self {
            white_pawns:   0x40_40_40_40_40_40_40_40,
            white_rooks:   0x80_00_00_00_00_00_00_80,
            white_knights: 0x00_80_00_00_00_00_80_00,
            white_bishops: 0x00_00_80_00_00_80_00_00,
            white_queens:  0x00_00_00_80_00_00_00_00,
            white_kings:   0x00_00_00_00_80_00_00_00,
            black_pawns:   0x02_02_02_02_02_02_02_02,
            black_rooks:   0x01_00_00_00_00_00_00_01,
            black_knights: 0x00_01_00_00_00_00_01_00,
            black_bishops: 0x00_00_01_00_00_01_00_00,
            black_queens:  0x00_00_00_01_00_00_00_00,
            black_kings:   0x00_00_00_00_01_00_00_00,
        }
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
        let unique_white_king = self.white_kings > 0 && (self.white_kings & (self.white_kings - 1) == 0);
        let unique_black_king = self.black_kings > 0 && (self.black_kings & (self.black_kings - 1) == 0);
        let unique_kings = unique_white_king && unique_black_king;

        // self.black_pawns.count_ones();

        !overlap && unique_kings
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        todo!()
    }

    pub fn make_move(&mut self, mov: Move) {
        todo!()
    }
}

#[test]
fn no_overlap() {
    let board = Board::new();
    assert!(board.is_legal_position());
}
