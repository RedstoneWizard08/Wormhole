// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

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

    filter = filter.add_directive("tokio_postgres::connection=warn".parse().unwrap());
    filter = filter.add_directive("tokio_postgres::query=warn".parse().unwrap());
    filter = filter.add_directive("diesel_async_migrations=warn".parse().unwrap());
    filter = filter.add_directive("tokio_util::codec::framed_impl=warn".parse().unwrap());
    filter = filter.add_directive("tokio_tungstenite=warn".parse().unwrap());
    filter = filter.add_directive("want=warn".parse().unwrap());
    filter = filter.add_directive("tungstenite=warn".parse().unwrap());
    filter = filter.add_directive("arboard=warn".parse().unwrap());

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
