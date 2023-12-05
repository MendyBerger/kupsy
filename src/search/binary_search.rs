use core::cmp::Ordering;

pub fn binary_search<T>(slice: &[T], target: &T) -> Option<usize>
where
    T: Ord,
{
    let mut left = 0_usize;
    let mut right = slice.len() as isize - 1;
    while left as isize <= right {
        let middle = ((right as usize - left) / 2) + left;
        if &slice[middle] < target {
            left = middle + 1;
        } else if &slice[middle] > target {
            right = middle as isize - 1;
        } else {
            return Some(middle);
        }
    }
    None
}

pub fn binary_search_with<T, F>(slice: &[T], f: F) -> Option<usize>
where
    T: Ord,
    F: Fn(&T) -> Ordering,
{
    let mut left = 0_isize;
    let mut right = slice.len() as isize - 1;
    while left <= right {
        let middle = (((right - left) / 2) + left) as usize;
        match f(&slice[middle]) {
            Ordering::Less => {
                left = middle as isize + 1;
            }
            Ordering::Greater => {
                right = middle as isize - 1;
            }
            Ordering::Equal => {
                return Some(middle);
            }
        };
    }
    None
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]

    use super::*;
    use alloc::vec;

    #[test]
    fn simple() {
        let vec = vec![3, 11, 30, 31, 34, 38, 69, 72, 80, 82, 85];
        let res = binary_search(&vec, &31);
        assert_eq!(res, Some(3));
    }

    #[test]
    fn simple_with() {
        let vec = vec![3, 11, 30, 31, 34, 38, 69, 72, 80, 82, 85];
        let res = binary_search_with(&vec, |a| a.cmp(&31));
        assert_eq!(res, Some(3));
    }

    #[test]
    fn simple_slice() {
        let vec = vec![3, 11, 30, 31, 34, 38, 69, 72, 80, 82, 85];
        let res = binary_search(&vec[3..], &31);
        assert_eq!(res, Some(0));
    }

    #[test]
    fn empty_slice_not_found() {
        let slice: &[i32] = &[];
        let target = 5;
        assert_eq!(binary_search(slice, &target), None);
    }

    #[test]
    fn single_element_found() {
        let slice = &[5];
        let target = 5;
        assert_eq!(binary_search(slice, &target), Some(0));
    }

    #[test]
    fn single_element_not_found_less() {
        let slice = &[5];
        let target = 4;
        assert_eq!(binary_search(slice, &target), None);
    }

    #[test]
    fn single_element_not_found_greater() {
        let slice = &[5];
        let target = 6;
        assert_eq!(binary_search(slice, &target), None);
    }

    #[test]
    fn multiple_elements_found_middle() {
        let slice = &[1, 3, 5, 7, 9];
        let target = 5;
        assert_eq!(binary_search(slice, &target), Some(2));
    }

    #[test]
    fn multiple_elements_found_beginning() {
        let slice = &[1, 3, 5, 7, 9];
        let target = 1;
        assert_eq!(binary_search(slice, &target), Some(0));
    }

    #[test]
    fn multiple_elements_found_end() {
        let slice = &[1, 3, 5, 7, 9];
        let target = 9;
        assert_eq!(binary_search(slice, &target), Some(4));
    }

    #[test]
    fn repeated_elements_first_occurrence() {
        let slice = &[1, 1, 3, 5, 5];
        let target = 5;
        assert_eq!(binary_search(slice, &target), Some(3));
    }

    #[test]
    fn custom_comparison_reverse_order() {
        let slice = &[1, 3, 5];
        let target = 5;
        let comparison = |a: &i32| a.cmp(&target);
        assert_eq!(binary_search_with(slice, comparison), Some(2));
    }

    #[test]
    fn custom_comparison_ignore_field() {
        let slice = &[(5, "a"), (3, "b"), (1, "c")];
        let target = 5;
        let comparison = |(a, _): &(i32, &str)| target.cmp(&a);
        assert_eq!(binary_search_with(slice, comparison), Some(0));
    }
}
