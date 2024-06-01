use criterion::{criterion_group, criterion_main, Criterion};
use sudoku::{
    game::generator::random_sudoku_puzzle_normal,
    neo::{
        puzzle::{SudokuPuzzleFull, SudokuPuzzleSimple},
        solver::{Solver, StochasticSolver, TechniquesSolver},
    },
};

fn benchmarks(c: &mut Criterion) {
    let puzzle = random_sudoku_puzzle_normal();
    let mut solver = StochasticSolver::<SudokuPuzzleSimple>::new(puzzle);
    c.bench_function("StochasticSolver", |b| {
        b.iter(|| {
            solver.any_solution();
        })
    });
    let mut solver = TechniquesSolver::<SudokuPuzzleFull>::new(puzzle);
    c.bench_function("TechniquesSolver", |b| {
        b.iter(|| {
            solver.any_solution();
        })
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
