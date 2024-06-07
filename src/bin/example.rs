use sudoku::{
    generator::random_sudoku_puzzle_normal,
    solver::{advanced::AdvancedSolver, Grader, Solver},
    state::full_state::FullState,
    techniques::{
        hidden_subsets::{HiddenPairBlock, HiddenPairColumn, HiddenPairRow},
        locked_candidates::{Claiming, Pointing},
        naked_subsets::{NakedPairBlock, NakedPairColumn, NakedPairRow},
        singles::{HiddenSingleBlock, HiddenSingleColumn, HiddenSingleRow, NakedSingle},
        Technique,
    },
};

fn main() {
    let grid = random_sudoku_puzzle_normal();
    let state = FullState::from(grid);
    println!("{}", grid);
    let res_hidden_single_row = HiddenSingleRow::check(&state).0;
    let res_hidden_single_col = HiddenSingleColumn::check(&state).0;
    let res_hidden_single_blk = HiddenSingleBlock::check(&state).0;
    let res_naked_single = NakedSingle::check(&state).0;
    let res_hidden_pair_row = HiddenPairRow::check(&state).0;
    let res_hidden_pair_col = HiddenPairColumn::check(&state).0;
    let res_hidden_pair_blk = HiddenPairBlock::check(&state).0;
    let res_naked_pair_row = NakedPairRow::check(&state).0;
    let res_naked_pair_col = NakedPairColumn::check(&state).0;
    let res_naked_pair_blk = NakedPairBlock::check(&state).0;
    let res_pointing = Pointing::check(&state).0;
    let res_claiming = Claiming::check(&state).0;
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
    println!("claiming: {:?}", res_claiming);
    let mut solver2 = AdvancedSolver::<FullState>::from(grid);
    solver2.have_unique_solution();
    println!("{}", solver2.difficulty());
    println!("{:?}", grid.0);
}
