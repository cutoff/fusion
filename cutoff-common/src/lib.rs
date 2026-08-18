//! # Cutoff Common
//!
//! A collection of common utilities and helpers used across Cutoff projects.
//!
//! This crate provides various utility traits, functions, and modules that are
//! commonly used throughout the Cutoff ecosystem.
//!
//! ## Features
//!
//! - Common traits like `IntoOk` and `MaybeFrom`
//! - Thread utilities
//! - Collections utilities
//! - I/O utilities
//! - URN handling
//! - Optional logging utilities (with the `tracing-subscriber` feature)
//! - Optional serialization support (with the `serde` feature)

pub mod urn;
pub mod collections;
pub mod io;

#[cfg(feature = "tracing-subscriber")]
pub mod logging;

use std::thread;
use std::thread::JoinHandle;

/// A trait for converting a value into a `Result::Ok` variant.
///
/// This trait provides a convenient way to wrap any value in a `Result::Ok`,
/// which can be useful in contexts where a `Result` is expected but you have
/// a value that cannot fail.
///
/// # Examples
///
/// ```
/// use cutoff_common::IntoOk;
///
/// let value = 42;
/// let result: Result<i32, &str> = value.into_ok();
/// assert_eq!(result, Ok(42));
/// ```
pub trait IntoOk
where
    Self: Sized,
{
    /// Converts `self` into a `Result::Ok` variant.
    ///
    /// # Type Parameters
    ///
    /// * `E` - The error type for the resulting `Result`.
    ///
    /// # Returns
    ///
    /// A `Result` with `self` wrapped in the `Ok` variant.
    fn into_ok<E>(self) -> Result<Self, E>;
}

/// A trait for attempting to convert from one type to another.
///
/// Similar to the standard library's `From` trait, but returns an `Option`
/// to indicate whether the conversion was successful.
///
/// # Examples
///
/// ```
/// use cutoff_common::MaybeFrom;
///
/// // Define a wrapper type for the example
/// #[derive(Debug, PartialEq)]
/// struct MyWrapper(String);
///
/// // Implementing MaybeFrom for our wrapper type
/// impl MaybeFrom<i32> for MyWrapper {
///     fn maybe_from(value: i32) -> Option<Self> {
///         if value > 0 {
///             Some(MyWrapper(value.to_string()))
///         } else {
///             None
///         }
///     }
/// }
///
/// // Using the implementation
/// let result = MyWrapper::maybe_from(42);
/// assert_eq!(result.unwrap().0, "42");
///
/// let result = MyWrapper::maybe_from(0);
/// assert_eq!(result, None);
/// ```
pub trait MaybeFrom<T> {
    /// Attempts to convert from `value` to `Self`.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to convert from.
    ///
    /// # Returns
    ///
    /// `Some(Self)` if the conversion was successful, `None` otherwise.
    fn maybe_from(value: T) -> Option<Self>
    where
        Self: Sized;
}

impl<T> IntoOk for T
where
    Self: Sized,
{
    fn into_ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }
}

/// Creates a new thread with the specified name and executes the provided function.
///
/// This is a convenience wrapper around the standard library's thread creation
/// functionality. It creates a thread with the given name and executes the
/// provided function in that thread.
///
/// # Parameters
///
/// * `name` - The name to assign to the thread.
/// * `f` - The function to execute in the new thread.
///
/// # Returns
///
/// A `JoinHandle` that can be used to wait for the thread to complete and
/// retrieve its result.
///
/// # Panics
///
/// This function will panic if thread creation fails.
///
/// # Examples
///
/// ```
/// use cutoff_common::thread_spawn;
/// use std::sync::mpsc;
///
/// let (tx, rx) = mpsc::channel();
///
/// let handle = thread_spawn("example-thread", move || {
///     tx.send("Hello from thread").unwrap();
///     42 // Return value
/// });
///
/// assert_eq!(rx.recv().unwrap(), "Hello from thread");
/// assert_eq!(handle.join().unwrap(), 42);
/// ```
pub fn thread_spawn<F, T>(name: &str, f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
{
    thread::Builder::new().name(name.into()).spawn(f).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn test_into_ok() {
        let value = 42;
        let result: Result<i32, &str> = value.into_ok();
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_maybe_from() {
        // Test a simple implementation of MaybeFrom
        #[allow(non_local_definitions)]
        impl MaybeFrom<i32> for Option<String> {
            fn maybe_from(value: i32) -> Option<Self> {
                if value > 0 {
                    Some(Some(value.to_string()))
                } else {
                    None
                }
            }
        }

        // Test with a positive value
        let result = Option::<String>::maybe_from(42);
        assert_eq!(result, Some(Some("42".to_string())));

        // Test with a non-positive value
        let result = Option::<String>::maybe_from(0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_thread_spawn() {
        let (tx, rx) = mpsc::channel();

        // Spawn a thread that sends a value
        let handle = thread_spawn("test-thread", move || {
            tx.send(42).unwrap();
            "thread result"
        });

        // Verify the thread name
        assert_eq!(handle.thread().name(), Some("test-thread"));

        // Verify the thread executed the closure
        assert_eq!(rx.recv().unwrap(), 42);

        // Verify the thread returned the expected value
        assert_eq!(handle.join().unwrap(), "thread result");
    }
}
