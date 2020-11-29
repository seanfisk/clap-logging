#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could decode value of environment variable with name {name:?} as Unicode")]
    EnvVarDecoding { name: String },
    #[error("Invalid minimum log level {:?}; valid values are {0:?}", &crate::LogLevel::variants())]
    InvalidLogLevel {
        level: String,
        source: log::ParseLevelError,
    },
    #[error("Could not set logger for log framework")]
    SetLogger { source: log::SetLoggerError },
}
