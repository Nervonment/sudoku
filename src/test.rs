use crate::game::{
    generator::random_sudoku_puzzle_normal,
    judge::judge_sudoku,
    solver::{SudokuSolver, SudokuSolverH, SudokuSolverS},
};

#[test]
fn solver() {
    for _ in 0..100 {
        let puzzle = random_sudoku_puzzle_normal();

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
    }
}
