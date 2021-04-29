use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, App};
use clap_logging::AppExt;
use log::{debug, error, info, trace, warn, LevelFilter};

fn main() -> Result<()> {
    let config = clap_logging::Config::new()?;
    let app = App::new(crate_name!())
        .settings(&config.clap_settings())
        .about(crate_description!())
        .version(crate_version!())
        .log_level_arg()
        .author(crate_authors!());

    debug!("Parsing command-line arguments");

    let matches = app.get_matches();

    debug!("Parsing has completed");

    config.init_logger(&matches, "DEMO_LOG_LEVEL", LevelFilter::Trace)?;

    debug!("Logger was succesfully instantiated");

    trace!("Trace message");
    debug!("Debug message");
    info!("Info message");
    warn!("Warn message");
    error!("Error message");

    info!("Color mode: {:?}", config.color_mode());

    Ok(())
}
