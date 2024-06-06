pub trait State: From<[[i8; 9]; 9]> {
    fn grid_val(&self, r: usize, c: usize) -> i8;
    fn is_grid_empty(&self, r: usize, c: usize) -> bool;
    fn board(&self) -> [[i8; 9]; 9];
}

pub trait TrackingCandidates: State {
    fn is_candidate_of(&self, r: usize, c: usize, num: i8) -> bool;
}

pub trait TrackingCandidateCountOfGrid: State {
    fn candidate_cnt_of_grid(&self, r: usize, c: usize) -> i8;
    fn candidate_cnt_of_grid_in_row(&self, r: usize, c: usize) -> i8;
    fn candidate_cnt_of_grid_in_col(&self, c: usize, r: usize) -> i8;
    fn candidate_cnt_of_grid_in_blk(&self, b: usize, bidx: usize) -> i8;
}

pub trait TrackingGridCountOfCandidate: State {
    fn grid_cnt_of_candidate_in_row(&self, r: usize, num: i8) -> i8;
    fn grid_cnt_of_candidate_in_col(&self, c: usize, num: i8) -> i8;
    fn grid_cnt_of_candidate_in_blk(&self, b: usize, num: i8) -> i8;
}

pub trait Fillable: State {
    fn fill_grid(&mut self, r: usize, c: usize, num: i8);
    fn unfill_grid(&mut self, r: usize, c: usize);
}

pub trait CandidatesSettable: TrackingCandidates {
    fn remove_candidate_of_grid(&mut self, r: usize, c: usize, to_remove: i8);
    fn add_candidate_of_grid(&mut self, r: usize, c: usize, to_add: i8);
}

pub mod full_state;
pub mod simple_state;
