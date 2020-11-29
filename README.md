# Clap Logging

Simple logging for command-line utilities written using Clap.

Why not use any of the [existing solutions][]? Well, they didn't do exactly what we wanted, and because exactly what we wanted could be implemented in ≈200 lines of code, we decided to do it ourselves!

In particular, both env_logger and pretty_env_logger use the `RUST_LOG` environment variable. We want to use a program-specific variable to control the log level. Furthermore, we do not want to expose to users that our application is written in Rust. We are proud that it is written in Rust, but the language of implementation is not something with which an end user should be concerned.

[existing solutions]: https://github.com/rust-lang/log#in-executables
