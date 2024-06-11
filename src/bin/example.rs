use sudoku::{
    generator::random_sudoku_puzzle_normal,
    solver::{advanced::AdvancedSolver, Grader, Solver},
    state::full_state::FullState,
    techniques::{
        hidden_subsets::{HiddenPairBlock, HiddenPairColumn, HiddenPairRow},
        locked_candidates::{Claiming, Pointing},
        naked_subsets::{NakedPairBlock, NakedPairColumn, NakedPairRow},
        singles::{HiddenSingleBlock, HiddenSingleColumn, HiddenSingleRow, NakedSingle},
        Direct, ReducingCandidates,
    },
};

fn main() {
    let grid = random_sudoku_puzzle_normal();
    println!("The sudoku puzzle: ");
    println!("{}", grid);
    println!();

    let state = FullState::from(grid);

    let direct_techniques: [(&mut dyn Direct<FullState>, &str); 4] = [
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

    let reducing_techniques: [(&mut dyn ReducingCandidates<FullState>, &str); 8] = [
        (&mut Pointing::default(), "Pointing"),
        (&mut Claiming::default(), "Claiming"),
        (&mut NakedPairBlock::default(), "NakedPairBlock"),
        (&mut NakedPairRow::default(), "NakedPairRow"),
        (&mut NakedPairColumn::default(), "NakedPairColumn"),
        (&mut HiddenPairBlock::default(), "HiddenPairBlock"),
        (&mut HiddenPairRow::default(), "HiddenPairRow"),
        (&mut HiddenPairColumn::default(), "HiddenPairColumn"),
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
