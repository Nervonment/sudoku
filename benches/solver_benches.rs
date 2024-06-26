use criterion::{criterion_group, criterion_main, Criterion};
use sudoku::{
    generator::random_sudoku_puzzle_normal,
    solver::{advanced::AdvancedSolver, stochastic::StochasticSolver, Solver},
    state::{full_state::FullState, simple_state::SimpleState},
};

fn benchmarks(c: &mut Criterion) {
    let puzzle = random_sudoku_puzzle_normal();
    let mut solver = StochasticSolver::<SimpleState>::from(puzzle);
    c.bench_function("StochasticSolver", |b| {
        b.iter(|| {
            solver.any_solution();
        })
    });
    let mut solver = AdvancedSolver::<FullState>::from(puzzle);
    c.bench_function("AdvancedSolver", |b| {
        b.iter(|| {
            solver.any_solution();
        })
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
