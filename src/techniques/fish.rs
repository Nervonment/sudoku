use itertools::Itertools;

use crate::state::{State, TrackingCandidates, TrackingCellCountOfCandidate};

use super::{House, ReducingCandidates, ReducingCandidatesOption, Technique};

fn basic_fish_row_base<T>(state: &T, size: usize) -> Option<FishInfo>
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    for base_rows in (0..9).combinations(size) {
        'outer: for num in 1..=9 {
            for r in &base_rows {
                if state.cell_cnt_of_candidate_in_row(*r, num) < 2
                    || state.cell_cnt_of_candidate_in_row(*r, num) > size as i8
                {
                    continue 'outer;
                }
            }

            let cover_cols = (0..9)
                .filter(|c| {
                    base_rows
                        .iter()
                        .any(|r| state.is_cell_empty(*r, *c) && state.is_candidate_of(*r, *c, num))
                })
                .collect::<Vec<_>>();
            if cover_cols.len() != size {
                continue 'outer;
            }

            for c in &cover_cols {
                if state.cell_cnt_of_candidate_in_col(*c, num) < 2 {
                    continue 'outer;
                }
            }

            let overlap = base_rows
                .iter()
                .flat_map(|r| cover_cols.iter().map(move |c| (*r, *c)));

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
                    let overlap = overlap.clone();
                    (0..9).filter_map(move |r| {
                        if state.is_cell_empty(r, *c)
                            && state.is_candidate_of(r, *c, num)
                            && !overlap.clone().any(|coord| coord == (r, *c))
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
                overlap: overlap.collect_vec(),
                candidate: num,
                rem_cells: remove,
            });
        }
    }
    None
}

fn basic_fish_col_base<T>(state: &T, size: usize) -> Option<FishInfo>
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    for base_cols in (0..9).combinations(size) {
        'outer: for num in 1..=9 {
            for c in &base_cols {
                if state.cell_cnt_of_candidate_in_col(*c, num) < 2
                    || state.cell_cnt_of_candidate_in_col(*c, num) > size as i8
                {
                    continue 'outer;
                }
            }

            let cover_rows = (0..9)
                .filter(|r| {
                    base_cols
                        .iter()
                        .any(|c| state.is_cell_empty(*r, *c) && state.is_candidate_of(*r, *c, num))
                })
                .collect::<Vec<_>>();
            if cover_rows.len() != size {
                continue 'outer;
            }

            for r in &cover_rows {
                if state.cell_cnt_of_candidate_in_row(*r, num) < 2 {
                    continue 'outer;
                }
            }

            let overlap = base_cols
                .iter()
                .flat_map(|c| cover_rows.iter().map(move |r| (*r, *c)));

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
                    let overlap = overlap.clone();
                    (0..9).filter_map(move |c| {
                        if state.is_cell_empty(*r, c)
                            && state.is_candidate_of(*r, c, num)
                            && !overlap.clone().any(|coord| coord == (*r, c))
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
                overlap: overlap.collect_vec(),
                candidate: num,
                rem_cells: remove,
            });
        }
    }
    None
}

#[derive(Clone, Debug)]
pub struct FishInfo {
    pub size: usize,
    pub base_set: Vec<House>,
    pub cover_set: Vec<House>,
    pub overlap: Vec<(usize, usize)>,
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
        self.0 = basic_fish_row_base(state, 2);
        if self.0.is_some() {
            return;
        }
        self.0 = basic_fish_col_base(state, 2);
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
    fn option(&self) -> Option<ReducingCandidatesOption> {
        self.0
            .clone()
            .map(|info| ReducingCandidatesOption(vec![(info.rem_cells, vec![info.candidate])]))
    }
}

#[derive(Default)]
pub struct Swordfish(pub Option<FishInfo>);
impl<T> Technique<T> for Swordfish
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn analyze(&mut self, state: &T) {
        self.0 = basic_fish_row_base(state, 3);
        if self.0.is_some() {
            return;
        }
        self.0 = basic_fish_col_base(state, 3);
    }
    fn appliable(&self) -> bool {
        self.0.is_some()
    }
    fn score(&self) -> Option<f32> {
        if self.0.is_some() {
            return Some(3.8);
        }
        None
    }
}
impl<T> ReducingCandidates<T> for Swordfish
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn option(&self) -> Option<ReducingCandidatesOption> {
        self.0
            .clone()
            .map(|info| ReducingCandidatesOption(vec![(info.rem_cells, vec![info.candidate])]))
    }
}

#[derive(Default)]
pub struct Jellyfish(pub Option<FishInfo>);
impl<T> Technique<T> for Jellyfish
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn analyze(&mut self, state: &T) {
        self.0 = basic_fish_row_base(state, 4);
        if self.0.is_some() {
            return;
        }
        self.0 = basic_fish_col_base(state, 4);
    }
    fn appliable(&self) -> bool {
        self.0.is_some()
    }
    fn score(&self) -> Option<f32> {
        if self.0.is_some() {
            return Some(5.2);
        }
        None
    }
}
impl<T> ReducingCandidates<T> for Jellyfish
where
    T: State + TrackingCandidates + TrackingCellCountOfCandidate,
{
    fn option(&self) -> Option<ReducingCandidatesOption> {
        self.0
            .clone()
            .map(|info| ReducingCandidatesOption(vec![(info.rem_cells, vec![info.candidate])]))
    }
}
