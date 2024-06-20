use itertools::Itertools;

use crate::{
    state::{State, TrackingCandidates, TrackingCellCountOfCandidate},
    utils::block_idx_2_coord,
};

use super::{House, ReducingCandidates, ReducingCandidatesOption, Technique};

pub fn overlap_region(
    (h1, h_idx1): (usize, usize),
    (h2, h_idx2): (usize, usize),
) -> Vec<(usize, usize)> {
    match h1 {
        0 => match h2 {
            0 => {
                if h_idx1 == h_idx2 {
                    (0..9).map(|c| (h_idx1, c)).collect()
                } else {
                    vec![]
                }
            }
            1 => vec![(h_idx1, h_idx2)],
            2 => {
                if h_idx2 / 3 == h_idx1 / 3 {
                    (0..3).map(|c0| (h_idx1, c0 + h_idx2 % 3 * 3)).collect()
                } else {
                    vec![]
                }
            }
            _ => vec![],
        },
        1 => match h2 {
            0 => overlap_region((h2, h_idx2), (h1, h_idx1)),
            1 => {
                if h_idx1 == h_idx2 {
                    (0..9).map(|r| (r, h_idx1)).collect()
                } else {
                    vec![]
                }
            }
            2 => {
                if h_idx2 % 3 == h_idx1 / 3 {
                    (0..3).map(|r0| (r0 + h_idx2 / 3 * 3, h_idx1)).collect()
                } else {
                    vec![]
                }
            }
            _ => vec![],
        },
        2 => match h2 {
            0..=1 => overlap_region((h2, h_idx2), (h1, h_idx1)),
            2 => {
                if h_idx1 == h_idx2 {
                    (0..9).map(|bidx| block_idx_2_coord(h_idx1, bidx)).collect()
                } else {
                    vec![]
                }
            }
            _ => vec![],
        },
        _ => vec![],
    }
}

pub fn fish<
    T,
    const BASE_ROW_COUNT: usize,
    const BASE_COL_COUNT: usize,
    const BASE_BLK_COUNT: usize,
    const COVER_ROW_COUNT: usize,
    const COVER_COL_COUNT: usize,
    const COVER_BLK_COUNT: usize,
>(
    state: &T,
) -> Option<FishInfo>
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    let row_count: usize = BASE_ROW_COUNT + COVER_ROW_COUNT;
    let col_count: usize = BASE_COL_COUNT + COVER_COL_COUNT;
    let blk_count: usize = BASE_BLK_COUNT + COVER_BLK_COUNT;

    let coord_transforms = [|r, c| (r, c), |c, r| (r, c), block_idx_2_coord];
    let cell_cnt_of_candidate: [&dyn Fn(&T, usize, i8) -> i8; 3] = [
        &TrackingCellCountOfCandidate::cell_cnt_of_candidate_in_row,
        &TrackingCellCountOfCandidate::cell_cnt_of_candidate_in_col,
        &TrackingCellCountOfCandidate::cell_cnt_of_candidate_in_blk,
    ];

    for num in 1..=9 {
        for rows in (0..9).combinations(row_count) {
            for cols in (0..9).combinations(col_count) {
                for blks in (0..9).combinations(blk_count) {
                    for base_rows in rows.iter().combinations(BASE_ROW_COUNT) {
                        let cover_rows = rows.iter().filter_map(|r| {
                            if base_rows.iter().any(|r1| **r1 == *r) {
                                Some((0, *r))
                            } else {
                                None
                            }
                        });
                        for base_cols in cols.iter().combinations(BASE_COL_COUNT) {
                            let cover_cols = cols.iter().filter_map(|c| {
                                if base_cols.iter().any(|c1| **c1 == *c) {
                                    Some((1, *c))
                                } else {
                                    None
                                }
                            });
                            'outer: for base_blks in blks.iter().combinations(BASE_BLK_COUNT) {
                                let cover_blks = blks.iter().filter_map(|b| {
                                    if base_blks.iter().any(|b1| **b1 == *b) {
                                        Some((2, *b))
                                    } else {
                                        None
                                    }
                                });

                                let base_houses = base_rows
                                    .iter()
                                    .map(|r| (0, **r))
                                    .chain(base_cols.iter().map(|c| (1, **c)))
                                    .chain(base_blks.iter().map(|b| (2, **b)));
                                let cover_houses = cover_rows
                                    .clone()
                                    .chain(cover_cols.clone())
                                    .chain(cover_blks);

                                for house in cover_houses.clone().chain(base_houses.clone()) {
                                    if cell_cnt_of_candidate[house.0](state, house.1, num) == 0 {
                                        continue 'outer;
                                    }
                                }

                                let overlap = base_houses.clone().flat_map(|base_house| {
                                    cover_houses.clone().flat_map(move |cover_house| {
                                        overlap_region(base_house, cover_house)
                                    })
                                });

                                for base_house in base_houses.clone() {
                                    for idx_in_h in 0..9 {
                                        let (r, c) =
                                            coord_transforms[base_house.0](base_house.1, idx_in_h);
                                        if state.is_cell_empty(r, c)
                                            && state.is_candidate_of(r, c, num)
                                            && !overlap.clone().any(|coord| coord == (r, c))
                                        {
                                            continue 'outer;
                                        }
                                    }
                                }

                                let remove: Vec<(usize, usize)> = cover_houses
                                    .clone()
                                    .flat_map(|cover_house| {
                                        let mut overlap = overlap.clone();
                                        (0..9).filter_map(move |idx_in_h| {
                                            let (r, c) = coord_transforms[cover_house.0](
                                                cover_house.1,
                                                idx_in_h,
                                            );
                                            if state.is_cell_empty(r, c)
                                                && state.is_candidate_of(r, c, num)
                                                && !overlap.any(|coord| coord == (r, c))
                                            {
                                                Some((r, c))
                                            } else {
                                                None
                                            }
                                        })
                                    })
                                    .collect();

                                if remove.is_empty() {
                                    continue 'outer;
                                }

                                let to_house = |(h, h_idx): (usize, usize)| match h {
                                    0 => House::Row(h_idx),
                                    1 => House::Column(h_idx),
                                    _ => House::Block(h_idx),
                                };

                                return Some(FishInfo {
                                    size: BASE_ROW_COUNT + BASE_COL_COUNT + BASE_BLK_COUNT,
                                    base_set: base_houses.map(to_house).collect(),
                                    cover_set: cover_houses.map(to_house).collect(),
                                    candidate: num,
                                    rem_cells: remove,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

pub fn basic_fish<
    T,
    const BASE_ROW_COUNT: usize,
    const BASE_COL_COUNT: usize,
    const COVER_ROW_COUNT: usize,
    const COVER_COL_COUNT: usize,
>(
    state: &T,
) -> Option<FishInfo>
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    let row_count: usize = BASE_ROW_COUNT + COVER_ROW_COUNT;
    let col_count: usize = BASE_COL_COUNT + COVER_COL_COUNT;

    let coord_transforms = [|r, c| (r, c), |c, r| (r, c)];
    let cell_cnt_of_candidate: [&dyn Fn(&T, usize, i8) -> i8; 2] = [
        &TrackingCellCountOfCandidate::cell_cnt_of_candidate_in_row,
        &TrackingCellCountOfCandidate::cell_cnt_of_candidate_in_col,
    ];

    for num in 1..=9 {
        for rows in (0..9).combinations(row_count) {
            for cols in (0..9).combinations(col_count) {
                for base_rows in rows.iter().combinations(BASE_ROW_COUNT) {
                    let cover_rows = rows.iter().filter_map(|r| {
                        if base_rows.iter().any(|r1| **r1 == *r) {
                            Some((0, *r))
                        } else {
                            None
                        }
                    });
                    'outer: for base_cols in cols.iter().combinations(BASE_COL_COUNT) {
                        let cover_cols = cols.iter().filter_map(|c| {
                            if base_cols.iter().any(|c1| **c1 == *c) {
                                Some((1, *c))
                            } else {
                                None
                            }
                        });

                        let base_houses = base_rows
                            .iter()
                            .map(|r| (0, **r))
                            .chain(base_cols.iter().map(|c| (1, **c)));
                        let cover_houses = cover_rows.clone().chain(cover_cols);

                        for house in cover_houses.clone().chain(base_houses.clone()) {
                            if cell_cnt_of_candidate[house.0](state, house.1, num) == 0 {
                                continue 'outer;
                            }
                        }

                        let overlap = base_houses.clone().flat_map(|base_house| {
                            cover_houses.clone().flat_map(move |cover_house| {
                                overlap_region(base_house, cover_house)
                            })
                        });

                        for base_house in base_houses.clone() {
                            for idx_in_h in 0..9 {
                                let (r, c) = coord_transforms[base_house.0](base_house.1, idx_in_h);
                                if state.is_cell_empty(r, c)
                                    && state.is_candidate_of(r, c, num)
                                    && !overlap.clone().any(|coord| coord == (r, c))
                                {
                                    continue 'outer;
                                }
                            }
                        }

                        let remove: Vec<(usize, usize)> = cover_houses
                            .clone()
                            .flat_map(|cover_house| {
                                let mut overlap = overlap.clone();
                                (0..9).filter_map(move |idx_in_h| {
                                    let (r, c) =
                                        coord_transforms[cover_house.0](cover_house.1, idx_in_h);
                                    if state.is_cell_empty(r, c)
                                        && state.is_candidate_of(r, c, num)
                                        && !overlap.any(|coord| coord == (r, c))
                                    {
                                        Some((r, c))
                                    } else {
                                        None
                                    }
                                })
                            })
                            .collect();

                        if remove.is_empty() {
                            continue 'outer;
                        }

                        let to_house = |(h, h_idx): (usize, usize)| match h {
                            0 => House::Row(h_idx),
                            1 => House::Column(h_idx),
                            _ => House::Block(h_idx),
                        };

                        return Some(FishInfo {
                            size: BASE_ROW_COUNT + BASE_COL_COUNT,
                            base_set: base_houses.map(to_house).collect(),
                            cover_set: cover_houses.map(to_house).collect(),
                            candidate: num,
                            rem_cells: remove,
                        });
                    }
                }
            }
        }
    }

    None
}

pub fn basic_fish_row_base<T>(
    state: &T,
    size: usize
) -> Option<FishInfo>
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    for num in 1..=9 {
        for base_rows in (0..9).combinations(size) {
            'outer: for cover_cols in (0..9).combinations(size) {
                for r in &base_rows {
                    if state.cell_cnt_of_candidate_in_row(*r, num) == 0 {
                        continue 'outer;
                    }
                }
                for c in &cover_cols {
                    if state.cell_cnt_of_candidate_in_col(*c, num) == 0 {
                        continue 'outer;
                    }
                }

                let overlap = base_rows.iter().flat_map(|r| {
                    cover_cols
                        .iter()
                        .flat_map(move |c| overlap_region((0, *r), (1, *c)))
                });

                for r in base_rows.iter() {
                    for c in 0..9 {
                        if state.is_cell_empty(*r, c)
                            && state.is_candidate_of(*r, c, num)
                            && !overlap.clone().any(|coord| coord == (*r, c))
                        {
                            continue 'outer;
                        }
                    }
                }

                let remove: Vec<(usize, usize)> = cover_cols
                    .iter()
                    .flat_map(|c| {
                        let mut overlap = overlap.clone();
                        (0..9).filter_map(move |r| {
                            if state.is_cell_empty(r, *c)
                                && state.is_candidate_of(r, *c, num)
                                && !overlap.any(|coord| coord == (r, *c))
                            {
                                Some((r, *c))
                            } else {
                                None
                            }
                        })
                    })
                    .collect();

                if remove.is_empty() {
                    continue 'outer;
                }

                return Some(FishInfo {
                    size,
                    base_set: base_rows.iter().map(|r| House::Row(*r)).collect(),
                    cover_set: cover_cols.iter().map(|c| House::Column(*c)).collect(),
                    candidate: num,
                    rem_cells: remove,
                });
            }
        }
    }
    None
}

pub fn basic_fish_col_base<T>(
    state: &T,
    size: usize
) -> Option<FishInfo>
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    for num in 1..=9 {
        for base_cols in (0..9).combinations(size) {
            'outer: for cover_rows in (0..9).combinations(size) {
                for c in &base_cols {
                    if state.cell_cnt_of_candidate_in_col(*c, num) == 0 {
                        continue 'outer;
                    }
                }
                for r in &cover_rows {
                    if state.cell_cnt_of_candidate_in_row(*r, num) == 0 {
                        continue 'outer;
                    }
                }

                let overlap = base_cols.iter().flat_map(|c| {
                    cover_rows
                        .iter()
                        .flat_map(move |r| overlap_region((0, *r), (1, *c)))
                });

                for c in base_cols.iter() {
                    for r in 0..9 {
                        if state.is_cell_empty(r, *c)
                            && state.is_candidate_of(r, *c, num)
                            && !overlap.clone().any(|coord| coord == (r, *c))
                        {
                            continue 'outer;
                        }
                    }
                }

                let remove: Vec<(usize, usize)> = cover_rows
                    .iter()
                    .flat_map(|r| {
                        let mut overlap = overlap.clone();
                        (0..9).filter_map(move |c| {
                            if state.is_cell_empty(*r, c)
                                && state.is_candidate_of(*r, c, num)
                                && !overlap.any(|coord| coord == (*r, c))
                            {
                                Some((*r, c))
                            } else {
                                None
                            }
                        })
                    })
                    .collect();

                if remove.is_empty() {
                    continue 'outer;
                }

                return Some(FishInfo {
                    size,
                    base_set: base_cols.iter().map(|c| House::Column(*c)).collect(),
                    cover_set: cover_rows.iter().map(|r| House::Row(*r)).collect(),
                    candidate: num,
                    rem_cells: remove,
                });
            }
        }
    }
    None
}

#[derive(Clone, Debug)]
pub struct FishInfo {
    pub size: usize,
    pub base_set: Vec<House>,
    pub cover_set: Vec<House>,
    pub candidate: i8,
    pub rem_cells: Vec<(usize, usize)>,
}

#[derive(Default)]
pub struct XWing(pub Option<FishInfo>);
impl<T> Technique<T> for XWing
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn analyze(&mut self, state: &T) {
        self.0 = basic_fish::<T, 2, 0, 0, 2>(state);
        // self.0 = fish::<T, 2, 0, 0, 0, 2, 0>(state);
        // self.0 = basic_fish_row_base(state, 2);
        if self.0.is_some() {
            return;
        }
        // self.0 = basic_fish_col_base(state, 2);
        self.0 = basic_fish::<T, 0, 2, 2, 0>(state);
        // self.0 = fish::<T, 0, 2, 0, 2, 0, 0>(state);
    }
    fn appliable(&self) -> bool {
        self.0.is_some()
    }
    fn score(&self) -> Option<f32> {
        if self.0.is_some() {
            return Some(3.2);
        }
        None
    }
}
impl<T> ReducingCandidates<T> for XWing
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn option(&self) -> Option<super::ReducingCandidatesOption> {
        self.0
            .clone()
            .map(|info| ReducingCandidatesOption(vec![(info.rem_cells, vec![info.candidate])]))
    }
}
