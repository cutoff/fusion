pub mod buffer_util;
pub mod lang_util;
pub mod io_util;

/// Just a common way of initializing the logging infrastructure
pub fn init_logging(max_level: tracing::Level) {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(max_level)
        // .without_time()
        // disable printing the name of the module in every log line.
        .with_target(false)
        .with_thread_names(true)
        .with_thread_ids(false)
        // disabling coloring, which won't work well in non-display logs
        // .with_ansi(false)
        .init();
}
