//! Complete Rust lexer generator.
//!
//! This backend emits a reusable tokenizer, not only metadata tables. The
//! generated module has no dependency on the ULG crate itself; it uses the
//! regex crate for user-defined punctuation expressions. Consumers should
//! add regex = "1" to the generated project's dependencies.

use crate::model::LanguageDefinition;

use super::escaping::{rust_string, rust_string_array};
use super::RustGenerationOptions;

fn comment_array(definition: &LanguageDefinition) -> Result<String, String> {
    let rules = definition
        .comment_rules()
        .map_err(|error| format!("cannot generate malformed comment rules: {error}"))?;

    let entries = rules
        .iter()
        .map(|rule| match rule.closer.as_deref() {
            Some(closer) => format!(
                "    ({}, Some({})),",
                rust_string(&rule.opener),
                rust_string(closer)
            ),
            None => format!("    ({}, None),", rust_string(&rule.opener)),
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(format!("[\n{entries}\n]"))
}

fn punctuation_array(definition: &LanguageDefinition) -> String {
    let entries = definition
        .punctuation
        .iter()
        .map(|rule| {
            format!(
                "    ({}, {}),",
                rust_string(&rule.name),
                rust_string(&rule.pattern)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!("[\n{entries}\n]")
}

/// Generates a standalone Rust lexer module from a validated definition.
///
/// The resulting source includes language metadata, lexical tables, token
/// and span types, comment handling, literal scanning, numeric scanning,
/// maximal-munch operators, and custom punctuation matchers.
///
/// The generated module uses regex for custom punctuation definitions.
pub fn generate_rust_lexer(
    definition: &LanguageDefinition,
    options: &RustGenerationOptions,
) -> Result<String, String> {
    let mut output = include_str!("lexer_template.rs.in").to_owned();

    let operators = definition
        .operators_longest_first()
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();

    let replacements = [
        ("@@MODULE_NAME@@", rust_string(&options.module_name)),
        ("@@LANGUAGE_NAME@@", rust_string(&definition.name)),
        (
            "@@LANGUAGE_DESCRIPTION@@",
            rust_string(&definition.description),
        ),
        (
            "@@LANGUAGE_VERSION@@",
            rust_string(&definition.version.to_string()),
        ),
        ("@@KEYWORDS@@", rust_string_array(&definition.keywords)),
        ("@@COMMENTS@@", comment_array(definition)?),
        ("@@OPERATORS@@", rust_string_array(&operators)),
        ("@@PUNCTUATION@@", punctuation_array(definition)),
    ];

    for (marker, value) in replacements {
        output = output.replace(marker, &value);
    }

    if output.contains("@@") {
        return Err("internal Rust lexer template contains an unresolved marker".into());
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{LanguageVersion, PunctuationDefinition};

    fn sample() -> LanguageDefinition {
        let mut definition = LanguageDefinition::new(
            "Mini",
            "A generated language",
            LanguageVersion::new(vec![1, 2, 3]).unwrap(),
        );
        definition.keywords = vec!["fn".into(), "return".into()];
        definition.comments = vec!["//".into(), "/* */".into()];
        definition.operators = vec!["=".into(), "==".into(), "+".into()];
        definition.punctuation = vec![
            PunctuationDefinition::new("PARENS", r"\\(\\)"),
            PunctuationDefinition::new("SEMICOLON", ";"),
        ];
        definition
    }

    #[test]
    fn emits_a_tokenizer_not_only_tables() {
        let generated = generate_rust_lexer(
            &sample(),
            &RustGenerationOptions::default(),
        )
        .unwrap();

        assert!(generated.contains("pub fn tokenize(source: &str)"));
        assert!(generated.contains("struct Scanner"));
        assert!(generated.contains("pub enum TokenKind"));
        assert!(generated.contains("fn scan_comment"));
        assert!(generated.contains("fn scan_number"));
    }

    #[test]
    fn emits_language_metadata_and_lexical_tables() {
        let generated = generate_rust_lexer(
            &sample(),
            &RustGenerationOptions::default(),
        )
        .unwrap();

        assert!(generated.contains("LANGUAGE_NAME: &str = \"Mini\""));
        assert!(generated.contains("\"fn\""));
        assert!(generated.contains("Some(\"*/\")"));
        assert!(generated.contains("\"==\""));
        assert!(generated.contains("SEMICOLON"));
    }

    #[test]
    fn output_is_deterministic() {
        let options = RustGenerationOptions::default();
        let definition = sample();

        assert_eq!(
            generate_rust_lexer(&definition, &options).unwrap(),
            generate_rust_lexer(&definition, &options).unwrap()
        );
    }

    #[test]
    fn rejects_unresolved_template_markers() {
        let generated = generate_rust_lexer(
            &sample(),
            &RustGenerationOptions::default(),
        )
        .unwrap();

        assert!(!generated.contains("@@"));
    }
}
