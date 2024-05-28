use criterion::{criterion_group, criterion_main, Criterion};
use sudoku::game::{
    generator::{
        random_sudoku_puzzle_easy, random_sudoku_puzzle_extraeasy, random_sudoku_puzzle_extrahard,
        random_sudoku_puzzle_hard, random_sudoku_puzzle_normal,
    },
    // solver::{SudokuSolver, SudokuSolverH, SudokuSolverHP},
};

pub fn benchmarks(c: &mut Criterion) {
    // let puzzle = [
    //     [5, 0, 0, 0, 0, 0, 3, 0, 0],
    //     [0, 2, 0, 1, 0, 0, 0, 7, 0],
    //     [0, 0, 8, 0, 0, 0, 0, 0, 9],
    //     [0, 4, 0, 0, 0, 7, 0, 0, 0],
    //     [0, 0, 0, 8, 2, 1, 0, 0, 0],
    //     [0, 0, 0, 6, 0, 0, 0, 1, 0],
    //     [3, 0, 0, 0, 0, 0, 8, 0, 0],
    //     [0, 6, 0, 0, 0, 4, 0, 2, 0],
    //     [0, 0, 9, 0, 0, 0, 0, 0, 5],
    // ];
    // // let puzzle = random_sudoku_puzzle(80);
    // let mut solver = SudokuSolverH::new(puzzle);
    // c.bench_function("SudokuSolverH", |b| {
    //     b.iter(|| {
    //         solver.get_solution();
    //     })
    // });
    // let mut solver_hp = SudokuSolverHP::new(puzzle);
    // c.bench_function("SudoluSolverHP", |b| {
    //     b.iter(|| {
    //         solver_hp.get_solution();
    //     })
    // });

    c.bench_function("generator_extrahard", |b| {
        b.iter(|| {
            random_sudoku_puzzle_extrahard();
        })
    });

    c.bench_function("generator_hard", |b| {
        b.iter(|| {
            random_sudoku_puzzle_hard();
        })
    });

    c.bench_function("generator_normal", |b| {
        b.iter(|| {
            random_sudoku_puzzle_normal();
        })
    });

    c.bench_function("generator_easy", |b| {
        b.iter(|| {
            random_sudoku_puzzle_easy();
        })
    });

    c.bench_function("generator_extraeasy", |b| {
        b.iter(|| {
            random_sudoku_puzzle_extraeasy();
        })
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
