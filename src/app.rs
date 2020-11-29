impl<'a, 'b> crate::AppExt<'a, 'b> for clap::App<'a, 'b> {
    fn log_level_arg(self) -> clap::App<'a, 'b> {
        self.arg(
            clap::Arg::with_name(crate::LOG_LEVEL_ARG_NAME)
                .short("l")
                .possible_values(&crate::LogLevel::variants())
                .help("Set the minimum log level")
                .long("log-level")
                .takes_value(true)
                .value_name("LEVEL"),
        )
    }
}
