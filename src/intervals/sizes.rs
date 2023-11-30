use core::ops::Sub;

use alloc::vec::Vec;

use super::Interval;

pub fn get_length_of_each_interval<T>(intervals: &Vec<Interval<T>>) -> Vec<T>
where
    T: Copy + Ord + Sub<Output = T>,
{
    intervals.iter().map(|interval| interval.length()).collect()
}

pub fn get_gaps_intervals<T>(_intervals: &Vec<Interval<T>>) -> Vec<Interval<T>>
where
    T: Copy + Ord,
{
    todo!()
}
