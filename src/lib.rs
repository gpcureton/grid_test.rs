// use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log::{LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Handle
};
use std::path::PathBuf;
use chrono::prelude::*;
// use chrono::offset::LocalResult;


pub fn config_logger(verbosity: &u8, log_file: Option<&PathBuf>) -> Result<Handle, SetLoggerError> {

    log::info!("The verbosity is {verbosity}");

    let levels = [
        LevelFilter::Error,
        LevelFilter::Warn,
        LevelFilter::Info,
        LevelFilter::Debug,
        LevelFilter::Trace,
    ];
    // let my_index: usize = verbosity.clone() as usize + 2;
    let my_index: usize = *verbosity as usize + 2;
    let my_index: usize = match my_index {
        // v if v < 2 => 2 as usize,
        v if v > 4 => 4_usize,
        _ => my_index,
        // _ => verbosity.clone() as usize
    };
    log::info!("my_index is {my_index}");
    let level = levels[my_index];
    log::info!("The level is {level}");

    // Figure out what to do with the log file...
    // TODO: Decide whether to have the default be a log file, or no log file.

    let dt: DateTime<Utc> = Utc::now();
    let dt_str: String = dt.format("%Y%m%dT%H%M%SZ").to_string();
    let logname = format!("data/grid_test.{dt_str}.log");
    log::info!("Creating log file = {:?}", logname);

    let binding = &PathBuf::from(logname);
    let file_path: &PathBuf = match log_file {
        Some(f) => f,
        None => binding,
    };
    log::info!("The log filename is {file_path:?}");

    // Set the log output pattern
    // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
    let log_output_pattern = match level {
        LevelFilter::Trace => "{d(%Y-%m-%d %H:%M:%S)}: {l:5} : {M}::{f}:{L:4} : {m}\n",
        _ => "{l:5} : {m}\n",
    };

    // Build a stdout logger.
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            log_output_pattern,
        )))
        .target(Target::Stdout).build();

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            log_output_pattern,
        )))
        .target(Target::Stderr).build();

    // Build a file logger.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new(
            log_output_pattern,
        )))
        .build(file_path)
        .unwrap();

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stdout", Box::new(stdout)),
        )
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stdout")
                // .appender("stderr")
                .build(level),
        )
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let handle = log4rs::init_config(config)?;
    // handle.set_config(config);

    log::error!("args() Goes to stderr and file");
    log::warn!("args() Goes to stderr and file");
    log::info!("args() Goes to stderr and file");
    log::debug!("args() Goes to file only");
    log::trace!("args() Goes to file only");

    // SetLoggerError
    Ok(handle)
}
