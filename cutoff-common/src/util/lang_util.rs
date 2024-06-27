//! Standard features extensions

use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;

use log::log;

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

pub trait LogAndNone<T> {
    fn ok_or_log(self, level: log::Level) -> Option<T>;
}

impl<T, E> LogAndNone<T> for Result<T, E>
where
    E: Display
{
    /// Converts `self` into an [`Option<T>`], consuming `self`,
    /// and log the error, if any, in level `level`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use log::Level;
    /// let x: Result<u32, &str> = Ok(2);
    /// assert_eq!(x.ok_or_log(Level::Info), Some(2));
    ///
    /// let x: Result<u32, &str> = Err("Nothing here");
    /// assert_eq!(x.ok_or_log(Level::Info), None); // log "Nothing Here" in Info level 
    /// ```
    fn ok_or_log(self, level: log::Level) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                log!(level, "{}", err);
                None
            },
        }
    }
}