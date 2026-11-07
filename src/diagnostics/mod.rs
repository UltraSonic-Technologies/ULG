//! Diagnostic primitives shared by parsing, validation, and lexing.
//!
//! Every diagnostic can identify a location in the original document.
//! Byte offsets are used for UTF-8 slicing; line and column are one-based
//! coordinates intended for terminal and editor display.

mod diagnostic;
mod span;

pub use diagnostic::{Diagnostic, DiagnosticCode, Severity};
pub use span::SourceSpan;

/// Collection alias used by routines that report multiple findings.
pub type Diagnostics = Vec<Diagnostic>;

/// Formats a group of diagnostics with one diagnostic per line.
pub fn format_all(diagnostics: &[Diagnostic]) -> String {
    diagnostics
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}
