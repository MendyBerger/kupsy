/// Test Interval Vec.
macro_rules! tiv {
    () => (
        {
            let v: Vec<$crate::intervals::Interval<u32>> = alloc::vec::Vec::new();
            v
        }
    );
    ($($interval:expr),* $(,)?) => (
        {
            let mut v: Vec<$crate::intervals::Interval<u32>> = alloc::vec::Vec::new();
            $(
                v.push($interval.into());
            )*
            v
        }
    );
}

pub(crate) use tiv;
