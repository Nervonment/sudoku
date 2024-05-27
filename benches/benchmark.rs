use criterion::{criterion_group, criterion_main, Criterion};
use sudoku::game::solver::{SudokuSolver, SudokuSolverH};

pub fn solver_benchmark(c: &mut Criterion) {
    let puzzle = [
        [0, 6, 0, 8, 0, 0, 0, 0, 5],
        [0, 0, 9, 0, 0, 2, 0, 0, 0],
        [0, 8, 0, 0, 6, 4, 0, 0, 7],
        [0, 0, 0, 4, 7, 0, 8, 0, 0],
        [0, 0, 2, 0, 0, 0, 1, 0, 6],
        [0, 5, 0, 2, 0, 0, 0, 0, 0],
        [0, 0, 0, 5, 0, 0, 0, 0, 0],
        [0, 0, 3, 0, 0, 8, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 9, 2, 0],
    ];
    let mut solver = SudokuSolverH::new(puzzle);
    c.bench_function("solve sudoku", |b| {
        b.iter(|| {
            solver.get_solution();
        })
    });
}

criterion_group!(benches, solver_benchmark);
criterion_main!(benches);
