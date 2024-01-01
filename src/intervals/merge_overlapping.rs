use alloc::vec::Vec;
use core::cmp::max;
use hashbrown::HashSet;

use super::{sort_by_start, Interval};

/// # Complexity:
///
/// | Space | Runtime |
/// |-------|---------|
/// | O(n)  | O(n)    |
///
/// Where n = len(input)
///
/// # Examples
///
/// ```
/// use kupsy::intervals::{Interval, merge_overlapping};
///
/// let mut input = vec![Interval {start: 1, end: 5}, Interval {start: 4, end: 7}, Interval {start: 8, end: 10}];
/// merge_overlapping(&mut input);
/// assert_eq!(
///     vec![Interval {start: 1, end: 7}, Interval {start: 8, end: 10}],
///     input
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
///
/// def mergeOverlapping(input: list[Interval]):
///     input.sort(key=lambda interval: interval.start)
///     result = []
///     for interval in input:
///         print(interval)
///         if result and interval.start <= result[-1].end:
///             result[-1].end = max(result[-1].end, interval.end)
///         else:
///             result.append(interval)
///     input.clear()
///     input.extend(result)
/// # input = [Interval(1, 5), Interval(4, 7), Interval(8, 10)]
/// # mergeOverlapping(input)
/// # assert(
/// #     [Interval(1, 7), Interval(8, 10)] == input
/// # )
/// # }
///
/// ```
/// assumptions:
/// Vec is sorted by Interval.start.
//
// Not taking a slice, as I don't think I can resize the underlying data-structure through a slice.
pub fn merge_overlapping<T>(input: &mut Vec<Interval<T>>)
where
    T: Copy + Ord,
{
    // might be able to do away with any memory allocations by doing everything in the retain callback. Will likely require unsafe code.
    let mut holes = HashSet::new();

    let mut last_valid: usize = 0;
    for i in 1..input.len() {
        let current = input[i];
        let last = &mut input[last_valid];
        // let last = input.last_mut().unwrap();
        if current.start <= last.end {
            last.end = max(last.end, current.end);
            holes.insert(i);
        } else {
            last_valid = i;
        }
    }

    let mut index: usize = 0;
    input.retain(|_| {
        let keep = !holes.contains(&index);
        index += 1;
        keep
    })
}

/// Sort, and merge overlapping [Interval]s in a <code>Vec\<[`Interval<T>`]></code>.
///
/// Combination of <code>[sort_by_start]</code> and <code>[merge_overlapping]</code>.
pub fn sort_and_merge_overlapping<T>(input: &mut Vec<Interval<T>>)
where
    T: Copy + Ord,
{
    sort_by_start(input);
    merge_overlapping(input)
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

    #[test]
    fn test_empty_vec() {
        let mut input = tiv![];
        merge_overlapping(&mut input);
        assert_eq!(input, tiv![]);
    }

    #[test]
    fn test_single_interval() {
        let mut input = tiv![(1, 3)];
        merge_overlapping(&mut input);
        assert_eq!(input, tiv![(1, 3)]);
    }

    #[test]
    fn test_disjoint_intervals() {
        let mut input = tiv![(1, 3), (5, 7)];
        merge_overlapping(&mut input);
        assert_eq!(input, tiv![(1, 3), (5, 7)]);
    }

    #[test]
    fn test_disjoint_intervals_unsorted() {
        let mut input = tiv![(5, 7), (1, 3)];
        sort_and_merge_overlapping(&mut input);
        assert_eq!(input, tiv![(1, 3), (5, 7)]);
    }

    #[test]
    fn test_overlapping_intervals() {
        let mut input = tiv![(1, 3), (2, 5), (4, 7), (6, 8)];
        merge_overlapping(&mut input);
        assert_eq!(input, tiv![(1, 8)]);
    }

    #[test]
    fn test_overlapping_intervals_unsorted() {
        let mut input = tiv![(1, 3), (2, 5), (6, 8), (4, 7)];
        sort_and_merge_overlapping(&mut input);
        assert_eq!(input, tiv![(1, 8)]);
    }

    #[test]
    fn test_nested_intervals() {
        let mut input = tiv![(1, 7), (2, 4), (6, 8)];
        merge_overlapping(&mut input);
        assert_eq!(input, tiv![(1, 8)]);
    }

    #[test]
    fn test_nested_intervals_unsorted() {
        let mut input = tiv![(2, 4), (6, 8), (1, 7)];
        sort_and_merge_overlapping(&mut input);
        assert_eq!(input, tiv![(1, 8)]);
    }

    #[test]
    fn test_adjacent_intervals() {
        let mut input = tiv![(1, 3), (4, 5), (5, 7)];
        merge_overlapping(&mut input);
        assert_eq!(input, tiv![(1, 3), (4, 7)]);
    }

    #[test]
    fn test_duplicate_intervals() {
        let mut input = tiv![(1, 3), (2, 5), (1, 3)];
        merge_overlapping(&mut input);
        assert_eq!(input, tiv![(1, 5)]);
    }

    #[test]
    fn test_large_input() {
        let mut input = (1..1000)
            .map(|i| (i * 2 - 1, i * 2 + 1).into())
            .collect::<Vec<_>>();
        merge_overlapping(&mut input);
        assert_eq!(input, tiv![(1, 1999)]);
    }
}
