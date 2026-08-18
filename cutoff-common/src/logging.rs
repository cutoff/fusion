//! Logging utilities for error handling and initialization.
//!
//! This module provides utilities for working with the `tracing` crate,
//! including error handling with logging and a standardized way to initialize
//! the logging infrastructure.
//!
//! The module is only available when the `tracing-subscriber` feature is enabled.
//!
//! # Examples
//!
//! ```
//! # #[cfg(feature = "tracing-subscriber")]
//! # {
//! use cutoff_common::logging::{OkOrLog, init_logging};
//! use tracing::Level;
//!
//! // Initialize logging with INFO level
//! init_logging(Level::INFO);
//!
//! // Use OkOrLog to handle a Result and log any errors
//! let result: Result<i32, &str> = Ok(42);
//! let value = result.ok_or_log(Level::WARN);
//! assert_eq!(value, Some(42));
//!
//! let result: Result<i32, &str> = Err("An error occurred");
//! let value = result.ok_or_log(Level::WARN);
//! assert_eq!(value, None); // The error is logged at WARN level
//! # }
//! ```

use std::fmt::Display;
use tracing::{event, Level};

/// A trait for converting a `Result` into an `Option` while logging any errors.
///
/// This trait provides a convenient way to handle errors by logging them and
/// converting the `Result` into an `Option`. This is useful in situations where
/// you want to log errors but don't want to propagate them up the call stack.
///
/// # Type Parameters
///
/// * `T` - The type of the value in the `Ok` variant of the `Result`.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "tracing-subscriber")]
/// # {
/// use cutoff_common::logging::OkOrLog;
/// use tracing::Level;
///
/// // With an Ok result
/// let result: Result<i32, &str> = Ok(42);
/// let value = result.ok_or_log(Level::WARN);
/// assert_eq!(value, Some(42));
///
/// // With an Err result
/// let result: Result<i32, &str> = Err("An error occurred");
/// let value = result.ok_or_log(Level::WARN);
/// assert_eq!(value, None); // The error is logged at WARN level
/// # }
/// ```
pub trait OkOrLog<T> {
    /// Converts `self` into an `Option<T>`, consuming `self`,
    /// and logs the error, if any, at the specified log level.
    ///
    /// # Parameters
    ///
    /// * `level` - The log level at which to log any error.
    ///
    /// # Returns
    ///
    /// * `Some(T)` - If `self` is `Ok(T)`.
    /// * `None` - If `self` is `Err(E)`. The error is logged at the specified level.
    fn ok_or_log(self, level: Level) -> Option<T>;
}

impl<T, E> OkOrLog<T> for Result<T, E>
where
    E: Display,
{
    fn ok_or_log(self, level: Level) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(err) => {
                // Log the error message at the specified level
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

/// Initializes the logging infrastructure with a standardized configuration.
///
/// This function sets up the `tracing_subscriber` with a compact format and
/// the specified maximum log level. It configures the subscriber to include
/// thread names but not thread IDs, and to include the target (module path).
///
/// # Parameters
///
/// * `max_level` - The maximum log level to display. Messages with a level
///   higher than this will be filtered out.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "tracing-subscriber")]
/// # {
/// use cutoff_common::logging::init_logging;
/// use tracing::Level;
///
/// // Initialize logging with INFO level
/// init_logging(Level::INFO);
///
/// // Now you can use tracing macros to log messages
/// tracing::info!("This is an info message");
/// tracing::debug!("This debug message won't be displayed");
/// # }
/// ```
///
/// # Note
///
/// This function should be called early in your application's startup process,
/// typically in the `main` function or during application initialization.
/// Calling it multiple times may have unintended consequences, as it will
/// attempt to set the global default subscriber each time.
pub fn init_logging(max_level: Level) {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(max_level)
        // .without_time()
        // Enable printing the name of the module in every log line
        .with_target(true)
        // Include thread names for better debugging of multi-threaded code
        .with_thread_names(true)
        // Exclude thread IDs to keep the output cleaner
        .with_thread_ids(false)
        // ANSI colors are enabled by default for better readability in terminals
        // .with_ansi(false) // Uncomment to disable colors
        .init();
}
