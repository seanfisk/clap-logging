use clap::arg_enum;

mod app;
mod color;
mod config;
mod env;
mod error;
mod log;

pub type Config = config::Config;
pub type Error = error::Error;
pub type Result<T> = std::result::Result<T, Error>;
pub type ColorMode = color::Mode;

const LOG_LEVEL_ARG_NAME: &str = "log-level";

// Can't do trait aliases so this needs to be in lib.rs
pub trait AppExt<'a, 'b> {
    fn log_level_arg(self) -> clap::App<'a, 'b>;
}

// Can't use pub(crate) with this due to a bug with the macro, so we need to put it in lib.rs
arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum LogLevel {
        trace,
        debug,
        info,
        warn,
        error,
    }
}
