use std::collections::HashSet;
use std::hash::Hash;

pub enum DiffItem<T> {
    Same(T),
    Added(T),
    Removed(T),
}

pub trait MoreHashSet<T>
where
    T: Eq + Hash + Clone
{
    fn diff(&self, y: &HashSet<T>) -> Vec<DiffItem<T>>;
}

impl<T> MoreHashSet<T> for HashSet<T>
where
    T: Eq + Hash + Clone
{
    fn diff(&self, other: &HashSet<T>) -> Vec<DiffItem<T>> {
        self.intersection(other).cloned()
            .map(|item| DiffItem::Same(item))
            .chain(self.difference(other).cloned()
                .map(|item| DiffItem::Removed(item))
            ).chain(other.difference(self).cloned()
            .map(|item| DiffItem::Added(item)))
            .collect()
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
}