//! Implementation of the inspect command.

use std::error::Error;
use std::path::Path;

use super::ensure_definition;

/// Prints all declarations in a validated language definition.
pub(super) fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let definition = ensure_definition(path)?;
    println!("Language: {}", definition.name);
    println!("Description: {}", definition.description);
    println!("Version: {}", definition.version);
    println!();
    print_section("KEYWORDS", &definition.keywords);
    print_section("COMMENTS", &definition.comments);
    print_section("OPERATORS", &definition.operators);
    println!("PUNCTUATION:");
    for rule in &definition.punctuation {
        println!("  {} -> {:?}", rule.name, rule.pattern);
    }
    Ok(())
}

fn print_section(name: &str, values: &[String]) {
    println!("{name}:");
    for value in values {
        println!("  {value}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_section_accepts_empty_values() {
        print_section("EMPTY", &[]);
    }
}
