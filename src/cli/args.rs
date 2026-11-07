//! Command-line argument model.
//!
//! Commands are explicit and predictable so shell scripts can rely on stable
//! subcommand names. File paths are PathBuf values and are never interpreted
//! as shell fragments.

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

/// Ultra Language Grammar command-line application.
#[derive(Debug, Parser)]
#[command(name = "ulg", version, about = "Validate and generate language tooling from .ulg files")]
pub struct Cli {
    /// Command to execute.
    #[command(subcommand)]
    pub command: Command,
}

/// Available ULG operations.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Parse and semantically validate a language definition.
    Check {
        /// Input ULG definition.
        definition: PathBuf,
    },
    /// Print metadata and all declared lexical rules.
    Inspect {
        /// Input ULG definition.
        definition: PathBuf,
    },
    /// Tokenize a source file using a ULG definition.
    Lex {
        /// Language definition.
        definition: PathBuf,
        /// Source file to tokenize.
        source: PathBuf,
    },
    /// Generate source artifacts for a selected backend.
    Generate {
        /// Output backend.
        #[arg(value_enum)]
        backend: OutputBackend,
        /// Input ULG definition.
        definition: PathBuf,
        /// Output directory or file, depending on the backend.
        #[arg(long, default_value = "generated")]
        out: PathBuf,
        /// Module identifier used in generated output.
        #[arg(long, default_value = "language_definition")]
        module: String,
    },
}

/// Output backend supported by the generator.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputBackend {
    Rust,
}
