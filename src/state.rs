pub trait State: From<[[i8; 9]; 9]> {
    fn cell_val(&self, r: usize, c: usize) -> i8;
    fn is_cell_empty(&self, r: usize, c: usize) -> bool;
    fn grid(&self) -> [[i8; 9]; 9];
}

pub trait TrackingCandidates: State {
    fn is_candidate_of(&self, r: usize, c: usize, num: i8) -> bool;
}

pub trait TrackingCandidateCountOfCell: State {
    fn candidate_cnt_of_cell(&self, r: usize, c: usize) -> i8;
    fn candidate_cnt_of_cell_in_row(&self, r: usize, c: usize) -> i8;
    fn candidate_cnt_of_cell_in_col(&self, c: usize, r: usize) -> i8;
    fn candidate_cnt_of_cell_in_blk(&self, b: usize, bidx: usize) -> i8;
}

pub trait TrackingCellCountOfCandidate: State {
    fn cell_cnt_of_candidate_in_row(&self, r: usize, num: i8) -> i8;
    fn cell_cnt_of_candidate_in_col(&self, c: usize, num: i8) -> i8;
    fn cell_cnt_of_candidate_in_blk(&self, b: usize, num: i8) -> i8;
}

pub trait Fillable: State {
    fn fill_cell(&mut self, r: usize, c: usize, num: i8);
    fn unfill_cell(&mut self, r: usize, c: usize);
}

pub trait CandidatesSettable: TrackingCandidates {
    fn remove_candidate_of_cell(&mut self, r: usize, c: usize, to_remove: i8);
    fn add_candidate_of_cell(&mut self, r: usize, c: usize, to_add: i8);
}

pub mod full_state;
pub mod simple_state;
