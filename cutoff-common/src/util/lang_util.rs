//! Standard features extensions

use std::collections::HashSet;
use std::hash::Hash;

pub mod vecmap;

pub enum DiffItem<T> {
    Same(T),
    Added(T),
    Removed(T),
}

pub fn diff<T: Eq + Hash + Clone>(x: &HashSet<T>, y: &HashSet<T>) -> Vec<DiffItem<T>> {
    x.intersection(y).cloned()
        .map(|item| DiffItem::Same(item))
        .chain(x.difference(y).cloned()
            .map(|item| DiffItem::Removed(item))
        ).chain(y.difference(x).cloned()
        .map(|item| DiffItem::Added(item)))
        .collect()
}

/*#[cfg(test)]
mod tests {
    use common_macros::hash_set;
    use crate::util::lang_util::diff;

    #[test]
    fn test_diff() {
        let x = hash_set! { 1, 2, 3, 4 };
        let y = hash_set! { 3, 4, 5, 6 };
        let diff = diff(&x, &y);

        assert_eq!(
            hash_set! { &3, &4 },
            diff.same,
        );
        assert_eq!(
            hash_set! { &5, &6 },
            diff.added,
        );
        assert_eq!(
            hash_set! { &1, &2 },
            diff.removed,
        );
    }
}*/