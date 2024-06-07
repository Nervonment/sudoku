use super::state::State;

pub trait Technique<T>
where
    T: State,
{
    fn check(state: &T) -> Self;
    fn score() -> f32;
}

pub trait Direct<T>: Technique<T> + Into<Option<(usize, usize, i8)>>
where
    T: State,
{
    fn fillable(state: &T) -> (Option<(usize, usize, i8)>, f32) {
        (Self::check(state).into(), Self::score())
    }
}

pub trait ReducingCandidates<T>:
    Technique<T> + Into<Option<Vec<(Vec<(usize, usize)>, Vec<i8>)>>>
where
    T: State,
{
    // 如果返回值为 Some(removes)，
    // 对于 removes 中的任意元素 (cells, nums)，
    // cells 与 nums 中元素的笛卡尔积为所有的移除对，
    // 即：可以从 cells 中的任意格的候选数中移除 nums 中的任意数
    fn reducible(state: &T) -> (Option<Vec<(Vec<(usize, usize)>, Vec<i8>)>>, f32) {
        (Self::check(state).into(), Self::score())
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
