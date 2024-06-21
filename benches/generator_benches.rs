use criterion::{criterion_group, criterion_main, Criterion};
use sudoku::generator::{
    random_sudoku_puzzle_easy, random_sudoku_puzzle_extraeasy, random_sudoku_puzzle_extrahard, random_sudoku_puzzle_hard, random_sudoku_puzzle_normal
};

fn benchmarks(c: &mut Criterion) {
    c.bench_function("extraeasy", |b| {
        b.iter(|| {
            random_sudoku_puzzle_extraeasy();
        })
    });
    c.bench_function("easy", |b| {
        b.iter(|| {
            random_sudoku_puzzle_easy();
        })
    });
    c.bench_function("normal", |b| {
        b.iter(|| {
            random_sudoku_puzzle_normal();
        })
    });
    c.bench_function("hard", |b| {
        b.iter(|| {
            random_sudoku_puzzle_hard();
        })
    });
    c.bench_function("extrahard", |b| {
        b.iter(|| {
            random_sudoku_puzzle_extrahard();
        })
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
