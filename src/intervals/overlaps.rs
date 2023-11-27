use super::Interval;

// Consider: should there be a count_overlaps? How would counts work?

/// does not merge
pub fn remove_overlaps<T>(_input: &mut Vec<Interval<T>>)
where
    T: Copy + Ord,
{
    todo!()
}

pub fn has_overlaps<T>(_input: &Vec<Interval<T>>) -> bool
where
    T: Copy + Ord,
{
    todo!()
}
pub fn has_overlaps_interval_iters<T>(_input: &Vec<Interval<T>>) -> bool
where
    T: Copy + Ord,
{
    todo!()
}

pub fn max_overlaps<T>(_input: &Vec<Interval<T>>) -> u32
where
    T: Copy + Ord,
{
    todo!()
}

pub fn max_overlaps_interval_iters<T>(_input: &Vec<Vec<Interval<T>>>) -> u32
where
    T: Copy + Ord,
{
    todo!()
}
