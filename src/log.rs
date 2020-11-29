use ansi_term::Color::*;
use chrono::Local;
use log::{Level, Log, Metadata, Record};
use std::fmt::Display;
use std::io::{stderr, Write};
use std::thread;

pub(crate) struct Logger {
    pub(crate) colorize: bool,
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            log_message(self.colorize, record.level(), record.args());
        }
    }

    fn flush(&self) {
        stderr().flush().expect("Failed to flush stderr!");
    }
}

fn log_message<T: Display>(colorize: bool, level: Level, message: T) {
    let mut time_string = Local::now().format("%Y-%m-%dT%H:%M:%S%Z ").to_string();
    if colorize {
        time_string = Black.bold().paint(time_string).to_string();
    }

    let mut level_string = level.to_string();
    if colorize {
        level_string = match level {
            Level::Error => Red.paint(level_string),
            Level::Warn => Yellow.paint(level_string),
            Level::Info => Cyan.paint(level_string),
            Level::Debug => Blue.paint(level_string),
            Level::Trace => Purple.paint(level_string),
        }
        .to_string();
    }

    let current_thread = thread::current();
    let thread_name = current_thread.name().map_or_else(
        || format!("{:?}", current_thread.id()),
        |name| name.to_string(),
    );

    eprintln!(
        "{}[{}] {}: {}",
        time_string, thread_name, level_string, message
    );
}
