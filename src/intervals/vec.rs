use alloc::vec::Vec;
use core::ops::{Index, Sub};

use super::{
    get_gaps_intervals, get_length_of_each_interval, insert_interval_and_merge_pre_sorted,
    intersection_interval_iters, merge_interval_iters, sort_and_merge_overlapping, Interval,
};

/// Simple Vec wrapper that ensures it's always sorted, and contains no overlaps
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
        Self { v: Vec::new() }
    }

    pub fn new_intersection(intervals: &Vec<Vec<Interval<T>>>) -> Self {
        Self {
            v: intersection_interval_iters(intervals),
        }
    }

    pub fn insert(&mut self, interval: Interval<T>) {
        insert_interval_and_merge_pre_sorted(&mut self.v, interval);
    }

    pub fn remove(&mut self, index: usize) -> Interval<T> {
        self.v.remove(index)
    }

    pub fn inner_vec(&self) -> &Vec<Interval<T>> {
        &self.v
    }

    pub unsafe fn inner_vec_mut(&mut self) -> &mut Vec<Interval<T>> {
        &mut self.v
    }

    pub fn gaps(&self) -> Self {
        get_gaps_intervals(&self.v).into()
    }

    pub fn clear(&mut self) {
        self.v.clear()
    }

    pub fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    pub fn len(&self) -> usize {
        self.v.len()
    }

    pub fn pop(&mut self) -> Option<Interval<T>> {
        self.v.pop()
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Interval<T>) -> bool,
    {
        self.v.retain(f)
    }

    pub fn truncate(&mut self, len: usize) {
        self.v.truncate(len)
    }
}

impl<T> IntervalVec<T>
where
    T: Copy + Ord + Sub<Output = T>,
{
    pub fn interval_lengths(&self) -> Vec<T> {
        get_length_of_each_interval(&self.v)
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

impl<T> From<Vec<Interval<T>>> for IntervalVec<T>
where
    T: Copy + Ord,
{
    fn from(mut value: Vec<Interval<T>>) -> Self {
        sort_and_merge_overlapping(&mut value);

        Self { v: value }
    }
}

impl<T> From<Vec<Vec<Interval<T>>>> for IntervalVec<T>
where
    T: Copy + Ord,
{
    fn from(mut value: Vec<Vec<Interval<T>>>) -> Self {
        let value = merge_interval_iters(&mut value);
        Self { v: value }
    }
}
