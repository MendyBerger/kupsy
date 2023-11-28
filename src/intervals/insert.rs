use core::cmp::{max, min};

use alloc::vec::Vec;

use super::{sort_by_start, Interval};

// Uses binary search
pub fn insert_interval_and_merge_pre_sorted<T>(_input: &mut Vec<Interval<T>>, _new: Interval<T>)
where
    T: Copy + Ord,
{
    todo!()
}

/// # Complexity:
///
/// | Space   | Runtime              |
/// |---------|----------------------|
/// | O(n)    | O(a log a + b log b) |
///
/// Where a = len(a), b = len(b), n = len(a) + len(b)
///
/// # Examples
///
/// ```
/// use kupsy::intervals::{Interval, insert_interval_and_merge_pre_sorted_linear};
///
/// let mut vec = vec![Interval{ start: 1, end: 2}, Interval {start: 5, end: 6}];
/// insert_interval_and_merge_pre_sorted_linear(
///     &mut vec,
///     Interval {
///         start: 0,
///         end: 1
///     },
/// );
/// assert_eq!(
///     vec![Interval{ start: 0, end: 2}, Interval {start: 5, end: 6}],
///     vec
/// );
/// ```
///
/// In python:
/// ```
/// # inline_python::python! {
/// # class Interval:
/// #   start: int
/// #   end: int
/// #   def __init__(self, start: int, end: int):
/// #       self.start = start
/// #       self.end = end
/// #   def __eq__(self, other):
/// #     return self.start == other.start and self.end == other.end
/// #
/// def insertIntervalAndMergePreSortedLinear(intervals: list[Interval], new: Interval) -> list[Interval]:
///     pos = len(intervals)
///     for i, current in enumerate(intervals):
///         if current.end >= new.start:
///             pos = i
///             break
///     replace_count = 0
///     for current in intervals[pos:]:
///         if current.start <= new.end:
///             replace_count += 1
///         else:
///             break
///     if pos < len(intervals) and replace_count > 0:
///         new.start = min(new.start, intervals[pos].start)
///         if pos + replace_count - 1 < len(intervals):
///             new.end = max(new.end, intervals[pos + replace_count - 1].end)
///     return intervals[:pos] + [new] + intervals[pos+replace_count:]
/// #
/// # result = insertIntervalAndMergePreSortedLinear(
/// #     [Interval(1, 2), Interval(3, 4), Interval(6, 7), Interval(10, 11), Interval(13, 15)],
/// #     Interval(4, 8)
/// # )
/// # assert(
/// #     [Interval(1, 2), Interval(3, 8), Interval(10, 11), Interval(13, 15)] == result
/// # )
/// # }
///
/// ```
///
// TODO: take an iterator, not a vec?
#[deprecated(
    note = "Use insert_interval_and_merge_pre_sorted instead. It uses binary search instead of linear search"
)]
pub fn insert_interval_and_merge_pre_sorted_linear<T>(
    intervals: &mut Vec<Interval<T>>,
    mut new: Interval<T>,
) where
    T: Copy + Ord,
{
    // TODO: change to binary search, improve runtime to O(log n)
    let mut pos = intervals.len();
    for (i, current) in intervals.iter().enumerate() {
        if current.end >= new.start {
            pos = i;
            break;
        }
    }
    let mut replace_count = 0;
    for current in intervals[pos..].iter() {
        if current.start <= new.end {
            replace_count += 1;
        } else {
            break;
        }
    }
    if pos < intervals.len() && replace_count > 0 {
        new.start = min(new.start, intervals[pos].start);
        if pos + replace_count - 1 < intervals.len() {
            new.end = max(new.end, intervals[pos + replace_count - 1].end);
        }
    }
    if pos < intervals.len() {
        intervals.splice(pos..pos + replace_count, [new]);
    } else {
        intervals.push(new);
    }
}

pub fn sort_and_insert_interval_and_merge<T>(mut vec: &mut Vec<Interval<T>>, interval: Interval<T>)
where
    T: Copy + Ord,
{
    sort_by_start(&mut vec);
    insert_interval_and_merge_pre_sorted(vec, interval)
}

#[deprecated(
    note = "Use sort_and_insert_interval_and_merge instead. It uses binary search instead of linear search"
)]
pub fn sort_and_insert_interval_and_merge_linear<T>(
    mut vec: &mut Vec<Interval<T>>,
    interval: Interval<T>,
) where
    T: Copy + Ord,
{
    sort_by_start(&mut vec);
    #[allow(deprecated)]
    insert_interval_and_merge_pre_sorted_linear(vec, interval)
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]

    use super::*;
    use crate::intervals::testing::tiv;

    #[test]
    fn simple_a() {
        let mut vec = tiv![(1, 2), (5, 6)];
        insert_interval_and_merge_pre_sorted_linear(&mut vec, (0, 1).into());
        assert_eq!(tiv![(0, 2), (5, 6)], vec);
    }

    #[test]
    fn simple_b() {
        let mut vec = tiv![(2, 4), (8, 9)];
        insert_interval_and_merge_pre_sorted_linear(&mut vec, (2, 6).into());
        assert_eq!(tiv![(2, 6), (8, 9)], vec);
    }

    #[test]
    fn simple_c() {
        let mut vec = tiv![(1, 2), (3, 4), (6, 7), (10, 11), (13, 15)];
        insert_interval_and_merge_pre_sorted_linear(&mut vec, (4, 8).into());
        assert_eq!(tiv![(1, 2), (3, 8), (10, 11), (13, 15)], vec);
    }
    #[test]
    fn empty_vec() {
        let mut vec = tiv![];
        insert_interval_and_merge_pre_sorted_linear(&mut vec, (4, 8).into());
        assert_eq!(tiv![(4, 8)], vec);
    }

    #[test]
    fn unsorted() {
        let mut vec = tiv![(12, 20), (2, 3), (5, 10)];
        sort_and_insert_interval_and_merge_linear(&mut vec, (4, 8).into());
        assert_eq!(tiv![(2, 3), (4, 10), (12, 20)], vec);
    }
}
