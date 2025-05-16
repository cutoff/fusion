//! Extensions for the standard library's `HashSet` type.
//!
//! This module provides additional functionality for `HashSet` through the
//! `MoreHashSet` trait, including methods for comparing sets and filtering elements.

use std::collections::HashSet;
use std::hash::Hash;
use std::mem;

/// Represents an item in the difference between two sets.
///
/// When comparing two sets, each element can be in one of three states:
/// - Present in both sets (`Same`)
/// - Present only in the second set (`Added`)
/// - Present only in the first set (`Removed`)
///
/// This enum is used by the `diff` method to represent these states.
///
/// # Examples
///
/// ```
/// use cutoff_common::collections::more_hashset::{MoreHashSet, DiffItem};
/// use std::collections::HashSet;
///
/// let mut set1 = HashSet::new();
/// set1.insert(1);
/// set1.insert(2);
///
/// let mut set2 = HashSet::new();
/// set2.insert(2);
/// set2.insert(3);
///
/// let diff = set1.diff(&set2);
/// // diff contains: DiffItem::Same(2), DiffItem::Removed(1), DiffItem::Added(3)
/// ```
#[derive(Debug, PartialEq, Eq)]
pub enum DiffItem<T> {
    /// The item is present in both sets.
    Same(T),
    /// The item is present only in the second set.
    Added(T),
    /// The item is present only in the first set.
    Removed(T),
}

/// Extension trait for `HashSet` providing additional functionality.
///
/// This trait extends the standard library's `HashSet` with methods for
/// comparing sets and filtering elements.
///
/// # Type Parameters
///
/// * `T` - The type of elements in the set. Must implement `Eq`, `Hash`, and `Clone`.
///
/// # Examples
///
/// ```
/// use cutoff_common::collections::more_hashset::MoreHashSet;
/// use std::collections::HashSet;
///
/// let mut set = HashSet::new();
/// set.insert(1);
/// set.insert(2);
/// set.insert(3);
///
/// // Use drain_filter to remove even numbers
/// let removed = set.drain_filter(|&x| x % 2 == 0);
///
/// assert_eq!(set.len(), 2);
/// assert!(set.contains(&1));
/// assert!(set.contains(&3));
///
/// assert_eq!(removed.len(), 1);
/// assert!(removed.contains(&2));
/// ```
pub trait MoreHashSet<T>
where
    T: Eq + Hash + Clone,
{
    /// Compares two sets and returns a vector of `DiffItem`s representing the differences.
    ///
    /// This method identifies elements that are:
    /// - Present in both sets (`Same`)
    /// - Present only in the other set (`Added`)
    /// - Present only in this set (`Removed`)
    ///
    /// # Parameters
    ///
    /// * `other` - The set to compare with.
    ///
    /// # Returns
    ///
    /// A vector of `DiffItem`s representing the differences between the sets.
    ///
    /// # Examples
    ///
    /// ```
    /// use cutoff_common::collections::more_hashset::{MoreHashSet, DiffItem};
    /// use std::collections::HashSet;
    ///
    /// let mut set1 = HashSet::new();
    /// set1.insert(1);
    /// set1.insert(2);
    ///
    /// let mut set2 = HashSet::new();
    /// set2.insert(2);
    /// set2.insert(3);
    ///
    /// let diff = set1.diff(&set2);
    ///
    /// // Count items by type
    /// let same_count = diff.iter().filter(|item| matches!(item, DiffItem::Same(_))).count();
    /// let added_count = diff.iter().filter(|item| matches!(item, DiffItem::Added(_))).count();
    /// let removed_count = diff.iter().filter(|item| matches!(item, DiffItem::Removed(_))).count();
    ///
    /// assert_eq!(same_count, 1);    // 2 is in both sets
    /// assert_eq!(added_count, 1);   // 3 is only in set2
    /// assert_eq!(removed_count, 1); // 1 is only in set1
    /// ```
    fn diff(&self, other: &HashSet<T>) -> Vec<DiffItem<T>>;

    /// Removes elements from the set that match a predicate and returns them as a new set.
    ///
    /// This method is similar to the standard library's `retain` method, but it returns
    /// the removed elements as a new set.
    ///
    /// # Parameters
    ///
    /// * `predicate` - A function that returns `true` for elements that should be removed.
    ///
    /// # Returns
    ///
    /// A new `HashSet` containing all elements that were removed.
    ///
    /// # Examples
    ///
    /// ```
    /// use cutoff_common::collections::more_hashset::MoreHashSet;
    /// use std::collections::HashSet;
    ///
    /// let mut set = HashSet::new();
    /// set.insert(1);
    /// set.insert(2);
    /// set.insert(3);
    /// set.insert(4);
    ///
    /// // Remove all even numbers
    /// let removed = set.drain_filter(|&x| x % 2 == 0);
    ///
    /// // Check the original set
    /// assert_eq!(set.len(), 2);
    /// assert!(set.contains(&1));
    /// assert!(set.contains(&3));
    ///
    /// // Check the removed elements
    /// assert_eq!(removed.len(), 2);
    /// assert!(removed.contains(&2));
    /// assert!(removed.contains(&4));
    /// ```
    fn drain_filter<F>(&mut self, predicate: F) -> HashSet<T>
    where
        F: FnMut(&T) -> bool;
}

impl<T> MoreHashSet<T> for HashSet<T>
where
    T: Eq + Hash + Clone,
{
    fn diff(&self, other: &HashSet<T>) -> Vec<DiffItem<T>> {
        // Find elements that are in both sets
        self.intersection(other).cloned()
            .map(|item| DiffItem::Same(item))
            // Find elements that are only in self
            .chain(self.difference(other).cloned()
                .map(|item| DiffItem::Removed(item))
            )
            // Find elements that are only in other
            .chain(other.difference(self).cloned()
                .map(|item| DiffItem::Added(item))
            )
            .collect()
    }

    fn drain_filter<F>(&mut self, mut predicate: F) -> HashSet<T>
    where
        F: FnMut(&T) -> bool,
    {
        // Use mem::take to swap out the original set with an empty one
        let original = mem::take(self);
        let mut removed = HashSet::new();

        // Move matching elements to removed, non-matching back to self
        for item in original {
            if predicate(&item) {
                removed.insert(item);
            } else {
                self.insert(item);
            }
        }

        removed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn set_from_slice<T: Eq + Hash + Clone>(slice: &[T]) -> HashSet<T> {
        slice.iter().cloned().collect()
    }

    #[test]
    fn test_identical_sets() {
        let set1 = set_from_slice(&[1, 2, 3]);
        let set2 = set_from_slice(&[1, 2, 3]);
        let diff = set1.diff(&set2);
        assert_eq!(diff.len(), 3);
        assert!(diff.iter().all(|item| matches!(item, DiffItem::Same(_))));
    }

    #[test]
    fn test_disjoint_sets() {
        let set1 = set_from_slice(&[1, 2, 3]);
        let set2 = set_from_slice(&[4, 5, 6]);
        let diff = set1.diff(&set2);
        assert_eq!(diff.len(), 6);
        assert_eq!(diff.iter().filter(|item| matches!(item, DiffItem::Removed(_))).count(), 3);
        assert_eq!(diff.iter().filter(|item| matches!(item, DiffItem::Added(_))).count(), 3);
    }

    #[test]
    fn test_partially_overlapping_sets() {
        let set1 = set_from_slice(&[1, 2, 3, 4]);
        let set2 = set_from_slice(&[3, 4, 5, 6]);
        let diff = set1.diff(&set2);
        assert_eq!(diff.len(), 6);
        assert_eq!(diff.iter().filter(|item| matches!(item, DiffItem::Same(_))).count(), 2);
        assert_eq!(diff.iter().filter(|item| matches!(item, DiffItem::Removed(_))).count(), 2);
        assert_eq!(diff.iter().filter(|item| matches!(item, DiffItem::Added(_))).count(), 2);
    }

    #[test]
    fn test_subset() {
        let set1 = set_from_slice(&[1, 2, 3, 4, 5]);
        let set2 = set_from_slice(&[2, 3, 4]);
        let diff = set1.diff(&set2);
        assert_eq!(diff.len(), 5);
        assert_eq!(diff.iter().filter(|item| matches!(item, DiffItem::Same(_))).count(), 3);
        assert_eq!(diff.iter().filter(|item| matches!(item, DiffItem::Removed(_))).count(), 2);
        assert_eq!(diff.iter().filter(|item| matches!(item, DiffItem::Added(_))).count(), 0);
    }

    #[test]
    fn test_empty_set() {
        let set1 = set_from_slice(&[1, 2, 3]);
        let set2 = HashSet::new();
        let diff = set1.diff(&set2);
        assert_eq!(diff.len(), 3);
        assert!(diff.iter().all(|item| matches!(item, DiffItem::Removed(_))));

        let diff = set2.diff(&set1);
        assert_eq!(diff.len(), 3);
        assert!(diff.iter().all(|item| matches!(item, DiffItem::Added(_))));
    }

    #[test]
    fn test_with_strings() {
        let set1 = set_from_slice(&["apple", "banana", "cherry"]);
        let set2 = set_from_slice(&["banana", "cherry", "date"]);
        let diff = set1.diff(&set2);
        assert_eq!(diff.len(), 4);
        assert_eq!(diff.iter().filter(|item| matches!(item, DiffItem::Same(_))).count(), 2);
        assert_eq!(diff.iter().filter(|item| matches!(item, DiffItem::Removed(_))).count(), 1);
        assert_eq!(diff.iter().filter(|item| matches!(item, DiffItem::Added(_))).count(), 1);
    }

    #[test]
    fn test_order_independence() {
        let set1 = set_from_slice(&[3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]);
        let set2 = set_from_slice(&[2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4]);
        let diff1 = set1.diff(&set2);
        let diff2 = set2.diff(&set1);

        assert_eq!(diff1.len(), diff2.len());

        let count_same1 = diff1.iter().filter(|item| matches!(item, DiffItem::Same(_))).count();
        let count_same2 = diff2.iter().filter(|item| matches!(item, DiffItem::Same(_))).count();
        assert_eq!(count_same1, count_same2);

        let count_removed1 = diff1.iter().filter(|item| matches!(item, DiffItem::Removed(_))).count();
        let count_added2 = diff2.iter().filter(|item| matches!(item, DiffItem::Added(_))).count();
        assert_eq!(count_removed1, count_added2);

        let count_added1 = diff1.iter().filter(|item| matches!(item, DiffItem::Added(_))).count();
        let count_removed2 = diff2.iter().filter(|item| matches!(item, DiffItem::Removed(_))).count();
        assert_eq!(count_added1, count_removed2);
    }

    #[test]
    fn test_drain_filter_all() {
        let mut set = set_from_slice(&[1, 2, 3, 4, 5]);
        let removed = set.drain_filter(|_| true);

        // All elements should be removed
        assert!(set.is_empty());
        assert_eq!(removed.len(), 5);
        assert!(removed.contains(&1));
        assert!(removed.contains(&2));
        assert!(removed.contains(&3));
        assert!(removed.contains(&4));
        assert!(removed.contains(&5));
    }

    #[test]
    fn test_drain_filter_none() {
        let mut set = set_from_slice(&[1, 2, 3, 4, 5]);
        let original_set = set.clone();
        let removed = set.drain_filter(|_| false);

        // No elements should be removed
        assert_eq!(set, original_set);
        assert!(removed.is_empty());
    }

    #[test]
    fn test_drain_filter_predicate() {
        let mut set = set_from_slice(&[1, 2, 3, 4, 5]);
        let removed = set.drain_filter(|&x| x % 2 == 0);

        // Only even numbers should be removed
        assert_eq!(set.len(), 3);
        assert!(set.contains(&1));
        assert!(set.contains(&3));
        assert!(set.contains(&5));

        assert_eq!(removed.len(), 2);
        assert!(removed.contains(&2));
        assert!(removed.contains(&4));
    }

    #[test]
    fn test_drain_filter_empty_set() {
        let mut set: HashSet<i32> = HashSet::new();
        let removed = set.drain_filter(|_| true);

        // Nothing should happen with an empty set
        assert!(set.is_empty());
        assert!(removed.is_empty());
    }

    #[test]
    fn test_drain_filter_with_strings() {
        let mut set = set_from_slice(&["apple", "banana", "cherry", "date", "elderberry"]);
        let removed = set.drain_filter(|s| s.len() > 5);

        // Only strings with length > 5 should be removed
        assert_eq!(set.len(), 2);
        assert!(set.contains("apple"));
        assert!(set.contains("date"));

        assert_eq!(removed.len(), 3);
        assert!(removed.contains("banana"));
        assert!(removed.contains("cherry"));
        assert!(removed.contains("elderberry"));
    }
}
