use std::ops::RangeInclusive;

pub trait MoreRangeInclusive<T>
where
    T: Copy + PartialOrd
{
    fn intersection(&self, range2: &RangeInclusive<T>) -> Option<RangeInclusive<T>>;    
}

impl<T> MoreRangeInclusive<T> for RangeInclusive<T>
where
    T: Copy + PartialOrd
{
    fn intersection(&self, other: &RangeInclusive<T>) -> Option<RangeInclusive<T>> {
        let start = if self.start() > other.start() { *self.start() } else { *other.start() };
        let end = if self.end() < other.end() { *self.end() } else { *other.end() };

        if start <= end {
            Some(start..=end)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_i32() {
        let range1 = 1..=5;
        let range2 = 3..=7;
        assert_eq!(range1.intersection(&range2), Some(3..=5));
    }

    #[test]
    fn test_no_intersection_i32() {
        let range1 = 1..=3;
        let range2 = 4..=6;
        assert_eq!(range1.intersection(&range2), None);
    }

    #[test]
    fn test_touching_ranges_i32() {
        let range1 = 1..=3;
        let range2 = 3..=5;
        assert_eq!(range1.intersection(&range2), Some(3..=3));
    }

    #[test]
    fn test_one_range_inside_another_i32() {
        let range1 = 1..=10;
        let range2 = 3..=7;
        assert_eq!(range1.intersection(&range2), Some(3..=7));
    }

    #[test]
    fn test_identical_ranges_i32() {
        let range1 = 1..=5;
        let range2 = 1..=5;
        assert_eq!(range1.intersection(&range2), Some(1..=5));
    }

    #[test]
    fn test_single_element_range_i32() {
        let range1 = 3..=3;
        let range2 = 1..=5;
        assert_eq!(range1.intersection(&range2), Some(3..=3));
    }

    #[test]
    fn test_intersection_f64() {
        let range1 = 1.0..=5.0;
        let range2 = 3.0..=7.0;
        assert_eq!(range1.intersection(&range2), Some(3.0..=5.0));
    }

    #[test]
    fn test_no_intersection_f64() {
        let range1 = 1.0..=3.0;
        let range2 = 3.1..=6.0;
        assert_eq!(range1.intersection(&range2), None);
    }

    #[test]
    fn test_intersection_char() {
        let range1 = 'a'..='e';
        let range2 = 'c'..='g';
        assert_eq!(range1.intersection(&range2), Some('c'..='e'));
    }

    #[test]
    fn test_no_intersection_char() {
        let range1 = 'a'..='c';
        let range2 = 'd'..='f';
        assert_eq!(range1.intersection(&range2), None);
    }
}