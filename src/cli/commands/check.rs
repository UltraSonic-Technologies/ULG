//! Implementation of the check command.

use std::error::Error;
use std::path::Path;

use super::{display_path, ensure_definition};

/// Validates a definition without generating output.
pub(super) fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let definition = ensure_definition(path)?;
    let summary = definition.summary();
    println!("ULG definition is valid.");
    println!("File: {}", display_path(path));
    println!("Language: {}", summary.name);
    println!("Version: {}", summary.version);
    println!("Keywords: {}", summary.keyword_count);
    println!("Comments: {}", summary.comment_count);
    println!("Operators: {}", summary.operator_count);
    println!("Punctuation: {}", summary.punctuation_count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_handler_is_a_result_returning_operation() {
        let function: fn(&Path) -> Result<(), Box<dyn Error>> = run;
        let _ = function;
    }
}
