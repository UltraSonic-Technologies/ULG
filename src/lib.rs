//! ULG is a language-definition toolkit.
//!
//! The library separates the ULG document format from the language being
//! described. A parsed definition is data; validation turns that data into a
//! trustworthy specification; the lexer consumes source text using that
//! specification; and code generation emits a portable Rust representation.
//!
//! The processing pipeline is:
//! 1. Parse a ULG document into a LanguageDefinition.
//! 2. Validate semantic constraints.
//! 3. Tokenize source text using the validated definition.
//! 4. Generate a Rust module containing the lexical tables.
//!
//! ULG files are declarative input and are never executed. Treat definitions
//! as untrusted data and validate before using them in build pipelines.

pub mod cli;
pub mod codegen;
pub mod diagnostics;
pub mod lexing;
pub mod model;
pub mod parser;
pub mod validate;

pub use diagnostics::{Diagnostic, DiagnosticCode, Severity, SourceSpan};
pub use lexing::{LexError, Token, TokenKind, tokenize};
pub use model::{LanguageDefinition, PunctuationDefinition};
pub use parser::{ParseError, parse};
pub use validate::{ValidationError, validate};

/// Version of the ULG implementation, independent from language versions.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Canonical extension for Ultra Language Grammar documents.
pub const FILE_EXTENSION: &str = "ulg";

/// Returns the implementation version.
pub fn version() -> &'static str {
    VERSION
}
