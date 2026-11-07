//! Language artifact generators.
//!
//! Backends consume validated language definitions and emit deterministic
//! output. The first backend generates a Rust module containing metadata and
//! lexical tables. Generated modules are data-oriented and intentionally do
//! not depend on the ULG crate at runtime.

mod escaping;
mod rust;

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
