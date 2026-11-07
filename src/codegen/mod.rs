//! Language artifact generators.
//!
//! Backends consume validated language definitions and emit deterministic
//! output. The Rust backend generates a complete reusable lexer, including
//! metadata, token and span types, literal scanning, comments, operators,
//! and custom punctuation matchers. Generated modules depend only on the
//! standard library and regex crate, not on the ULG implementation.

mod escaping;
mod lexer;
mod rust;

pub use lexer::generate_rust_lexer;
pub use rust::{generate_rust, RustGenerationOptions};

/// Supported code-generation targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Rust,
}

impl Backend {
    /// Returns the CLI spelling for this backend.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rust => "rust",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_is_the_first_supported_backend() {
        assert_eq!(Backend::Rust.as_str(), "rust");
    }
}
