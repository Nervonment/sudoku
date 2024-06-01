use std::io::{self, stdout};

use crossterm::{terminal::EnterAlternateScreen, ExecutableCommand};
use sudoku::{
    // game::{
    // generator::{random_sudoku_puzzle_extrahard, random_sudoku_puzzle_normal},
    // solver::{SudokuSolver, SudokuSolverH},
    // },
    neo::{
        generator::random_sudoku_puzzle,
        puzzle::{Grid, SudokuPuzzleFull, SudokuPuzzleSimple},
        solver::{Grader, Solver, StochasticSolver, TechniquesSolver},
        techniques::{
            hidden_pair_blk, hidden_pair_col, hidden_pair_row, hidden_single_blk,
            hidden_single_col, hidden_single_row, naked_pair_blk, naked_pair_col, naked_pair_row,
            naked_single,
        },
    },
    ui::{draw_grid, draw_numbers},
};

fn main() -> io::Result<()> {
    let board = random_sudoku_puzzle::<
        StochasticSolver<SudokuPuzzleSimple>,
        TechniquesSolver<SudokuPuzzleFull>,
    >(45, 1000, 100000);
    // let board = [
    //     [4, 0, 8, 0, 0, 0, 0, 0, 9],
    //     [0, 0, 0, 4, 9, 0, 7, 0, 0],
    //     [0, 0, 0, 6, 8, 0, 0, 0, 0],
    //     [0, 0, 6, 8, 0, 0, 5, 0, 0],
    //     [8, 0, 1, 0, 4, 9, 0, 0, 0],
    //     [0, 0, 0, 0, 1, 6, 8, 0, 0],
    //     [0, 0, 0, 3, 7, 4, 9, 0, 0],
    //     [0, 0, 3, 9, 0, 0, 2, 0, 0],
    //     [9, 0, 0, 0, 6, 0, 0, 3, 7],
    // ];
    // let board = [
    //     [4, 0, 8, 0, 0, 0, 0, 0, 9],
    //     [0, 0, 0, 0, 0, 0, 7, 0, 0],
    //     [0, 0, 0, 6, 8, 0, 0, 0, 0],
    //     [0, 0, 6, 8, 0, 0, 5, 0, 0],
    //     [8, 0, 1, 0, 4, 9, 0, 0, 0],
    //     [0, 0, 0, 0, 1, 0, 8, 0, 0],
    //     [0, 0, 0, 0, 7, 4, 0, 0, 0],
    //     [0, 0, 3, 0, 0, 0, 2, 0, 0],
    //     [9, 0, 0, 0, 6, 0, 0, 3, 7],
    // ];
    // let board = [
    //     [4, 0, 8, 0, 0, 0, 0, 0, 9],
    //     [0, 0, 0, 4, 0, 0, 7, 0, 0],
    //     [0, 0, 0, 6, 8, 0, 0, 0, 0],
    //     [0, 0, 6, 8, 0, 0, 5, 0, 0],
    //     [8, 0, 1, 0, 4, 9, 0, 0, 0],
    //     [0, 0, 0, 0, 1, 0, 8, 0, 0],
    //     [0, 0, 0, 3, 7, 4, 0, 0, 0],
    //     [0, 0, 3, 0, 0, 0, 2, 0, 0],
    //     [9, 0, 0, 0, 6, 0, 0, 3, 7],
    // ];
    let puzzle = SudokuPuzzleFull::new(board);
    stdout().execute(EnterAlternateScreen)?;
    draw_grid()?;
    draw_numbers(&board, &board, &[[true; 9]; 9])?;
    println!("");
    println!("");
    let res_hidden_single_row = hidden_single_row(&puzzle);
    let res_hidden_single_col = hidden_single_col(&puzzle);
    let res_hidden_single_blk = hidden_single_blk(&puzzle);
    let res_naked_single = naked_single(&puzzle);
    let res_hidden_pair_row = hidden_pair_row(&puzzle);
    let res_hidden_pair_col = hidden_pair_col(&puzzle);
    let res_hidden_pair_blk = hidden_pair_blk(&puzzle);
    let res_naked_pair_row = naked_pair_row(&puzzle);
    let res_naked_pair_col = naked_pair_col(&puzzle);
    let res_naked_pair_blk = naked_pair_blk(&puzzle);
    println!("hidden single in row: {:?}", res_hidden_single_row);
    println!("hidden single in col: {:?}", res_hidden_single_col);
    println!("hidden single in blk: {:?}", res_hidden_single_blk);
    println!("naked single: {:?}", res_naked_single);
    println!("hidden pair in row: {:?}", res_hidden_pair_row);
    println!("hidden pair in col: {:?}", res_hidden_pair_col);
    println!("hidden pair in blk: {:?}", res_hidden_pair_blk);
    println!("naked pair in row: {:?}", res_naked_pair_row);
    println!("naked pair in col: {:?}", res_naked_pair_col);
    println!("naked pair in blk: {:?}", res_naked_pair_blk);
    let mut solver2 = TechniquesSolver::<SudokuPuzzleFull>::new(board);
    solver2.have_unique_solution();
    println!("{}", solver2.difficulty());

    // let mut solver1_total_invoke = 0;
    // let mut solver2_total_invoke = 0;
    // let times = 1;
    // for _ in 0..times {
    //     // let puzzle = random_sudoku_puzzle_extrahard();
    //     let puzzle = [
    //         [4, 0, 0, 0, 0, 7, 6, 0, 0],
    //         [0, 7, 0, 6, 1, 0, 0, 2, 3],
    //         [5, 0, 0, 0, 0, 0, 1, 0, 0],
    //         [0, 0, 0, 2, 8, 0, 0, 0, 0],
    //         [0, 0, 5, 0, 3, 0, 0, 0, 0],
    //         [9, 0, 0, 0, 7, 0, 0, 0, 0],
    //         [0, 0, 8, 0, 9, 3, 0, 0, 1],
    //         [0, 0, 0, 0, 0, 0, 8, 0, 0],
    //         [3, 0, 0, 4, 0, 0, 0, 5, 0],
    //     ];
    //     let mut solver1 = SudokuSolverH::new(puzzle);
    //     let mut solver2 = TechniquesSolver::<SudokuPuzzleFull>::new(puzzle);
    //     solver1.have_unique_solution();
    //     solver2.have_unique_solution();
    //     solver1_total_invoke += solver1.invoke_cnt;
    //     solver2_total_invoke += solver2.difficulty();
    // }
    // println!("solver1: {}", solver1_total_invoke / times);
    // println!("solver2: {}", solver2_total_invoke / times);

    Ok(())
}
