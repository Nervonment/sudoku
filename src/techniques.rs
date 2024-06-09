use super::state::State;

pub trait Technique<T>
where
    T: State,
{
    fn check(state: &T) -> Self;
    fn score(&self) -> f32;
}

pub struct DirectOption(pub usize, pub usize, pub i8);

pub trait Direct<T>: Technique<T> + Into<Option<DirectOption>>
where
    T: State,
{
    fn get_option_and_score(state: &T) -> Option<(DirectOption, f32)> {
        let res = Self::check(state);
        let score = res.score();
        res.into().map(|option| (option, score))
    }
}

pub struct ReducingCandidatesOption(pub Vec<(Vec<(usize, usize)>, Vec<i8>)>);

pub trait ReducingCandidates<T>: Technique<T> + Into<Option<ReducingCandidatesOption>>
where
    T: State,
{
    // 如果返回值为 Some(ReducingCandidatesOption(removes), score)，
    // 对于 removes 中的任意元素 (cells, nums)，
    // cells 与 nums 中元素的笛卡尔积为所有的移除对，
    // 即：可以从 cells 中的任意格的候选数中移除 nums 中的任意数
    fn get_option_and_score(state: &T) -> Option<(ReducingCandidatesOption, f32)> {
        let res = Self::check(state);
        let score = res.score();
        res.into().map(|v| (v, score))
    }
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
