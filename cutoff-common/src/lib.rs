pub mod urn;
pub mod collections;
pub mod io;
pub mod logging;

use std::thread;
use std::thread::JoinHandle;

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