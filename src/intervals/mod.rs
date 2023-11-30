use core::ops::Sub;

mod insert;
mod intersection;
mod merge_iters;
mod merge_overlapping;
mod overlaps;
mod sizes;
mod sort;
#[cfg(test)]
mod testing;
mod vec;

pub use insert::*;
pub use intersection::*;
pub use merge_iters::*;
pub use merge_overlapping::*;
pub use overlaps::*;
pub use sizes::*;
pub use sort::*;
pub use vec::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Interval<T>
where
    T: Copy + Ord,
{
    pub start: T,
    pub end: T,
}

impl<T> Interval<T>
where
    T: Copy + Ord + Sub<Output = T>,
{
    pub fn length(&self) -> T {
        self.end - self.start
    }
}

impl<T> From<(T, T)> for Interval<T>
where
    T: Copy + Ord,
{
    fn from(value: (T, T)) -> Self {
        Self {
            start: value.0,
            end: value.1,
        }
    }
}

impl<T> From<Interval<T>> for (T, T)
where
    T: Copy + Ord,
{
    fn from(value: Interval<T>) -> (T, T) {
        (value.start, value.end)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum NeighborStrategy {
    #[default]
    Same,
    Distinct,
}
