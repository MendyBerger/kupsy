use alloc::vec::Vec;
use core::cmp::max;

use super::{sort_by_start, Interval};

pub fn sort_and_merge_overlapping<T>(_input: &mut Vec<Interval<T>>)
where
    T: Copy + Ord,
{
    todo!()
}

// Takes mutable references because of sorting.
pub fn sort_and_merge_overlapping_from_2_interval_iters<T>(
    mut a: &mut Vec<Interval<T>>,
    mut b: &mut Vec<Interval<T>>,
) -> Vec<Interval<T>>
where
    T: Copy + Ord,
{
    sort_by_start(&mut a);
    sort_by_start(&mut b);
    merge_overlapping_from_2_interval_iters_pre_sorted(a, b)
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
/// use kupsy::intervals::{Interval, merge_overlapping_from_2_interval_iters_pre_sorted};
///
/// let result = merge_overlapping_from_2_interval_iters_pre_sorted(
///     &mut vec![Interval {start: 0, end: 1}],
///     &mut vec![Interval {start: 1, end: 2}, Interval {start: 5, end: 6}]
/// );
/// assert_eq!(
///     vec![Interval {start: 0, end: 2}, Interval {start: 5, end: 6}],
///     result
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
/// def mergeOverlappingFrom2IntervalListsPreSorted(a: list[Interval], b: list[Interval]) -> list[Interval]:
///     output = []
///     ai, bi = 0, 0
///     while ai < len(a) or bi < len(b):
///         if output and ai < len(a) and a[ai].start <= output[-1].end:
///             output[-1].end = max(output[-1].end, a[ai].end)
///             ai += 1
///         elif output and bi < len(b) and b[bi].start <= output[-1].end:
///             output[-1].end = max(output[-1].end, b[bi].end)
///             bi += 1
///         elif bi >= len(b) or (ai < len(a) and a[ai].start < b[bi].start):
///             output.append(a[ai])
///             ai += 1
///         else:
///             output.append(b[bi])
///             bi += 1
///     return output
/// # result = mergeOverlappingFrom2IntervalListsPreSorted(
/// #     [Interval(0, 1)],
/// #     [Interval(1, 2), Interval(5, 6)]
/// # )
/// # assert(
/// #     [Interval(0, 2), Interval(5, 6)] == result
/// # )
/// # }
///
/// ```
///
// TODO: take an iterator, not a vec.
pub fn merge_overlapping_from_2_interval_iters_pre_sorted<T>(
    a: &Vec<Interval<T>>,
    b: &Vec<Interval<T>>,
) -> Vec<Interval<T>>
where
    T: Copy + Ord,
{
    // ensure sorted?
    let mut output: Vec<Interval<T>> = Vec::new();

    let mut ai = 0;
    let mut bi = 0;
    while ai < a.len() || bi < b.len() {
        match output.last_mut() {
            Some(last) if ai < a.len() && a[ai].start <= last.end => {
                last.end = max(last.end, a[ai].end);
                ai += 1;
            }
            Some(last) if bi < b.len() && b[bi].start <= last.end => {
                last.end = max(last.end, b[bi].end);
                bi += 1;
            }
            _ => {
                if bi >= b.len() || (ai < a.len() && a[ai].start <= b[bi].start) {
                    output.push(a[ai]);
                    ai += 1;
                } else {
                    output.push(b[bi]);
                    bi += 1;
                }
            }
        }
    }

    output
}

pub fn sort_and_merge_overlapping_from_interval_iters<T>(
    interval_iters: &mut Vec<Vec<Interval<T>>>,
) -> Vec<Interval<T>>
where
    T: Copy + Ord,
{
    interval_iters
        .iter_mut()
        .for_each(|intervals| sort_by_start(intervals));

    merge_overlapping_from_interval_iters_pre_sorted(interval_iters)
}

pub fn merge_overlapping_from_interval_iters_pre_sorted<T>(
    _interval_iters: &Vec<Vec<Interval<T>>>,
) -> Vec<Interval<T>>
where
    T: Copy + Ord,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intervals::testing::tiv;

    #[test]
    fn simple_333() {
        let result = merge_overlapping_from_2_interval_iters_pre_sorted(
            &tiv![(0, 1)],
            &tiv![(1, 2), (5, 6)],
        );
        assert_eq!(tiv![(0, 2), (5, 6)], result);
    }

    #[test]
    fn simple() {
        let result = merge_overlapping_from_2_interval_iters_pre_sorted(
            &tiv![(0, 1)],
            &tiv![(1, 2), (5, 6)],
        );
        assert_eq!(tiv![(0, 2), (5, 6)], result);
    }
    #[test]
    fn all_overlap() {
        let result = merge_overlapping_from_2_interval_iters_pre_sorted(
            &tiv![],
            &tiv![(1, 3), (2, 3), (3, 8), (5, 9)],
        );
        assert_eq!(tiv![(1, 9)], result);
    }
    #[test]
    fn no_overlaps() {
        let result = merge_overlapping_from_2_interval_iters_pre_sorted(
            &tiv![],
            &tiv![(1, 2), (3, 4), (5, 6)],
        );
        assert_eq!(tiv![(1, 2), (3, 4), (5, 6)], result);
    }
    #[test]
    fn a_empty() {
        let result = merge_overlapping_from_2_interval_iters_pre_sorted(
            &tiv![],
            &tiv![(1, 2), (2, 3), (5, 6)],
        );
        assert_eq!(tiv![(1, 3), (5, 6)], result);
    }

    #[test]
    fn b_empty() {
        let result = merge_overlapping_from_2_interval_iters_pre_sorted(
            &tiv![(1, 2), (2, 3), (5, 6)],
            &tiv![],
        );
        assert_eq!(tiv![(1, 3), (5, 6)], result);
    }

    #[test]
    fn both_empty() {
        let result = merge_overlapping_from_2_interval_iters_pre_sorted(&tiv![], &tiv![]);
        assert_eq!(tiv![], result);
    }

    #[test]
    fn unsorted() {
        let result = sort_and_merge_overlapping_from_2_interval_iters(
            &mut tiv![(12, 20), (2, 3), (5, 6)],
            &mut tiv![(8, 11), (7, 8)],
        );
        assert_eq!(tiv![(2, 3), (5, 6), (7, 11), (12, 20)], result);
    }
}
