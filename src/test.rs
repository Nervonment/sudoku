use std::time::Instant;

use crate::game::{
    generator::random_sudoku_puzzle,
    judge::judge_sudoku,
    solver::{SudokuSolver, SudokuSolverH, SudokuSolverS},
};

#[test]
fn solver_benchmark() {
    let mut solver_s_time_ms = 0;
    let mut solver_h_time_ms = 0;
    for _ in 0..10 {
        let puzzle = random_sudoku_puzzle(60);

        let mut solver_s = SudokuSolverS::new(puzzle);
        let mut solver_h = SudokuSolverH::new(puzzle);

        // 生成的数独谜面具有唯一解
        assert!(solver_s.have_unique_solution());
        assert!(solver_h.have_unique_solution());

        let solution_solver_s = solver_s.get_solution().unwrap();
        let solution_solver_h = solver_h.get_solution().unwrap();
        // 两个Solver给出的解是满足约束的完全解
        assert!(judge_sudoku(&solution_solver_s).1);
        assert!(judge_sudoku(&solution_solver_h).1);

        let start = Instant::now();
        for _ in 0..10 {
            solver_s.have_unique_solution();
        }
        solver_s_time_ms += start.elapsed().as_millis();

        let start = Instant::now();
        for _ in 0..10 {
            solver_h.have_unique_solution();
        }
        solver_h_time_ms += start.elapsed().as_millis();
    }
    println!("solver_s: {}ms", solver_s_time_ms);
    println!("solver_h: {}ms", solver_h_time_ms);
}
