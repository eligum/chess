pub mod board;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let b = board::Board::new();
        assert_eq!(2 + 2, 4);
    }
}
