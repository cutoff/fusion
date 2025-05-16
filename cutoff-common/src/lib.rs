pub mod urn;
pub mod collections;
pub mod io;

#[cfg(feature = "tracing-subscriber")]
pub mod logging;

use std::thread;
use std::thread::JoinHandle;

pub trait IntoOk
where
    Self: Sized,
{
    fn into_ok<E>(self) -> Result<Self, E>;
}

pub trait MaybeFrom<T> {
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

/// Shortcut function for `thread::Builder::new().name(name.into()).spawn(f).unwrap()`.
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
