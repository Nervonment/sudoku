use super::state::State;

pub trait Solver {
    fn any_solution(&mut self) -> Option<[[i8; 9]; 9]>;
    fn solution_cnt(&mut self) -> u32;
    fn have_unique_solution(&mut self) -> bool;
}

pub trait Grader<T: PartialOrd> {
    fn difficulty(&self) -> T;
}

fn next_blank(mut row: usize, mut col: usize, state: &impl State) -> Option<(usize, usize)> {
    while row < 9 && !state.is_cell_empty(row, col) {
        if col == 8 {
            col = 0;
            row += 1;
        } else {
            col += 1
        }
    }
    if row == 9 {
        return None;
    }
    Some((row, col))
}

pub mod advanced;
pub mod naive;
pub mod stochastic;
