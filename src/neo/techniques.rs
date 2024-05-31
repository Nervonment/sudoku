use super::{
    puzzle::{Grid, TrackingCandidates},
    utils::block_idx_2_coord,
};

fn hidden_single<T, F1, F2>(
    puzzle: &T,
    grid_cnt_for_candidate: F1,
    coord_transform: F2,
) -> Option<(usize, usize, i8)>
where
    T: Grid + TrackingCandidates,
    F1: Fn(&T, usize, i8) -> i8,
    F2: Fn(usize, usize) -> (usize, usize),
{
    for i in 0..9 {
        for num in 1..=9 {
            if grid_cnt_for_candidate(puzzle, i, num) == 1 {
                let j = (0..9)
                    .filter(|j: &usize| {
                        let (r, c) = coord_transform(i, *j);
                        puzzle.is_candidate_of(r, c, num)
                    })
                    .next()
                    .unwrap();
                let (r, c) = coord_transform(i, j);
                if puzzle.is_grid_empty(r, c) {
                    return Some((r, c, num));
                }
            }
        }
    }
    None
}

pub fn hidden_single_row(puzzle: &(impl Grid + TrackingCandidates)) -> Option<(usize, usize, i8)> {
    hidden_single(
        puzzle,
        |p, r, num| p.grid_cnt_for_candidate_in_row(r, num),
        |r, c| (r, c),
    )
}

pub fn hidden_single_col(puzzle: &(impl Grid + TrackingCandidates)) -> Option<(usize, usize, i8)> {
    hidden_single(
        puzzle,
        |p, c, num| p.grid_cnt_for_candidate_in_col(c, num),
        |c, r| (r, c),
    )
}

pub fn hidden_single_blk(puzzle: &(impl Grid + TrackingCandidates)) -> Option<(usize, usize, i8)> {
    hidden_single(
        puzzle,
        |p, b, num| p.grid_cnt_for_candidate_in_blk(b, num),
        block_idx_2_coord,
    )
}
