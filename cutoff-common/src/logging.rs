use std::fmt::Display;
use tracing::{event, Level};

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

/// Just a common way of initializing the logging infrastructure
#[cfg(feature = "tracing-subscriber")]
pub fn init_logging(max_level: tracing::Level) {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(max_level)
        // .without_time()
        // disable printing the name of the module in every log line.
        .with_target(true)
        .with_thread_names(true)
        .with_thread_ids(false)
        // disabling coloring, which won't work well in non-display logs
        // .with_ansi(false)
        .init();
}
