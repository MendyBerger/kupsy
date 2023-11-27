// TODO:
// - testing.
// - proper docs.

use core::cmp::Ordering;

use super::Interval;

/// Complexity:
/// O(n * log n) Runtime
/// O(?) space
/// Where n = ...
///
/// example...
///
// TODO: take an iterator, not a vec.
pub fn sort_by_start<T>(v: &mut Vec<Interval<T>>)
where
    T: Copy + Ord,
{
    // consider: should sort_unstable_by be used instead?
    v.sort_by(|a, b| {
        let mut result = a.start.cmp(&b.start);
        if let Ordering::Equal = result {
            result = a.end.cmp(&b.end);
        }
        result
    })
}
pub fn sort_by_end<T>(v: &mut Vec<Interval<T>>)
where
    T: Copy + Ord,
{
    // consider: should sort_unstable_by be used instead?
    v.sort_by(|a, b| {
        let mut result = a.end.cmp(&b.end);
        if let Ordering::Equal = result {
            result = a.end.cmp(&b.end);
        }
        result
    })
}
