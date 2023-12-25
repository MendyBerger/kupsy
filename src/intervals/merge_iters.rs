use alloc::vec::Vec;
use core::cmp::max;

use super::Interval;

/// # Complexity:
///
/// | Space    | Runtime              |
/// |----------|----------------------|
/// | O(a + b) | O(a + b)             |
///
/// Where a = len(a), b = len(b)
///
/// # Examples
///
/// ```
/// use kupsy::intervals::{Interval, merge_2_interval_iters};
///
/// let result = merge_2_interval_iters(
///     &vec![Interval {start: 1, end: 3}, Interval {start: 5, end: 7}],
///     &vec![Interval {start: 4, end: 6}, Interval {start: 8, end: 10}]
/// );
/// assert_eq!(
///     vec![Interval {start: 1, end: 3}, Interval {start: 4, end: 7}, Interval {start: 8, end: 10}],
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
/// def merge2IntervalIters(a: list[Interval], b: list[Interval]) -> list[Interval]:
///     output = []
///     ai, bi = 0, 0
///     while ai < len(a) and bi < len(b):
///         if output and a[ai].start <= output[-1].end:
///             output[-1].end = max(output[-1].end, a[ai].end)
///             ai += 1
///         elif output and b[bi].start <= output[-1].end:
///             output[-1].end = max(output[-1].end, b[bi].end)
///             bi += 1
///         else:
///             if a[ai].start < b[bi].start:
///                 output.append(a[ai])
///                 ai += 1
///             else:
///                 output.append(b[bi])
///                 bi += 1
///     while ai < len(a):
///         if output and a[ai].start <= output[-1].end:
///             output[-1].end = min(output[-1].end, a[ai].end)
///         else:
///             output.append(a[ai])
///         ai += 1
///     while bi < len(b):
///         if output and b[bi].start <= output[-1].end:
///             output[-1].end = min(output[-1].end, b[bi].end)
///         else:
///             output.append(b[bi])
///         bi += 1
///     return output
/// # result = merge2IntervalIters(
/// #     [Interval(1, 3), Interval(5, 7)],
/// #     [Interval(4, 6), Interval(8, 10)]
/// # )
/// # assert(
/// #     [Interval(1, 3), Interval(4, 7), Interval(8, 10)] == result
/// # )
/// # }
///
/// ```

pub fn merge_2_interval_iters<'a, I, T>(a: I, b: I) -> Vec<Interval<T>>
where
    I: IntoIterator<Item = &'a Interval<T>>,
    T: Copy + Ord + 'a,
{
    let mut a_iter = a.into_iter();
    let mut b_iter = b.into_iter();
    let mut output: Vec<Interval<T>> = Vec::new();
    let mut a_next = a_iter.next();
    let mut b_next = b_iter.next();
    while let (Some(a), Some(b)) = (a_next, b_next) {
        match output.last_mut() {
            Some(last) if a.start <= last.end => {
                last.end = max(last.end, a.end);
                a_next = a_iter.next();
            }
            Some(last) if b.start <= last.end => {
                last.end = max(last.end, b.end);
                b_next = b_iter.next();
            }
            _ => {
                if a.start < b.start {
                    output.push(*a);
                    a_next = a_iter.next();
                } else {
                    output.push(*b);
                    b_next = b_iter.next();
                }
            }
        }
    }
    while let Some(a) = a_next {
        match output.last_mut() {
            Some(last) if a.start <= last.end => {
                last.end = max(last.end, a.end);
            }
            _ => output.push(*a),
        }
        a_next = a_iter.next();
    }
    while let Some(b) = b_next {
        match output.last_mut() {
            Some(last) if b.start <= last.end => {
                last.end = max(last.end, b.end);
            }
            _ => output.push(*b),
        }
        b_next = b_iter.next();
    }

    output
}

pub fn merge_interval_iters<T>(_input: &Vec<Vec<Interval<T>>>) -> Vec<Interval<T>>
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
    fn test_empty_vecs() {
        let a = tiv![];
        let b = tiv![];
        let expected = tiv![];
        assert_eq!(merge_2_interval_iters(&a, &b), expected);
    }

    #[test]
    fn test_one_empty_vec() {
        let a = tiv![];
        let b = tiv![(1, 3)];
        let expected = tiv![(1, 3)];
        assert_eq!(merge_2_interval_iters(&a, &b), expected);
    }

    #[test]
    fn test_disjoint_intervals() {
        let a = tiv![(1, 3), (5, 7)];
        let b = tiv![(4, 6), (8, 10)];
        let expected = tiv![(1, 3), (4, 7), (8, 10)];
        assert_eq!(merge_2_interval_iters(&a, &b), expected);
    }

    #[test]
    fn test_overlapping_intervals() {
        let a = tiv![(1, 3), (6, 8)];
        let b = tiv![(2, 5), (7, 9)];
        let expected = tiv![(1, 5), (6, 9)];
        assert_eq!(merge_2_interval_iters(&a, &b), expected);
    }

    #[test]
    fn test_nested_intervals() {
        let a = tiv![(2, 4), (6, 8)];
        let b = tiv![(1, 7)];
        let expected = tiv![(1, 8)];
        assert_eq!(merge_2_interval_iters(&a, &b), expected);
    }

    #[test]
    fn test_adjacent_intervals() {
        let a = tiv![(1, 3), (5, 7)];
        let b = tiv![(3, 5)];
        let expected = tiv![(1, 7)];
        assert_eq!(merge_2_interval_iters(&a, &b), expected);
    }
}
