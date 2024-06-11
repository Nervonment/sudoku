use std::fmt::Display;

use super::state::State;

pub trait Technique<T>
where
    T: State,
{
    fn analyze(&mut self, state: &T);
    fn appliable(&self) -> bool;
    fn score(&self) -> Option<f32>;
}

#[derive(Clone)]
pub struct DirectOption(pub usize, pub usize, pub i8);

impl Display for DirectOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fill R{}C{} with {}", self.0 + 1, self.1 + 1, self.2)
    }
}

pub trait Direct<T>: Technique<T>
where
    T: State,
{
    fn option(&self) -> Option<DirectOption>;
}

#[derive(Clone)]
pub struct ReducingCandidatesOption(pub Vec<(Vec<(usize, usize)>, Vec<i8>)>);

impl Display for ReducingCandidatesOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let clone = self.0.clone();
        for (i, (rem_cells, rem_nums)) in clone.iter().enumerate() {
            write!(f, "remove ")?;
            for (i, num) in rem_nums.iter().enumerate() {
                write!(f, "{}", num)?;
                if i + 1 < rem_nums.len() {
                    write!(f, ", ")?;
                }
            }
            write!(f, " from ")?;
            for (i, (r, c)) in rem_cells.iter().enumerate() {
                write!(f, "R{}C{}", r + 1, c + 1)?;
                if i + 1 < rem_cells.len() {
                    write!(f, ", ")?;
                }
            }
            if i + 1 < clone.len() {
                write!(f, "; ")?;
            }
        }
        Ok(())
    }
}

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

pub mod hidden_subsets;
pub mod locked_candidates;
pub mod naked_subsets;
pub mod singles;
