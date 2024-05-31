use std::io::{self, stdout};

use crossterm::{terminal::EnterAlternateScreen, ExecutableCommand};
use sudoku::{
    game::generator::random_sudoku_puzzle_normal,
    neo::{puzzle::SudokuPuzzle, techniques::{hidden_single_blk, hidden_single_col, hidden_single_row}},
    ui::{draw_grid, draw_numbers},
};

fn main() -> io::Result<()> {
    let board = random_sudoku_puzzle_normal();
    let puzzle = SudokuPuzzle::new(board);
    let res_row = hidden_single_row(&puzzle);
    let res_col = hidden_single_col(&puzzle);
    let res_blk = hidden_single_blk(&puzzle);
    stdout().execute(EnterAlternateScreen)?;
    draw_grid()?;
    draw_numbers(&board, &board, &[[true; 9]; 9])?;
    println!("");
    println!("");
    println!("hidden single in row: {:?}", res_row);
    println!("hidden single in col: {:?}", res_col);
    println!("hidden single in blk: {:?}", res_blk);
    Ok(())
}
