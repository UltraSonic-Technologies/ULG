//! Structured diagnostics and stable diagnostic codes.
//!
//! Codes are intended to remain stable even when diagnostic wording changes.
//! Tooling should prefer the code for machine handling and the message for
//! interactive display. Optional notes explain a likely correction.

use std::fmt;

use super::SourceSpan;

/// Importance of a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

/// Stable category identifier for a ULG diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticCode {
    Syntax,
    MissingMetadata,
    InvalidVersion,
    UnknownSection,
    DuplicateSection,
    DuplicateEntry,
    EmptyEntry,
    InvalidComment,
    InvalidRegex,
    LexicalError,
    Io,
}

impl DiagnosticCode {
    /// Returns the stable textual code used by CLI consumers.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Syntax => "ULG001",
            Self::MissingMetadata => "ULG002",
            Self::InvalidVersion => "ULG003",
            Self::UnknownSection => "ULG004",
            Self::DuplicateSection => "ULG005",
            Self::DuplicateEntry => "ULG006",
            Self::EmptyEntry => "ULG007",
            Self::InvalidComment => "ULG008",
            Self::InvalidRegex => "ULG009",
            Self::LexicalError => "ULG010",
            Self::Io => "ULG011",
        }
    }
}

/// A user-facing message with a source location and optional explanation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: DiagnosticCode,
    pub message: String,
    pub span: Option<SourceSpan>,
    pub note: Option<String>,
}

impl Diagnostic {
    /// Creates an error without an associated source position.
    pub fn error(code: DiagnosticCode, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code,
            message: message.into(),
            span: None,
            note: None,
        }
    }

    /// Associates a diagnostic with a source location.
    pub fn at(mut self, span: SourceSpan) -> Self {
        self.span = Some(span);
        self
    }

    /// Adds a secondary explanation.
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    /// Returns true for diagnostics that should fail a command.
    pub const fn is_error(&self) -> bool {
        matches!(self.severity, Severity::Error)
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}: {}",
            self.code.as_str(),
            match self.severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
                Severity::Note => "note",
            },
            self.message
        )?;
        if let Some(span) = self.span {
            write!(f, " at {}:{}", span.line, span.column)?;
        }
        if let Some(note) = &self.note {
            write!(f, "\n  note: {note}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Diagnostic {}
