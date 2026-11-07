//! Rust source generator for language definitions.
//!
//! The generated file contains constants and pure lookup functions. It can
//! be included by downstream applications without linking the ULG
//! implementation. Output is stable for the same language definition.

use crate::model::LanguageDefinition;

use super::escaping::{rust_doc_text, rust_string, rust_string_array};

/// Options controlling Rust artifact generation.
#[derive(Debug, Clone)]
pub struct RustGenerationOptions {
    pub module_name: String,
    pub include_documentation: bool,
}

impl Default for RustGenerationOptions {
    fn default() -> Self {
        Self {
            module_name: "language_definition".into(),
            include_documentation: true,
        }
    }
}

/// Generates a self-contained Rust module for a validated definition.
pub fn generate_rust(
    definition: &LanguageDefinition,
    options: &RustGenerationOptions,
) -> String {
    let mut output = String::new();
    output.push_str("#![allow(dead_code)]\n");
    if options.include_documentation {
        output.push_str(&rust_doc_text(&format!(
            "Generated ULG language table for {}.\nDo not edit by hand; regenerate from the source ULG file.",
            definition.name
        )));
        output.push('\n');
    }
    output.push_str(&format!(
        "pub const MODULE_NAME: &str = {};\n",
        rust_string(&options.module_name)
    ));
    output.push_str(&format!(
        "pub const LANGUAGE_NAME: &str = {};\n",
        rust_string(&definition.name)
    ));
    output.push_str(&format!(
        "pub const LANGUAGE_DESCRIPTION: &str = {};\n",
        rust_string(&definition.description)
    ));
    output.push_str(&format!(
        "pub const LANGUAGE_VERSION: &str = {};\n",
        rust_string(&definition.version.to_string())
    ));
    output.push_str(&format!(
        "pub const KEYWORDS: &[&str] = &{};\n",
        rust_string_array(&definition.keywords)
    ));
    output.push_str(&format!(
        "pub const OPERATORS: &[&str] = &{};\n",
        rust_string_array(&definition.operators)
    ));

    let comments = definition.comments
        .iter()
        .map(|value| format!("    {},", rust_string(value)))
        .collect::<Vec<_>>()
        .join("\n");
    output.push_str(&format!("pub const COMMENTS: &[&str] = &[\n{comments}\n];\n"));

    let punctuation = definition.punctuation.iter()
        .map(|rule| format!(
            "    ({}, {}),",
            rust_string(&rule.name),
            rust_string(&rule.pattern)
        ))
        .collect::<Vec<_>>()
        .join("\n");
    output.push_str("pub const PUNCTUATION: &[(&str, &str)] = &[\n");
    output.push_str(&punctuation);
    output.push_str("\n];\n");

    output.push_str(
        "pub fn is_keyword(value: &str) -> bool { KEYWORDS.contains(&value) }\n"
    );
    output.push_str(
        "pub fn longest_operator(value: &str) -> Option<&'static str> {\n"
    );
    output.push_str(
        "    OPERATORS.iter().copied().filter(|op| value.starts_with(op)).max_by_key(|op| op.len())\n"
    );
    output.push_str("}\n");
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::LanguageVersion;

    #[test]
    fn output_contains_language_metadata_and_lookup() {
        let definition = LanguageDefinition::new(
            "Tiny",
            "Test language",
            LanguageVersion::new(vec![1, 0]).unwrap(),
        );
        let generated = generate_rust(&definition, &RustGenerationOptions::default());
        assert!(generated.contains("LANGUAGE_NAME"));
        assert!(generated.contains("longest_operator"));
        assert!(generated.contains("Tiny"));
    }

    #[test]
    fn generated_source_starts_with_module_attribute() {
        let definition = LanguageDefinition::default();
        let generated = generate_rust(&definition, &RustGenerationOptions::default());
        assert!(generated.starts_with("#![allow(dead_code)]"));
    }
}
