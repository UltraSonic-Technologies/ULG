//! CLI interface for validation, inspection, lexing, and generation.
//!
//! The run function is generic over argument iterators to make command
//! dispatch testable. Errors are returned to the binary entry point, which
//! controls process exit status and standard-error formatting.

mod args;
mod commands;

pub use args::{Cli, Command, OutputBackend};

use clap::Parser;
use std::error::Error;
use std::ffi::OsString;

/// Parses arguments, executes one command, and returns any failure.
pub fn run<I, T>(arguments: I) -> Result<(), Box<dyn Error>>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let cli = Cli::try_parse_from(arguments)?;
    commands::execute(cli.command)
}

/// Executes the process argument list.
pub fn run_env() -> Result<(), Box<dyn Error>> {
    run(std::env::args_os())
}
