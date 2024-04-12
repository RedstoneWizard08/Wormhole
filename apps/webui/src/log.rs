use anyhow::Result;
use std::{io, path::PathBuf};
use tracing_appender::{non_blocking, rolling};

use tracing_subscriber::{
    filter::LevelFilter,
    fmt::{self, writer::MakeWriterExt},
    layer::SubscriberExt,
    registry,
    util::SubscriberInitExt,
    EnvFilter,
};

pub fn from_log_level(level: log::LevelFilter) -> LevelFilter {
    match level {
        log::LevelFilter::Debug => LevelFilter::DEBUG,
        log::LevelFilter::Error => LevelFilter::ERROR,
        log::LevelFilter::Info => LevelFilter::INFO,
        log::LevelFilter::Off => LevelFilter::OFF,
        log::LevelFilter::Trace => LevelFilter::TRACE,
        log::LevelFilter::Warn => LevelFilter::WARN,
    }
}

pub fn init_file_logger(file: impl AsRef<str>, verbosity: LevelFilter) -> Result<()> {
    let file = PathBuf::from(file.as_ref());
    let file = rolling::hourly(file.parent().unwrap(), file.file_name().unwrap());
    let (appender, _guard) = non_blocking(file);
    let mut filter = EnvFilter::from_default_env().add_directive(verbosity.into());

    if verbosity == LevelFilter::DEBUG {
        filter = filter.add_directive("tokio_postgres::connection=warn".parse().unwrap());
        filter = filter.add_directive("tokio_postgres::query=warn".parse().unwrap());
        filter = filter.add_directive("diesel_async_migrations=warn".parse().unwrap());
        filter = filter.add_directive("tokio_util::codec::framed_impl=warn".parse().unwrap());
    }

    let layer = fmt::layer()
        .pretty()
        .compact()
        .with_writer(appender.and(io::stdout))
        .with_ansi(true)
        .with_level(true)
        .with_target(true)
        .with_file(false)
        .without_time();

    registry().with(filter).with(layer).init();

    Ok(())
}
