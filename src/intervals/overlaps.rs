use alloc::vec::Vec;

use crate::ds::MinHeap;

use super::Interval;

// Consider: should there be a count_overlaps? How would counts work?

/// does not merge
pub fn remove_overlaps<T>(_intervals: &mut Vec<Interval<T>>)
where
    T: Copy + Ord,
{
    todo!()
}

pub fn clip_overlaps<T>(_intervals: &mut Vec<Interval<T>>)
where
    T: Copy + Ord,
{
    todo!()
}

pub fn has_overlaps<T>(_intervals: &Vec<Interval<T>>) -> bool
where
    T: Copy + Ord,
{
    todo!()
}
pub fn has_overlaps_interval_iters<T>(_intervals: &Vec<Interval<T>>) -> bool
where
    T: Copy + Ord,
{
    todo!()
}

/// Get the maximum depth of the overlaps.
///
/// Complexity:
/// Runtime O(n * log n)
/// Space O(n)
/// Where n = intervals.len()
///
/// Assumption: Vec is sorted by start.
pub fn max_overlaps_depth<T>(intervals: &Vec<Interval<T>>) -> u32
where
    T: Copy + Ord,
{
    let mut heap = MinHeap::new();
    for interval in intervals {
        if let Some(first_to_finish) = heap.peek() {
            if first_to_finish < &interval.start {
                heap.pop();
            }
        }
        heap.push(interval.end);
    }

    heap.len() as u32
}

pub fn max_overlaps_interval_iters<T>(_intervals: &Vec<Vec<Interval<T>>>) -> u32
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
        let intervals: Vec<Interval<u32>> = tiv![];

        let max_overlaps_depth = max_overlaps_depth(&intervals);
        assert_eq!(max_overlaps_depth, 0);
    }

    #[test]
    fn single_interval() {
        let intervals: Vec<Interval<u32>> = tiv![(1, 5)];

        let max_overlaps_depth = max_overlaps_depth(&intervals);
        assert_eq!(max_overlaps_depth, 1);
    }

    #[test]
    fn non_overlapping_intervals() {
        let intervals: Vec<Interval<u32>> = tiv![(1, 3), (4, 6)];

        let max_overlaps_depth = max_overlaps_depth(&intervals);
        assert_eq!(max_overlaps_depth, 1);
    }

    #[test]
    fn partially_overlapping_intervals() {
        let intervals: Vec<Interval<u32>> = tiv![(1, 3), (2, 4), (4, 5)];

        let max_overlaps_depth = max_overlaps_depth(&intervals);
        assert_eq!(max_overlaps_depth, 2);
    }

    #[test]
    fn fully_overlapping_intervals() {
        let intervals: Vec<Interval<u32>> = tiv![(1, 5), (2, 5), (3, 5)];

        let max_overlaps_depth = max_overlaps_depth(&intervals);
        assert_eq!(max_overlaps_depth, 3);
    }

    #[test]
    fn complex_interval_set() {
        let intervals: Vec<Interval<u32>> = tiv![(1, 10), (2, 3), (3, 7), (4, 8), (5, 9)];

        let max_overlaps_depth = max_overlaps_depth(&intervals);
        assert_eq!(max_overlaps_depth, 4);
    }
}
