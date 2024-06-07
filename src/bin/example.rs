use sudoku::{
    generator::random_sudoku_puzzle,
    solver::{advanced::AdvancedSolver, stochastic::StochasticSolver, Grader, Solver},
    state::{full_state::FullState, simple_state::SimpleState},
    techniques::{
        hidden_subsets::{HiddenPairBlock, HiddenPairColumn, HiddenPairRow},
        locked_candidates::Pointing,
        naked_subsets::{NakedPairBlock, NakedPairColumn, NakedPairRow},
        singles::{HiddenSingleBlock, HiddenSingleColumn, HiddenSingleRow, NakedSingle},
        Technique,
    },
};

fn main() {
    let grid = random_sudoku_puzzle::<StochasticSolver<SimpleState>, AdvancedSolver<FullState>, f32>(
        50, 0.0, 75.0,
    );
    let puzzle = FullState::from(grid);
    println!("{}", grid);
    let res_hidden_single_row = HiddenSingleRow::check(&puzzle).0;
    let res_hidden_single_col = HiddenSingleColumn::check(&puzzle).0;
    let res_hidden_single_blk = HiddenSingleBlock::check(&puzzle).0;
    let res_naked_single = NakedSingle::check(&puzzle).0;
    let res_hidden_pair_row = HiddenPairRow::check(&puzzle).0;
    let res_hidden_pair_col = HiddenPairColumn::check(&puzzle).0;
    let res_hidden_pair_blk = HiddenPairBlock::check(&puzzle).0;
    let res_naked_pair_row = NakedPairRow::check(&puzzle).0;
    let res_naked_pair_col = NakedPairColumn::check(&puzzle).0;
    let res_naked_pair_blk = NakedPairBlock::check(&puzzle).0;
    let res_pointing = Pointing::check(&puzzle).0;
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
    let mut solver2 = AdvancedSolver::<FullState>::from(grid);
    solver2.have_unique_solution();
    println!("{}", solver2.difficulty());
}
