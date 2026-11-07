//! Implementation of the lex command.

use std::error::Error;
use std::fs;
use std::path::Path;

use crate::lexing;

use super::ensure_definition;

/// Tokenizes a source file and prints an aligned token table.
pub(super) fn run(
    definition_path: &Path,
    source_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let definition = ensure_definition(definition_path)?;
    let source = fs::read_to_string(source_path)?;
    let tokens = lexing::tokenize(&definition, &source)?;
    println!(
        "{:<14} {:<24} {:>6} {:>6}",
        "KIND", "LEXEME", "LINE", "COLUMN"
    );
    for token in &tokens {
        println!(
            "{:<14} {:<24} {:>6} {:>6}",
            token.kind,
            token.lexeme.escape_debug().to_string(),
            token.span.line,
            token.span.column
        );
    }
    println!();
    println!("{} token(s)", tokens.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_debug_is_available_for_lexeme_display() {
        let text = "line\nvalue";
        assert!(text.escape_debug().to_string().contains("\\n"));
    }
}
