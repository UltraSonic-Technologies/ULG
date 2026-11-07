//! Individual semantic checks used by the validator.
//!
//! Each check is side-effect-free and appends human-readable errors. The
//! validator composes these checks so editor integrations can present the
//! full set of problems instead of stopping at the first issue.

use std::collections::HashSet;

use regex::Regex;

use crate::model::LanguageDefinition;

/// Checks that a list contains no empty or repeated entries.
pub fn check_unique_non_empty(
    values: &[String],
    label: &str,
    errors: &mut Vec<String>,
) {
    let mut seen = HashSet::new();
    for value in values {
        if value.trim().is_empty() {
            errors.push(format!("{label} contains an empty value"));
        }
        if !seen.insert(value.to_lowercase()) {
            errors.push(format!("{label} contains duplicate value: {value}"));
        }
    }
}

/// Checks lexical declarations for conflicting keyword and operator values.
pub fn check_lexical_conflicts(
    definition: &LanguageDefinition,
    errors: &mut Vec<String>,
) {
    let operators: HashSet<&str> =
        definition.operators.iter().map(String::as_str).collect();
    for keyword in &definition.keywords {
        if operators.contains(keyword.as_str()) {
            errors.push(format!(
                "keyword '{keyword}' is also declared as an operator"
            ));
        }
    }
}

/// Checks each declared comment delimiter for consistency.
pub fn check_comments(definition: &LanguageDefinition, errors: &mut Vec<String>) {
    for comment in &definition.comments {
        match comment.split_whitespace().count() {
            1 | 2 => {}
            _ => errors.push(format!(
                "comment declaration must contain one or two delimiters: {comment}"
            )),
        }
    }
}

/// Checks custom punctuation patterns for regex syntax errors.
pub fn check_punctuation(definition: &LanguageDefinition, errors: &mut Vec<String>) {
    for punctuation in &definition.punctuation {
        if punctuation.name.is_empty() {
            errors.push("punctuation rule has an empty name".into());
            continue;
        }
        if punctuation.is_delimiter_pair() {
            continue;
        }
        if let Err(error) = Regex::new(&punctuation.pattern) {
            errors.push(format!(
                "punctuation rule '{}' has invalid regex: {error}",
                punctuation.name
            ));
        }
    }
}

/// Checks that every custom name follows the ULG uppercase convention.
pub fn check_punctuation_names(
    definition: &LanguageDefinition,
    errors: &mut Vec<String>,
) {
    for punctuation in &definition.punctuation {
        if !punctuation.name.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_') {
            errors.push(format!(
                "punctuation name '{}' must use uppercase ASCII letters",
                punctuation.name
            ));
        }
    }
}
