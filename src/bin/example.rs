use sudoku::{
    generator::random_sudoku_puzzle,
    puzzle::{Grid, SudokuPuzzleFull, SudokuPuzzleSimple},
    solver::{Grader, Solver, StochasticSolver, TechniquesSolver},
    techniques::{
        hidden_pair_blk, hidden_pair_col, hidden_pair_row, hidden_single_blk, hidden_single_col,
        hidden_single_row, naked_pair_blk, naked_pair_col, naked_pair_row, naked_single, pointing,
    },
};

fn main() {
    let board = random_sudoku_puzzle::<
        StochasticSolver<SudokuPuzzleSimple>,
        TechniquesSolver<SudokuPuzzleFull>,
    >(45, 800, 100000);
    let puzzle = SudokuPuzzleFull::new(board);
    // TODO: print the puzzle
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
    let res_pointing = pointing(&puzzle);
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
    println!("pointing: {:?}", res_pointing);
    let mut solver2 = TechniquesSolver::<SudokuPuzzleFull>::new(board);
    solver2.have_unique_solution();
    println!("{}", solver2.difficulty());
    println!("{:?}", board);
}
