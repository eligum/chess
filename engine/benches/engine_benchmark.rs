use criterion::{black_box, criterion_group, criterion_main, Criterion};
use engine::board;
use engine::piece::*;
use rand::Rng;

pub fn access_bitboards(indices: &[usize], board: &board::Board) {
    let pieces = [
        Piece::Pawn(Color::White),
        Piece::Knight(Color::White),
        Piece::Bishop(Color::White),
        Piece::Rook(Color::White),
        Piece::Queen(Color::White),
        Piece::King(Color::White),
        Piece::Pawn(Color::Black),
        Piece::Knight(Color::Black),
        Piece::Bishop(Color::Black),
        Piece::Rook(Color::Black),
        Piece::Queen(Color::Black),
        Piece::King(Color::Black),
    ];
    for &index in indices {
        board.get_bitboard(pieces[index]);
    }
}

pub fn crit_benchmark(crit: &mut Criterion) {
    crit.bench_function("bitboard access", |b| {
        let mut rng = rand::thread_rng();
        let indices = (0..100_000).map(|_| rng.gen_range(0..12) as usize).collect::<Vec<usize>>();
        let board = board::Board::new();

        b.iter(|| access_bitboards(black_box(&indices), black_box(&board)))
    });
}

criterion_group!(benches, crit_benchmark);
criterion_main!(benches);
