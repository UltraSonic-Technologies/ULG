//! Implementation of the generate command.

use std::error::Error;
use std::fs;
use std::path::Path;

use super::{OutputBackend, ensure_definition};

/// Generates a backend artifact into the requested output location.
pub(super) fn run(
    backend: OutputBackend,
    definition_path: &Path,
    output: &Path,
    module: &str,
) -> Result<(), Box<dyn Error>> {
    let definition = ensure_definition(definition_path)?;
    match backend {
        OutputBackend::Rust => {
            let source = crate::codegen::generate_rust(
                &definition,
                &crate::codegen::RustGenerationOptions {
                    module_name: module.to_owned(),
                    include_documentation: true,
                },
            );
            let destination = if output.extension().is_some() {
                output.to_path_buf()
            } else {
                output.join(format!("{}.rs", module.replace('-', "_")))
            };
            if let Some(parent) = destination.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }
            fs::write(&destination, source)?;
            println!("Generated Rust language tables: {}", destination.display());
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
}
