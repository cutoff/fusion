pub mod urn;
pub mod collections;
pub mod io;

#[cfg(feature = "tracing-subscriber")]
pub mod logging;

use std::thread;
use std::thread::JoinHandle;

pub trait IntoOk
where
    Self: Sized
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