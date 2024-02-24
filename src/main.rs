#[allow(unused_imports)]
use chess::board::{Board, Piece, Position};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let b = Board::new();
    println!("{}", b);

    let stdin = std::io::stdin();

    println!("Enter 5 chess board positions in algebraic notation.");
    let mut i = 1;
    while i < 6 {
        print!("[{}] Answer: ", i);
        std::io::stdout().flush()?;

        let mut buff = String::new();
        stdin.read_line(&mut buff)?;
        let trimmed = buff.trim();

        match Position::from_notation(trimmed) {
            Ok(pos) => {
                println!("Great! You entered valid notation that maps to {:?}", pos);
                i += 1;
            }
            Err(msg) => println!("{}.", msg),
        }
    }

    Ok(())
}
