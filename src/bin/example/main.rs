use std::io::{self, stdout};

use crossterm::{terminal::EnterAlternateScreen, ExecutableCommand};
use sudoku::{
    game::generator::random_sudoku_puzzle_normal,
    neo::{
        puzzle::SudokuPuzzle,
        techniques::{
            hidden_pair_blk, hidden_pair_col, hidden_pair_row, hidden_single_blk,
            hidden_single_col, hidden_single_row, naked_pair_blk, naked_pair_col, naked_pair_row,
            naked_single,
        },
    },
    ui::{draw_grid, draw_numbers},
};

fn main() -> io::Result<()> {
    let board = random_sudoku_puzzle_normal();
    // let board = [
    //     [0, 1, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 4, 0, 1, 0, 8],
    //     [0, 0, 0, 0, 1, 0, 7, 5, 4],
    //     [0, 0, 8, 1, 0, 7, 0, 6, 5],
    //     [5, 7, 0, 0, 0, 0, 2, 1, 9],
    //     [0, 6, 1, 5, 0, 2, 8, 0, 0],
    //     [0, 0, 0, 3, 7, 0, 5, 0, 2],
    //     [0, 5, 0, 6, 2, 0, 0, 0, 0],
    //     [7, 0, 2, 0, 5, 0, 0, 0, 0],
    // ];
    let puzzle = SudokuPuzzle::new(board);
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
    Ok(())
}
