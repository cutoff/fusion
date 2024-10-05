//! Standard features extensions

use std::fmt::Display;
use std::thread;
use std::thread::JoinHandle;
use tracing::{event, Level};

pub mod vecmap;
pub mod more_hashset;
pub mod more_range;

pub trait OkOrLog<T> {
    fn ok_or_log(self, level: tracing::Level) -> Option<T>;
}

impl<T, E> OkOrLog<T> for Result<T, E>
where
    E: Display,
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
    fn ok_or_log(self, level: Level) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                match level {
                    Level::TRACE => event!(Level::TRACE, "{}", err),
                    Level::DEBUG => event!(Level::DEBUG, "{}", err),
                    Level::INFO => event!(Level::INFO, "{}", err),
                    Level::WARN => event!(Level::WARN, "{}", err),
                    Level::ERROR => event!(Level::ERROR, "{}", err),
                }
                None
            }
        }
    }
}

pub trait MaybeFrom<T> {
    fn maybe_from(value: T) -> Option<Self>
    where
        Self: Sized;
}

/// Shortcut function for `thread::Builder::new().name(name.into()).spawn(f).unwrap()`.
pub fn thread_spawn<F, T>(name: &str, f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    thread::Builder::new().name(name.into()).spawn(f).unwrap()
}