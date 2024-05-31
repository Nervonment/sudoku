use rand::random;

use crate::{
    game::generator::random_sudoku_puzzle_easy,
    neo::{
        puzzle::{SudokuPuzzle, TrackingCandidates},
        utils::{block_idx_2_coord, coord_2_block_idx},
    },
};

use super::puzzle::Fillable;

#[test]
fn sudoku_puzzle() {
    for _ in 0..100 {
        let puzzle = random_sudoku_puzzle_easy();
        let mut puzzle = SudokuPuzzle::new(puzzle);

        let mut moves = vec![];
        for _ in 0..10 {
            let (mut r, mut c) = (random::<usize>() % 9, random::<usize>() % 9);
            while puzzle.grid_val(r, c) > 0 {
                (r, c) = (random::<usize>() % 9, random::<usize>() % 9);
            }
            let num = (random::<u8>() % 9 + 1) as i8;
            moves.push((r, c));
            puzzle.fill_grid(r, c, num);
        }
        while !moves.is_empty(){
            let (r,c) = moves.pop().unwrap();
            puzzle.unfill_grid(r, c);
        }

        for r in 0..9 {
            for c in 0..9 {
                let (b, bidx) = coord_2_block_idx(r, c);
                // 如果格 (r,c) 已经填上数 num，那么同行、同列和同宫的空格子的候选数都不包含 num
                let num = puzzle.grid_val(r, c);
                if num > 0 {
                    for i in 0..9 {
                        if i != c && puzzle.grid_val(r, i) == 0 {
                            assert!(!puzzle.is_candidate_of(r, i, num));
                        }
                        if i != r && puzzle.grid_val(i, c) == 0 {
                            assert!(!puzzle.is_candidate_of(i, c, num));
                        }
                        if i != bidx {
                            let (r1, c1) = block_idx_2_coord(b, bidx);
                            if puzzle.grid_val(r1, c1) == 0 {
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
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_for_grid_in_row(r, c));
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_for_grid_in_col(c, r));
                    assert_eq!(candidate_cnt, puzzle.candidate_cnt_for_grid_in_blk(b, bidx));
                }
            }
        }

        for num in 1..=9 {
            for r in 0..9 {
                // grid_cnt 是第 r 行中候选数列表包含 num 的空格子数
                let mut grid_cnt = 0;
                for c in 0..9 {
                    grid_cnt +=
                        (puzzle.is_candidate_of(r, c, num) && puzzle.grid_val(r, c) == 0) as i8;
                }
                assert_eq!(grid_cnt, puzzle.grid_cnt_for_candidate_in_row(r, num));
            }
            for c in 0..9 {
                // grid_cnt 是第 c 列中候选数列表包含 num 的空格子数
                let mut grid_cnt = 0;
                for r in 0..9 {
                    grid_cnt +=
                        (puzzle.is_candidate_of(r, c, num) && puzzle.grid_val(r, c) == 0) as i8;
                }
                assert_eq!(grid_cnt, puzzle.grid_cnt_for_candidate_in_col(c, num));
            }
            for b in 0..9 {
                // grid_cnt 是第 b 宫中候选数列表包含 num 的空格子数
                let mut grid_cnt = 0;
                for bidx in 0..9 {
                    let (r, c) = block_idx_2_coord(b, bidx);
                    grid_cnt +=
                        (puzzle.is_candidate_of(r, c, num) && puzzle.grid_val(r, c) == 0) as i8;
                }
                assert_eq!(grid_cnt, puzzle.grid_cnt_for_candidate_in_blk(b, num));
            }
        }
    }
}
