use chrono::Local;
use core::fmt as core_fmt;
use core::result as core_result;
use std::fs as std_fs;
use std::io as std_io;
use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling;
use tracing_subscriber::fmt as tracing_fmt;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry::Registry;
use tracing_subscriber::reload::Error;
use tracing_subscriber::reload::Handle;
use tracing_subscriber::util::SubscriberInitExt;

struct LogTimer;

impl FormatTime for LogTimer {
    fn format_time(&self, writer: &mut Writer<'_>) -> core_fmt::Result {
        write!(writer, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

pub struct LogManager {
    _file_writer_guard: WorkerGuard,
    level_filter: LevelFilter,
    level_filter_handle: Handle<LevelFilter, Registry>,
}

impl LogManager {
    pub fn is_debug_enabled(&self) -> bool {
        self.level_filter == LevelFilter::DEBUG
    }

    pub fn enable_debug(&mut self) -> core_result::Result<(), Error> {
        self.level_filter = LevelFilter::DEBUG;
        self.apply_level_filter()
    }

    pub fn disable_debug(&mut self) -> core_result::Result<(), Error> {
        self.level_filter = LevelFilter::INFO;
        self.apply_level_filter()
    }

    fn apply_level_filter(&self) -> core_result::Result<(), Error> {
        self.level_filter_handle
            .modify(|level_filter| *level_filter = self.level_filter)
    }
}

pub fn init_log_manager() -> LogManager {
    std_fs::create_dir_all("logs").expect("Failed to create logs directory at ./logs");
    let level_filter = LevelFilter::INFO;
    let (level_filter_layer, level_filter_handle) =
        tracing_subscriber::reload::Layer::new(level_filter);
    let file_writer = rolling::daily("logs", "repox.log");
    let (file_writer, file_writer_guard) = tracing_appender::non_blocking(file_writer);
    let stdout_layer = tracing_fmt::layer()
        .with_writer(std_io::stdout)
        .with_ansi(true)
        .with_target(false)
        .with_timer(LogTimer);
    let file_layer = tracing_fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false)
        .with_target(false)
        .with_timer(LogTimer);
    tracing_subscriber::registry()
        .with(level_filter_layer)
        .with(stdout_layer)
        .with(file_layer)
        .init();
    LogManager {
        _file_writer_guard: file_writer_guard,
        level_filter,
        level_filter_handle,
    }
}
