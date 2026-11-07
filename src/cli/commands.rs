//! Command dispatch and shared file-loading operations.
//!
//! File loading is centralized so every command parses and validates the
//! definition in exactly the same way. This prevents inspect, lex, and
//! generate from accepting different subsets of the format.

use std::error::Error;
use std::fs;
use std::path::Path;

use super::Command;
pub(super) use super::args::OutputBackend;
use crate::{parser, validate};

mod check;
mod generate;
mod inspect;
mod lex;

/// Loads a ULG file and runs semantic validation.
pub(super) fn load_validated(
    path: &Path,
) -> Result<crate::LanguageDefinition, Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    let definition = parser::parse(&source)?;
    validate::validate(&definition)?;
    Ok(definition)
}

/// Executes a CLI command.
pub(super) fn execute(command: Command) -> Result<(), Box<dyn Error>> {
    match command {
        Command::Check { definition } => check::run(&definition),
        Command::Inspect { definition } => inspect::run(&definition),
        Command::Lex { definition, source } => lex::run(&definition, &source),
        Command::Generate { backend, definition, out, module } => {
            generate::run(backend, &definition, &out, &module)
        }
    }
}

/// Parses and validates before a command starts doing work.
pub(super) fn ensure_definition(
    path: &Path,
) -> Result<crate::LanguageDefinition, Box<dyn Error>> {
    load_validated(path)
}

/// Shared path display helper for consistent command output.
pub(super) fn display_path(path: &Path) -> String {
    path.display().to_string()
}
