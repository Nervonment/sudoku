use super::puzzle::{Grid, TrackingCandidates};

pub fn hidden_single(puzzle: &(impl TrackingCandidates + Grid)) -> Option<(usize, usize, i8)> {
    for r in 0..9 {
        for num in 1..=9 {
            if puzzle.grid_cnt_for_candidate_in_row(r, num) == 1 {
                let c = (0..9)
                    .filter(|c: &usize| puzzle.is_candidate_of(r, *c, num))
                    .next()
                    .unwrap();
                if puzzle.is_grid_empty(r, c) {
                    return Some((r, c, num));
                }
            }
        }
    }
    None
}
