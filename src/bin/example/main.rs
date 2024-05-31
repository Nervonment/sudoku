use std::io::{self, stdout};

use crossterm::{terminal::EnterAlternateScreen, ExecutableCommand};
use sudoku::{
    game::generator::random_sudoku_puzzle_normal,
    neo::{
        puzzle::SudokuPuzzle,
        techniques::{hidden_single_blk, hidden_single_col, hidden_single_row, naked_single},
    },
    ui::{draw_grid, draw_numbers},
};

fn main() -> io::Result<()> {
    let board = random_sudoku_puzzle_normal();
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
    println!("hidden single in row: {:?}", res_hidden_single_row);
    println!("hidden single in col: {:?}", res_hidden_single_col);
    println!("hidden single in blk: {:?}", res_hidden_single_blk);
    println!("naked single: {:?}", res_naked_single);
    Ok(())
}
