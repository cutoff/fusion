//! Extensions for the standard library's `RangeInclusive` type.
//!
//! This module provides additional functionality for `RangeInclusive` through the
//! `MoreRangeInclusive` trait, including methods for finding the intersection of ranges.

use std::ops::RangeInclusive;

/// Extension trait for `RangeInclusive` providing additional functionality.
///
/// This trait extends the standard library's `RangeInclusive` with methods for
/// finding the intersection of ranges and potentially other range operations.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the range. Must implement `Copy` and `PartialOrd`.
///
/// # Examples
///
/// ```
/// use cutoff_common::collections::more_range::MoreRangeInclusive;
/// use std::ops::RangeInclusive;
///
/// let range1 = 1..=5;
/// let range2 = 3..=7;
///
/// // Find the intersection of the two ranges
/// let intersection = range1.intersection(&range2);
/// assert_eq!(intersection, Some(3..=5));
/// ```
pub trait MoreRangeInclusive<T>
where
    T: Copy + PartialOrd,
{
    /// Finds the intersection of two ranges.
    ///
    /// The intersection is the range that contains all values that are in both
    /// ranges. If the ranges do not overlap, `None` is returned.
    ///
    /// # Parameters
    ///
    /// * `other` - The range to find the intersection with.
    ///
    /// # Returns
    ///
    /// * `Some(RangeInclusive<T>)` - The intersection of the two ranges.
    /// * `None` - If the ranges do not overlap.
    ///
    /// # Examples
    ///
    /// ```
    /// use cutoff_common::collections::more_range::MoreRangeInclusive;
    /// use std::ops::RangeInclusive;
    ///
    /// // Overlapping ranges
    /// let range1 = 1..=5;
    /// let range2 = 3..=7;
    /// assert_eq!(range1.intersection(&range2), Some(3..=5));
    ///
    /// // Non-overlapping ranges
    /// let range1 = 1..=3;
    /// let range2 = 4..=6;
    /// assert_eq!(range1.intersection(&range2), None);
    ///
    /// // Touching ranges
    /// let range1 = 1..=3;
    /// let range2 = 3..=5;
    /// assert_eq!(range1.intersection(&range2), Some(3..=3));
    ///
    /// // One range inside another
    /// let range1 = 1..=10;
    /// let range2 = 3..=7;
    /// assert_eq!(range1.intersection(&range2), Some(3..=7));
    /// ```
    fn intersection(&self, other: &RangeInclusive<T>) -> Option<RangeInclusive<T>>;
}

impl<T> MoreRangeInclusive<T> for RangeInclusive<T>
where
    T: Copy + PartialOrd,
{
    fn intersection(&self, other: &RangeInclusive<T>) -> Option<RangeInclusive<T>> {
        // Find the maximum of the start values
        let start = if self.start() > other.start() { *self.start() } else { *other.start() };
        // Find the minimum of the end values
        let end = if self.end() < other.end() { *self.end() } else { *other.end() };

        // If start <= end, the ranges overlap
        if start <= end {
            Some(start..=end)
        } else {
            // Otherwise, there is no intersection
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
