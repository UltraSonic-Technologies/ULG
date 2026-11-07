//! Command-line entry point.
//!
//! Argument parsing and dispatch live in the library so handlers can be
//! tested independently from the process. This binary only maps success or
//! failure to an operating-system exit code.

use std::process::ExitCode;

fn main() -> ExitCode {
    match ulg::cli::run(std::env::args_os()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("ulg: {error}");
            ExitCode::FAILURE
        }
    }
}
