use rand::random;

use crate::{
    generator::{
        random_sudoku_puzzle_easy, random_sudoku_puzzle_extraeasy, random_sudoku_puzzle_extrahard,
        random_sudoku_puzzle_hard, random_sudoku_puzzle_normal, random_sudoku_puzzle_ultimate,
    },
    judge::judge_sudoku,
    solver::{advanced::AdvancedSolver, stochastic::StochasticSolver, Solver},
    state::{
        full_state::FullState, simple_state::SimpleState, CandidatesSettable, Fillable, State,
        TrackingCandidateCountOfCell, TrackingCandidates, TrackingCellCountOfCandidate,
    },
    techniques::fish::overlap_region,
    utils::{block_idx_2_coord, coord_2_block_idx},
};

#[test]
fn sudoku_state() {
    for _ in 0..100 {
        let puzzle = random_sudoku_puzzle_easy();
        let mut puzzle = FullState::from(puzzle);

        let mut moves = vec![];
        for _ in 0..10 {
            let (mut r, mut c) = (random::<usize>() % 9, random::<usize>() % 9);
            while !puzzle.is_cell_empty(r, c) {
                (r, c) = (random::<usize>() % 9, random::<usize>() % 9);
            }
            let num = (random::<u8>() % 9 + 1) as i8;
            moves.push((r, c));
            puzzle.fill_cell(r, c, num);
        }
        while !moves.is_empty() {
            let (r, c) = moves.pop().unwrap();
            puzzle.unfill_cell(r, c);
        }

        for r in 0..9 {
            for c in 0..9 {
                let (b, bidx) = coord_2_block_idx(r, c);
                // 如果格 (r,c) 已经填上数 num，那么同行、同列和同宫的空格子的候选数都不包含 num
                let num = puzzle.cell_val(r, c);
                if num > 0 {
                    for i in 0..9 {
                        if i != c && puzzle.cell_val(r, i) == 0 {
                            assert!(!puzzle.is_candidate_of(r, i, num));
                        }
                        if i != r && puzzle.cell_val(i, c) == 0 {
                            assert!(!puzzle.is_candidate_of(i, c, num));
                        }
                        if i != bidx {
                            let (r1, c1) = block_idx_2_coord(b, bidx);
                            if puzzle.cell_val(r1, c1) == 0 {
                                assert!(!puzzle.is_candidate_of(r1, c1, num));
                            }
                        }
                    }
                }
                // 如果格 (r,c) 未填数，那么他的候选数数量等于候选数数量
                else {
                    let mut candidate_cnt = 0;
                    for num in 1..=9 {
                        candidate_cnt += puzzle.is_candidate_of(r, c, num) as i8;
                    }
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_of_cell_in_row(r, c));
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_of_cell_in_col(c, r));
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_of_cell_in_blk(b, bidx));
                }
            }
        }

        for _ in 0..10 {
            let (mut r, mut c) = (random::<usize>() % 9, random::<usize>() % 9);
            while !puzzle.is_cell_empty(r, c) {
                (r, c) = (random::<usize>() % 9, random::<usize>() % 9);
            }
            let num = (random::<u8>() % 9 + 1) as i8;
            puzzle.remove_candidate_of_cell(r, c, num);
        }

        for r in 0..9 {
            for c in 0..9 {
                let (b, bidx) = coord_2_block_idx(r, c);
                if puzzle.is_cell_empty(r, c) {
                    let mut candidate_cnt = 0;
                    for num in 1..=9 {
                        candidate_cnt += puzzle.is_candidate_of(r, c, num) as i8;
                    }
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_of_cell_in_row(r, c));
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_of_cell_in_col(c, r));
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_of_cell_in_blk(b, bidx));
                }
            }
        }

        for num in 1..=9 {
            for r in 0..9 {
                // cell_cnt 是第 r 行中候选数列表包含 num 的空格子数
                let mut cell_cnt = 0;
                for c in 0..9 {
                    cell_cnt +=
                        (puzzle.is_candidate_of(r, c, num) && puzzle.cell_val(r, c) == 0) as i8;
                }
                assert_eq!(cell_cnt, puzzle.cell_cnt_of_candidate_in_row(r, num));
            }
            for c in 0..9 {
                // cell_cnt 是第 c 列中候选数列表包含 num 的空格子数
                let mut cell_cnt = 0;
                for r in 0..9 {
                    cell_cnt +=
                        (puzzle.is_candidate_of(r, c, num) && puzzle.cell_val(r, c) == 0) as i8;
                }
                assert_eq!(cell_cnt, puzzle.cell_cnt_of_candidate_in_col(c, num));
            }
            for b in 0..9 {
                // cell_cnt 是第 b 宫中候选数列表包含 num 的空格子数
                let mut cell_cnt = 0;
                for bidx in 0..9 {
                    let (r, c) = block_idx_2_coord(b, bidx);
                    cell_cnt +=
                        (puzzle.is_candidate_of(r, c, num) && puzzle.cell_val(r, c) == 0) as i8;
                }
                assert_eq!(cell_cnt, puzzle.cell_cnt_of_candidate_in_blk(b, num));
            }
        }
    }
}

// #[test]
// fn techniques_single() {
//     for _ in 0..100 {
//         let puzzle = random_sudoku_puzzle_normal();
//         let mut puzzle = FullState::from(puzzle);
//         let res_hidden_single_row = hidden_single_row(&puzzle);
//         let res_hidden_single_col = hidden_single_col(&puzzle);
//         let res_hidden_single_blk = hidden_single_blk(&puzzle);
//         let res_naked_single = naked_single(&puzzle);
//         let singles = [
//             res_hidden_single_row,
//             res_hidden_single_col,
//             res_hidden_single_blk,
//             res_naked_single,
//         ];
//         for single in singles {
//             if single.is_some() {
//                 let (r, c, num) = single.unwrap();
//                 puzzle.fill_cell(r, c, num);
//                 assert!(judge_sudoku(&puzzle.grid()).0);
//                 puzzle.unfill_cell(r, c);
//             }
//         }
//     }
// }

// #[test]
// fn techniques_pair() {
//     for _ in 0..10 {
//         let mut res_hidden_pair_row = None;
//         let mut puzzle = random_sudoku_puzzle_normal();
//         while res_hidden_pair_row.is_none() {
//             puzzle = random_sudoku_puzzle_normal();
//             let puzzle = FullState::from(puzzle);
//             res_hidden_pair_row = hidden_pair_row(&puzzle);
//         }
//         let mut puzzle = FullState::from(puzzle);
//         let ((r1, c1), _, (r2, c2), _, num1, num2) = res_hidden_pair_row.clone().unwrap();
//         let nums: Vec<i8> = (1..=9).filter(|n| *n != num1 && *n != num2).collect();
//         for num in &nums {
//             puzzle.remove_candidate_of_cell(r1, c1, *num);
//             puzzle.remove_candidate_of_cell(r2, c2, *num);
//         }
//         let res_hidden_pair_row_1 = hidden_pair_row(&puzzle);
//         assert_ne!(res_hidden_pair_row, res_hidden_pair_row_1);
//     }
// }
#[test]
fn generate_sudoku() {
    println!("generating extraeasy puzzle");
    random_sudoku_puzzle_extraeasy();
    println!("generating easy puzzle");
    random_sudoku_puzzle_easy();
    println!("generating normal puzzle");
    random_sudoku_puzzle_normal();
    println!("generating hard puzzle");
    random_sudoku_puzzle_hard();
    println!("generating extrahard puzzle");
    random_sudoku_puzzle_extrahard();
    println!("generating ultimate puzzle");
    random_sudoku_puzzle_ultimate();
}

#[test]
fn sudoku_solver() {
    for _ in 0..50 {
        let puzzle = random_sudoku_puzzle_ultimate();
        let mut solver1 = StochasticSolver::<SimpleState>::from(puzzle);
        let mut solver2 = AdvancedSolver::<FullState>::from(puzzle);
        assert!(solver1.have_unique_solution());
        assert!(solver2.have_unique_solution());
        let solution1 = solver1.any_solution().unwrap();
        let solution2 = solver2.any_solution().unwrap();
        assert!(judge_sudoku(&solution1).1);
        assert!(judge_sudoku(&solution2).1);
    }
}

#[test]
fn overlap_region_test() {
    // row and columns
    for r in 0..9 {
        for c in 0..9 {
            let region1 = overlap_region((0, r), (1, c));
            let region2 = overlap_region((1, c), (0, r));
            for c1 in 0..9 {
                for r1 in 0..9 {
                    if (r1, c) == (r, c1) {
                        assert!(
                            region1
                                .iter()
                                .find(|(r0, c0)| (*r0, *c0) == (r1, c))
                                .is_some(),
                            "row {} & column {}, region: {:?}",
                            r,
                            c,
                            region1
                        );
                        assert!(
                            region2
                                .iter()
                                .find(|(r0, c0)| (*r0, *c0) == (r1, c))
                                .is_some(),
                            "row {} & column {}, region: {:?}",
                            r,
                            c,
                            region2
                        );
                    }
                }
            }
        }
    }

    // rows and blocks
    for r in 0..9 {
        for b in 0..9 {
            let region1 = overlap_region((0, r), (2, b));
            let region2 = overlap_region((2, b), (0, r));
            for c1 in 0..9 {
                for bidx1 in 0..9 {
                    let (r2, c2) = block_idx_2_coord(b, bidx1);
                    if (r, c1) == (r2, c2) {
                        assert!(
                            region1
                                .iter()
                                .find(|(r0, c0)| (*r0, *c0) == (r, c1))
                                .is_some(),
                            "row {} & block {}, region: {:?}",
                            r,
                            b,
                            region1
                        );
                        assert!(
                            region2
                                .iter()
                                .find(|(r0, c0)| (*r0, *c0) == (r, c1))
                                .is_some(),
                            "row {} & block {}, region: {:?}",
                            r,
                            b,
                            region2
                        );
                    }
                }
            }
        }
    }

    // columns and blocks
    for c in 0..9 {
        for b in 0..9 {
            let region1 = overlap_region((1, c), (2, b));
            let region2 = overlap_region((2, b), (1, c));
            for r1 in 0..9 {
                for bidx1 in 0..9 {
                    let (r2, c2) = block_idx_2_coord(b, bidx1);
                    if (r1, c) == (r2, c2) {
                        assert!(
                            region1
                                .iter()
                                .find(|(r0, c0)| (*r0, *c0) == (r1, c))
                                .is_some(),
                            "column {} & block {}, region: {:?}",
                            c,
                            b,
                            region1
                        );
                        assert!(
                            region2
                                .iter()
                                .find(|(r0, c0)| (*r0, *c0) == (r1, c))
                                .is_some(),
                            "column {} & block {}, region: {:?}",
                            c,
                            b,
                            region2
                        );
                    }
                }
            }
        }
    }
}
