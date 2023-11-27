use core::ops::Index;

use super::Interval;

pub struct IntervalVec<T>
where
    T: Copy + Ord,
{
    v: Vec<Interval<T>>,
}
impl<T> IntervalVec<T>
where
    T: Copy + Ord,
{
    pub fn new() -> Self {
        Self { v: vec![] }
    }
}

impl<T> Index<usize> for IntervalVec<T>
where
    T: Copy + Ord,
{
    type Output = Interval<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}
