//! Semantic validation for language definitions.
//!
//! Parsing answers whether the document has valid structure. Validation
//! answers whether the resulting language definition is internally
//! consistent and safe to pass to the lexer or code generator.

mod rules;
mod validator;

pub use validator::{ValidationError, validate, validate_with_warnings};

/// Returns true when a name can be represented as a language identifier.
pub fn valid_language_name(value: &str) -> bool {
    let mut chars = value.chars();
    matches!(chars.next(), Some(first) if first.is_ascii_alphabetic() || first == '_')
        && chars.all(|character| character.is_ascii_alphanumeric() || character == '_' || character == '-')
}

/// Returns true when a lexical word contains no whitespace.
pub fn is_single_lexeme(value: &str) -> bool {
    !value.is_empty() && !value.chars().any(char::is_whitespace)
}
