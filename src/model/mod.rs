//! Typed representation of a parsed ULG language definition.
//!
//! Model types contain no file-system behavior and do not perform parsing.
//! They are intentionally easy to construct in tests and embedding
//! applications. Validation rules live in the validate module.

mod comment;
mod language;
mod punctuation;
mod version;

pub use comment::{CommentDefinition, CommentKind};
pub use language::{LanguageDefinition, LanguageSummary};
pub use punctuation::PunctuationDefinition;
pub use version::{LanguageVersion, VersionError};

/// A named section present in a ULG document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SectionKind {
    Keywords,
    Comments,
    Operators,
    Punctuation,
}

impl SectionKind {
    /// Returns the exact section name used in ULG documents.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Keywords => "KEYWORDS",
            Self::Comments => "COMMENTS",
            Self::Operators => "OPERATORS",
            Self::Punctuation => "PUNCTUATION",
        }
    }

    /// Returns the canonical declaration order used by formatters.
    pub const fn order(self) -> u8 {
        match self {
            Self::Keywords => 0,
            Self::Comments => 1,
            Self::Operators => 2,
            Self::Punctuation => 3,
        }
    }
}

/// Metadata shared by language definitions and generated artifacts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageMetadata {
    pub name: String,
    pub description: String,
    pub version: LanguageVersion,
}

impl LanguageMetadata {
    /// Creates metadata without bypassing the version parser.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        version: LanguageVersion,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            version,
        }
    }
}
