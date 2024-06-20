use sudoku::{
    generator::{random_sudoku_puzzle_hard, random_sudoku_puzzle_normal},
    solver::{advanced::AdvancedSolver, Grader, Solver},
    state::{full_state::FullState, CandidatesSettable},
    techniques::{
        fish::fish,
        hidden_subsets::{HiddenPair, HiddenPairBlock, HiddenPairColumn, HiddenPairRow},
        locked_candidates::{Claiming, Pointing},
        naked_subsets::{NakedPair, NakedPairBlock, NakedPairColumn, NakedPairRow, NakedSubset},
        singles::{
            HiddenSingle, HiddenSingleBlock, HiddenSingleColumn, HiddenSingleRow, NakedSingle,
        },
        Direct, ReducingCandidates,
    },
    Grid,
};

fn main() {
    // let mut grid = random_sudoku_puzzle_hard();
    let grid = Grid([
        [1, 7, 0, 2, 8, 0, 0, 0, 9],
        [8, 2, 0, 9, 5, 0, 1, 7, 0],
        [0, 0, 5, 7, 6, 1, 2, 8, 0],
        [0, 0, 0, 8, 4, 6, 7, 0, 1],
        [6, 0, 8, 1, 7, 2, 9, 0, 0],
        [7, 4, 1, 3, 9, 5, 6, 2, 8],
        [0, 6, 9, 5, 0, 7, 8, 0, 0],
        [0, 1, 7, 0, 3, 8, 0, 9, 0],
        [0, 8, 0, 0, 0, 9, 0, 0, 7],
    ]);

    let mut state = FullState::from(grid);

    state.remove_candidate_of_cell(3, 0, 5);

    fish(&state, 4, 3, 0, 0, 0, 3, 0);
    fish(&state, 4, 0, 3, 0, 3, 0, 0);
    // while !fish(&state, 4, 2, 0, 0, 0, 2, 0)
    //     && !fish(&state, 4, 0, 2, 0, 2, 0, 0)
    //     && !fish(&state, 4, 3, 0, 0, 0, 3, 0)
    //     && !fish(&state, 4, 0, 3, 0, 3, 0, 0)
    // {
    //     grid = random_sudoku_puzzle_hard();
    //     state = FullState::from(grid);
    // }
    println!("The sudoku puzzle: ");
    println!("{}", grid);
    println!();

    let direct_techniques: [(&mut dyn Direct<FullState>, &str); 5] = [
        (&mut HiddenSingle::default(), "HiddenSingle"),
        (&mut HiddenSingleBlock::default(), "HiddenSingleBlock"),
        (&mut HiddenSingleRow::default(), "HiddenSingleRow"),
        (&mut HiddenSingleColumn::default(), "HiddenSingleColumn"),
        (&mut NakedSingle::default(), "NakedSingle"),
    ];

    println!("Direct techniques appliable: ");
    for (technique, label) in direct_techniques {
        technique.analyze(&state);
        if technique.appliable() {
            println!("{} - {}", label, technique.option().unwrap());
        }
    }
    println!();

    let reducing_techniques: [(&mut dyn ReducingCandidates<FullState>, &str); 11] = [
        (&mut Pointing::default(), "Pointing"),
        (&mut Claiming::default(), "Claiming"),
        (&mut NakedPair::default(), "NakedPair"),
        (&mut NakedPairBlock::default(), "NakedPairBlock"),
        (&mut NakedPairRow::default(), "NakedPairRow"),
        (&mut NakedPairColumn::default(), "NakedPairColumn"),
        (&mut HiddenPair::default(), "HiddenPair"),
        (&mut HiddenPairBlock::default(), "HiddenPairBlock"),
        (&mut HiddenPairRow::default(), "HiddenPairRow"),
        (&mut HiddenPairColumn::default(), "HiddenPairColumn"),
        (&mut NakedSubset::default(), "NakedSubset"),
    ];

    println!("Reducing-candidates techniques appliable: ");
    for (technique, label) in reducing_techniques {
        technique.analyze(&state);
        if technique.appliable() {
            println!("{} - {}", label, technique.option().unwrap());
        }
    }
    println!();

    let mut solver = AdvancedSolver::<FullState>::from(grid);
    assert!(solver.have_unique_solution());
    println!("The difficulty of the puzzle: {}", solver.difficulty());
    println!();

    println!("The puzzle as [[i8; 9]; 9]: ");
    println!("{:?}", grid.0);
}
