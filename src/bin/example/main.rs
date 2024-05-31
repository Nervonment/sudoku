use std::io::{self, stdout};

use crossterm::{terminal::EnterAlternateScreen, ExecutableCommand};
use sudoku::{
    game::generator::random_sudoku_puzzle_normal,
    neo::{puzzle::SudokuPuzzle, techniques::hidden_single},
    ui::{draw_grid, draw_numbers},
};

fn main() -> io::Result<()> {
    let board = random_sudoku_puzzle_normal();
    let puzzle = SudokuPuzzle::new(board);
    let res = hidden_single(&puzzle);
    stdout().execute(EnterAlternateScreen)?;
    draw_grid()?;
    draw_numbers(&board, &board, &[[true; 9]; 9])?;
    println!("");
    println!("");
    println!("{:?}", res);
    Ok(())
}
