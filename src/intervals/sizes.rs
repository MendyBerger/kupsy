use core::ops::Sub;

use alloc::vec::Vec;

use super::Interval;

pub fn get_length_of_each_interval<T>(intervals: &Vec<Interval<T>>) -> Vec<T>
where
    T: Copy + Ord + Sub<Output = T>,
{
    intervals.iter().map(|interval| interval.length()).collect()
}

/// Assumptions: Sorted, no overlaps.
pub fn get_gaps_intervals<T>(intervals: &Vec<Interval<T>>) -> Vec<Interval<T>>
where
    T: Copy + Ord,
{
    if intervals.is_empty() {
        return Vec::new();
    }
    let mut output: Vec<Interval<T>> = Vec::with_capacity(intervals.len() - 1);
    for i in 0..(intervals.len() - 1) {
        output.push(Interval {
            start: intervals[i].end,
            end: intervals[i + 1].start,
        });
    }
    output
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]

    use super::*;
    use crate::intervals::testing::tiv;

    #[test]
    fn empty_intervals() {
        let intervals: Vec<Interval<u32>> = tiv![];

        let gaps = get_gaps_intervals(&intervals);
        assert_eq!(gaps, tiv![]);
    }

    #[test]
    fn single_interval() {
        let intervals: Vec<Interval<u32>> = tiv![(1, 5)];

        let gaps = get_gaps_intervals(&intervals);
        assert_eq!(gaps, tiv![]);
    }

    #[test]
    fn non_overlapping_intervals() {
        let intervals: Vec<Interval<u32>> = tiv![(1, 3), (4, 6)];

        let gaps = get_gaps_intervals(&intervals);
        assert_eq!(gaps, tiv![(3, 4)]);
    }

    #[test]
    fn fully_non_overlapping_intervals() {
        let intervals: Vec<Interval<u32>> = tiv![(1, 5), (6, 10), (11, 15)];

        let gaps = get_gaps_intervals(&intervals);
        assert_eq!(gaps, tiv![(5, 6), (10, 11)]);
    }

    #[test]
    fn ten_intervals() {
        let intervals: Vec<Interval<u32>> = tiv![
            (1, 3),
            (4, 6),
            (7, 9),
            (10, 12),
            (13, 15),
            (16, 18),
            (19, 21),
            (22, 24),
            (25, 27),
            (28, 30)
        ];

        let gaps = get_gaps_intervals(&intervals);
        assert_eq!(
            gaps,
            tiv![
                (3, 4),
                (6, 7),
                (9, 10),
                (12, 13),
                (15, 16),
                (18, 19),
                (21, 22),
                (24, 25),
                (27, 28)
            ]
        );
    }

    #[test]
    fn fifteen_intervals() {
        let intervals: Vec<Interval<u32>> = tiv![
            (1, 3),
            (4, 6),
            (7, 9),
            (10, 12),
            (13, 15),
            (16, 18),
            (19, 21),
            (22, 24),
            (25, 27),
            (28, 30),
            (31, 33),
            (34, 36),
            (37, 39),
            (40, 42),
            (43, 45)
        ];

        let gaps = get_gaps_intervals(&intervals);
        assert_eq!(
            gaps,
            tiv![
                (3, 4),
                (6, 7),
                (9, 10),
                (12, 13),
                (15, 16),
                (18, 19),
                (21, 22),
                (24, 25),
                (27, 28),
                (30, 31),
                (33, 34),
                (36, 37),
                (39, 40),
                (42, 43)
            ]
        );
    }

    #[test]
    fn twenty_intervals() {
        let intervals: Vec<Interval<u32>> = tiv![
            (1, 3),
            (4, 6),
            (7, 9),
            (10, 12),
            (13, 15),
            (16, 18),
            (19, 21),
            (22, 24),
            (25, 27),
            (28, 30),
            (31, 33),
            (34, 36),
            (37, 39),
            (40, 42),
            (43, 45),
            (46, 48),
            (49, 51),
            (52, 54),
            (55, 57),
            (58, 60)
        ];

        let gaps = get_gaps_intervals(&intervals);
        assert_eq!(
            gaps,
            tiv![
                (3, 4),
                (6, 7),
                (9, 10),
                (12, 13),
                (15, 16),
                (18, 19),
                (21, 22),
                (24, 25),
                (27, 28),
                (30, 31),
                (33, 34),
                (36, 37),
                (39, 40),
                (42, 43),
                (45, 46),
                (48, 49),
                (51, 52),
                (54, 55),
                (57, 58)
            ]
        );
    }
}

// (26, 27),
// (34, 35),
// (42, 43),
// (50, 51),
// (58, 59),
// (66, 67),
// (74, 75),
// (82, 83),
// (90, 91),
// (98, 99),
