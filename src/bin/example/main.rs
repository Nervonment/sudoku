use std::io::{self, stdout};

use crossterm::{terminal::EnterAlternateScreen, ExecutableCommand};
use sudoku::{
    game::{
        generator::*,
        hint::{hidden_single, naked_single},
        solver::{SudokuSolver, SudokuSolverH},
    },
    ui::{draw_grid, draw_numbers},
};

fn main() -> io::Result<()> {
    // let times = 1000;
    // let mut total_invoke_cnt_h = 0;
    // let mut total_invoke_cnt_hp = 0;
    // for _ in 0..times {
    //     let puzzle = random_sudoku_puzzle(80);
    //     let mut solver = SudokuSolverH::new(puzzle);
    //     let mut solver_hp = SudokuSolverHP::new(puzzle);
    //     solver.get_solution();
    //     total_invoke_cnt_h += solver.invoke_cnt;
    //     solver_hp.get_solution();
    //     total_invoke_cnt_hp += solver_hp.invoke_cnt;
    // }
    // println!("{}", total_invoke_cnt_h / times);
    // println!("{}", total_invoke_cnt_hp / times);

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
    // let puzzle = [
    //     [0, 0, 2, 0, 0, 9, 6, 0, 5],
    //     [4, 0, 8, 0, 0, 1, 0, 0, 0],
    //     [0, 0, 0, 0, 2, 0, 0, 0, 0],
    //     [1, 0, 6, 9, 0, 0, 0, 5, 0],
    //     [0, 8, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 5, 7, 0, 9, 0],
    //     [0, 0, 4, 0, 0, 2, 0, 0, 3],
    //     [0, 0, 0, 0, 0, 0, 0, 2, 4],
    //     [0, 0, 0, 4, 1, 8, 7, 0, 0],
    // ];
    // let puzzle = random_sudoku_puzzle(80);
    // let puzzle = random_sudoku_puzzle_1(55, 55, 90);
    let puzzle = random_sudoku_puzzle_phishing();
    let mut solver = SudokuSolverH::new(puzzle);
    solver.have_unique_solution();

    stdout().execute(EnterAlternateScreen)?;
    draw_grid()?;
    draw_numbers(&puzzle, &puzzle, &[[true; 9]; 9])?;
    println!("");
    println!("");
    println!("{}, {}", solver.invoke_cnt, solver.unsure_cnt);
    println!("{:?}", hidden_single(&puzzle));
    println!("{:?}", naked_single(&puzzle));

    Ok(())
}
