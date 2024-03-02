pub mod board;
pub mod bitboard;
pub mod piece;
pub mod parser;
pub mod errors;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _b = board::Board::new();
        assert_eq!(2 + 2, 4);
    }
}
