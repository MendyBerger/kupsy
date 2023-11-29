use alloc::vec::Vec;
use core::cmp::{max, min};

use super::Interval;

/// Complexity:
/// Runtime: O(n + m)
/// where n = a.len() and m = b.len()
///
/// space: O(n + m)
/// where n = a.len() and m = b.len()
///
/// assumptions:
/// Vec is sorted and non-overlapping.
pub fn intersection_2_interval_iters<T>(
    a: &Vec<Interval<T>>,
    b: &Vec<Interval<T>>,
) -> Vec<Interval<T>>
where
    T: Copy + Ord,
{
    let mut output = Vec::new();
    let mut ai = 0;
    let mut bi = 0;
    while ai < a.len() && bi < b.len() {
        if a[ai].end <= b[bi].start {
            ai += 1;
        } else if b[bi].end <= a[ai].start {
            bi += 1;
        } else {
            output.push(Interval {
                start: max(a[ai].start, b[bi].start),
                end: min(a[ai].end, b[bi].end),
            });
            if a[ai].end < b[bi].end {
                ai += 1;
            } else {
                bi += 1;
            }
        }
    }

    output
}

pub fn intersection_interval_iters<T>(_input: &Vec<Vec<Interval<T>>>) -> Vec<Interval<T>>
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
    fn empty_intervals() {
        let a = tiv![];
        let b = tiv![];

        let result = intersection_2_interval_iters(&a, &b);
        assert_eq!(result, tiv![]);
    }

    #[test]
    fn one_empty_one_nonempty() {
        let a = tiv![];
        let b = tiv![(1, 5)];

        let result = intersection_2_interval_iters(&a, &b);
        assert_eq!(result, tiv![]);
    }

    #[test]
    fn one_nonempty_one_empty() {
        let a = tiv![(1, 5)];
        let b = tiv![];

        let result = intersection_2_interval_iters(&a, &b);
        assert_eq!(result, tiv![]);
    }

    #[test]
    fn disjoint_intervals() {
        let a = tiv![(1, 3)];
        let b = tiv![(4, 6)];

        let result = intersection_2_interval_iters(&a, &b);
        assert_eq!(result, tiv![]);
    }

    #[test]
    fn overlapping_intervals() {
        let a = tiv![(1, 5)];
        let b = tiv![(3, 7)];

        let result = intersection_2_interval_iters(&a, &b);
        assert_eq!(result, tiv![(3, 5)]);
    }

    #[test]
    fn multiple_overlapping_intervals() {
        let a = tiv![(1, 5), (4, 8)];
        let b = tiv![(3, 7)];

        let result = intersection_2_interval_iters(&a, &b);
        assert_eq!(result, tiv![(3, 5), (4, 7)]);
    }

    #[test]
    fn touching_but_not_overlapping() {
        let a = tiv![(0, 2)];
        let b = tiv![(2, 3)];

        let result = intersection_2_interval_iters(&a, &b);
        assert_eq!(result, tiv![]);
    }
}
