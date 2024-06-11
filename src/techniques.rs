use super::state::State;

pub trait Technique<T>
where
    T: State,
{
    fn analyze(&mut self, state: &T);
    fn appliable(&self) -> bool;
    fn score(&self) -> Option<f32>;
}

pub struct DirectOption(pub usize, pub usize, pub i8);

pub trait Direct<T>: Technique<T>
where
    T: State,
{
    fn option(&self) -> Option<DirectOption>;
}

pub struct ReducingCandidatesOption(pub Vec<(Vec<(usize, usize)>, Vec<i8>)>);

pub trait ReducingCandidates<T>: Technique<T>
where
    T: State,
{
    fn option(&self) -> Option<ReducingCandidatesOption>;
}

#[derive(Clone, Copy, Debug)]
pub enum House {
    Row(usize),
    Column(usize),
    Block(usize),
}

// pub mod hidden_subsets;
// pub mod locked_candidates;
// pub mod naked_subsets;
pub mod singles;
