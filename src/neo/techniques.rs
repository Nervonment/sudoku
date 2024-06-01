use super::{
    puzzle::{
        Grid, TrackingCandidateCountOfGrid, TrackingCandidates, TrackingGridCountOfCandidate,
    },
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

pub fn hidden_single_row(
    puzzle: &(impl Grid + TrackingCandidates + TrackingGridCountOfCandidate),
) -> Option<(usize, usize, i8)> {
    hidden_single(
        puzzle,
        |p, r, num| p.grid_cnt_of_candidate_in_row(r, num),
        |r, c| (r, c),
    )
}

pub fn hidden_single_col(
    puzzle: &(impl Grid + TrackingCandidates + TrackingGridCountOfCandidate),
) -> Option<(usize, usize, i8)> {
    hidden_single(
        puzzle,
        |p, c, num| p.grid_cnt_of_candidate_in_col(c, num),
        |c, r| (r, c),
    )
}

pub fn hidden_single_blk(
    puzzle: &(impl Grid + TrackingCandidates + TrackingGridCountOfCandidate),
) -> Option<(usize, usize, i8)> {
    hidden_single(
        puzzle,
        |p, b, num| p.grid_cnt_of_candidate_in_blk(b, num),
        block_idx_2_coord,
    )
}

pub fn naked_single(
    puzzle: &(impl Grid + TrackingCandidates + TrackingCandidateCountOfGrid),
) -> Option<(usize, usize, i8)> {
    for r in 0..9 {
        for c in 0..9 {
            if puzzle.is_grid_empty(r, c) && puzzle.candidate_cnt_of_grid(r, c) == 1 {
                let num = (1..=9)
                    .filter(|num| puzzle.is_candidate_of(r, c, *num))
                    .next()
                    .unwrap();
                return Some((r, c, num));
            }
        }
    }
    None
}

fn hidden_pair<T, F1, F2>(
    puzzle: &T,
    grid_cnt_for_candidate: F1,
    coord_transform: F2,
) -> Option<((usize, usize), Vec<i8>, (usize, usize), Vec<i8>, i8, i8)>
where
    T: Grid + TrackingCandidates,
    F1: Fn(&T, usize, i8) -> i8,
    F2: Fn(usize, usize) -> (usize, usize),
{
    for i in 0..9 {
        let nums: Vec<i8> = (1..=9)
            .filter(|num| grid_cnt_for_candidate(puzzle, i, *num) == 2)
            .collect();
        for i1 in 0..nums.len() {
            for i2 in 0..i1 {
                let num1 = nums[i1];
                let num2 = nums[i2];
                if (0..9).all(|j| {
                    let (r, c) = coord_transform(i, j);
                    !puzzle.is_grid_empty(r, c)
                        || puzzle.is_candidate_of(r, c, num1) == puzzle.is_candidate_of(r, c, num2)
                }) {
                    let mut jiter = (0..9).filter(|j| {
                        let (r, c) = coord_transform(i, *j);
                        puzzle.is_grid_empty(r, c) && puzzle.is_candidate_of(r, c, nums[i1])
                    });
                    let j1 = jiter.next().unwrap();
                    let j2 = jiter.next().unwrap();
                    let (r1, c1) = coord_transform(i, j1);
                    let (r2, c2) = coord_transform(i, j2);
                    let removes_1: Vec<i8> = (1..=9)
                        .filter(|n| *n != num1 && *n != num2 && puzzle.is_candidate_of(r1, c1, *n))
                        .collect();
                    let removes_2: Vec<i8> = (1..=9)
                        .filter(|n| *n != num1 && *n != num2 && puzzle.is_candidate_of(r2, c2, *n))
                        .collect();
                    if !removes_1.is_empty() || !removes_2.is_empty() {
                        return Some((
                            (r1, c1),
                            removes_1,
                            (r2, c2),
                            removes_2,
                            nums[i1],
                            nums[i2],
                        ));
                    }
                }
            }
        }
    }
    None
}

pub fn hidden_pair_row(
    puzzle: &(impl Grid + TrackingCandidates + TrackingGridCountOfCandidate),
) -> Option<((usize, usize), Vec<i8>, (usize, usize), Vec<i8>, i8, i8)> {
    hidden_pair(
        puzzle,
        |p, r, num| p.grid_cnt_of_candidate_in_row(r, num),
        |r, c| (r, c),
    )
}

pub fn hidden_pair_col(
    puzzle: &(impl Grid + TrackingCandidates + TrackingGridCountOfCandidate),
) -> Option<((usize, usize), Vec<i8>, (usize, usize), Vec<i8>, i8, i8)> {
    hidden_pair(
        puzzle,
        |p, c, num| p.grid_cnt_of_candidate_in_col(c, num),
        |c, r| (r, c),
    )
}

pub fn hidden_pair_blk(
    puzzle: &(impl Grid + TrackingCandidates + TrackingGridCountOfCandidate),
) -> Option<((usize, usize), Vec<i8>, (usize, usize), Vec<i8>, i8, i8)> {
    hidden_pair(
        puzzle,
        |p, b, num| p.grid_cnt_of_candidate_in_blk(b, num),
        block_idx_2_coord,
    )
}

fn naked_pair<T, F>(
    puzzle: &T,
    coord_transform: F,
) -> Option<(
    (usize, usize),
    (usize, usize),
    i8,
    Vec<(usize, usize)>,
    i8,
    Vec<(usize, usize)>,
)>
where
    T: Grid + TrackingCandidates + TrackingCandidateCountOfGrid,
    F: Fn(usize, usize) -> (usize, usize),
{
    for i in 0..9 {
        let js: Vec<usize> = (0..9)
            .filter(|j| {
                let (r, c) = coord_transform(i, *j);
                puzzle.is_grid_empty(r, c) && puzzle.candidate_cnt_of_grid(r, c) == 2
            })
            .collect();
        for i1 in 0..js.len() {
            for i2 in 0..i1 {
                let j1 = js[i1];
                let j2 = js[i2];
                let (r1, c1) = coord_transform(i, j1);
                let (r2, c2) = coord_transform(i, j2);
                if (1..=9).all(|num| {
                    puzzle.is_candidate_of(r1, c1, num) == puzzle.is_candidate_of(r2, c2, num)
                }) {
                    let mut num_iter = (1..=9).filter(|num| puzzle.is_candidate_of(r1, c1, *num));
                    let num1 = num_iter.next().unwrap();
                    let num2 = num_iter.next().unwrap();
                    let removes_1: Vec<(usize, usize)> = (0..9)
                        .filter(|j| {
                            let (r, c) = coord_transform(i, *j);
                            *j != j1
                                && *j != j2
                                && puzzle.is_grid_empty(r, c)
                                && puzzle.is_candidate_of(r, c, num1)
                        })
                        .map(|j| coord_transform(i, j))
                        .collect();
                    let removes_2: Vec<(usize, usize)> = (0..9)
                        .filter(|j| {
                            let (r, c) = coord_transform(i, *j);
                            *j != j1
                                && *j != j2
                                && puzzle.is_grid_empty(r, c)
                                && puzzle.is_candidate_of(r, c, num2)
                        })
                        .map(|j| coord_transform(i, j))
                        .collect();
                    if !removes_1.is_empty() || !removes_2.is_empty() {
                        return Some(((r1, c1), (r2, c2), num1, removes_1, num2, removes_2));
                    }
                }
            }
        }
    }
    None
}

pub fn naked_pair_row(
    puzzle: &(impl Grid + TrackingCandidates + TrackingCandidateCountOfGrid),
) -> Option<(
    (usize, usize),
    (usize, usize),
    i8,
    Vec<(usize, usize)>,
    i8,
    Vec<(usize, usize)>,
)> {
    naked_pair(puzzle, |r, c| (r, c))
}

pub fn naked_pair_col(
    puzzle: &(impl Grid + TrackingCandidates + TrackingCandidateCountOfGrid),
) -> Option<(
    (usize, usize),
    (usize, usize),
    i8,
    Vec<(usize, usize)>,
    i8,
    Vec<(usize, usize)>,
)> {
    naked_pair(puzzle, |c, r| (r, c))
}

pub fn naked_pair_blk(
    puzzle: &(impl Grid + TrackingCandidates + TrackingCandidateCountOfGrid),
) -> Option<(
    (usize, usize),
    (usize, usize),
    i8,
    Vec<(usize, usize)>,
    i8,
    Vec<(usize, usize)>,
)> {
    naked_pair(puzzle, block_idx_2_coord)
}
