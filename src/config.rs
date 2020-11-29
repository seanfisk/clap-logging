use crate::color;
use crate::Error;
use atty::Stream;
use clap::{AppSettings, ArgMatches};
use log::LevelFilter;
use std::str::FromStr;

pub struct Config {
    pub(crate) color_mode: color::Mode,
}

impl Config {
    pub fn new() -> crate::Result<Config> {
        color::read_mode_from_env().map(|color_mode| Config { color_mode })
    }

    pub fn clap_settings(&self) -> Vec<AppSettings> {
        vec![
            AppSettings::ColoredHelp,
            match self.color_mode {
                color::Mode::Never => AppSettings::ColorNever,
                color::Mode::Always => AppSettings::ColorAlways,
                color::Mode::Auto => AppSettings::ColorAuto,
            },
        ]
    }

    pub fn init_logger(
        &self,
        matches: &ArgMatches,
        env_var_name: &str,
        default_level: LevelFilter,
    ) -> crate::Result<()> {
        let log_level = matches.value_of(crate::LOG_LEVEL_ARG_NAME).map_or_else(
            || {
                crate::env::get(env_var_name)?
                    .map_or(Ok(default_level), |value| parse_log_level(&value))
            },
            parse_log_level,
        )?;

        let colorize = match self.color_mode {
            color::Mode::Never => false,
            color::Mode::Always => true,
            color::Mode::Auto => atty::is(Stream::Stderr),
        };

        // Set the environment variable for any potential child processes
        std::env::set_var(env_var_name, log_level.to_string().to_ascii_lowercase());

        log::set_boxed_logger(Box::new(crate::log::Logger { colorize }))
            .map(|()| log::set_max_level(log_level))
            .map_err(|source| Error::SetLogger { source })
    }
}

fn parse_log_level(value: &str) -> crate::Result<LevelFilter> {
    LevelFilter::from_str(&value).map_err(|source| Error::InvalidLogLevel {
        level: value.to_owned(),
        source,
    })
}
