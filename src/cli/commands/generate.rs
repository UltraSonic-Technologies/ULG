//! Implementation of the generate command.

use std::error::Error;
use std::fs;
use std::path::Path;

use super::{OutputBackend, ensure_definition};

/// Generates a complete backend artifact into the requested output location.
///
/// Existing files are protected by default. This avoids silently replacing
/// handwritten source when a caller accidentally points ULG at the wrong
/// destination.
pub(super) fn run(
    backend: OutputBackend,
    definition_path: &Path,
    output: &Path,
    module: &str,
) -> Result<(), Box<dyn Error>> {
    let definition = ensure_definition(definition_path)?;

    match backend {
        OutputBackend::Rust => {
            let source = crate::codegen::generate_rust_lexer(
                &definition,
                &crate::codegen::RustGenerationOptions {
                    module_name: module.to_owned(),
                    include_documentation: true,
                },
            )?;

            let destination = if output.extension().is_some() {
                output.to_path_buf()
            } else {
                output.join(format!("{}.rs", module.replace('-', "_")))
            };

            if destination.exists() {
                return Err(format!(
                    "refusing to overwrite existing file {}; choose another --out path",
                    destination.display()
                ).into());
            }

            if let Some(parent) = destination.parent()
                && !parent.as_os_str().is_empty()
            {
                fs::create_dir_all(parent)?;
            }

            fs::write(&destination, source)?;
            println!("Generated standalone Rust lexer: {}", destination.display());
            println!("Generated language: {}", definition.name);
            println!("Keywords: {}", definition.keywords.len());
            println!("Comments: {}", definition.comments.len());
            println!("Operators: {}", definition.operators.len());
            println!("Punctuation rules: {}", definition.punctuation.len());
            println!("Generated code requires the regex crate.");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extension_detection_works_for_output_file() {
        assert!(Path::new("language.rs").extension().is_some());
        assert!(Path::new("generated").extension().is_none());
    }

    #[test]
    fn module_names_are_made_file_system_safe() {
        let filename = format!("{}.rs", "my-language".replace('-', "_"));
        assert_eq!(filename, "my_language.rs");
    }
}
