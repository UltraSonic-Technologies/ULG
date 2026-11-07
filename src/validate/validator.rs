//! Definition-level semantic validation and aggregated errors.

use std::fmt;

use crate::model::LanguageDefinition;

use super::rules;

/// A group of semantic problems found in one language definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub problems: Vec<String>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "language definition is invalid:")?;
        for problem in &self.problems {
            writeln!(f, "  - {problem}")?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationError {}

/// Validates a definition and returns all detected semantic problems.
pub fn validate(definition: &LanguageDefinition) -> Result<(), ValidationError> {
    let mut problems = Vec::new();

    if definition.name.trim().is_empty() {
        problems.push("LANG_NAME must not be empty".into());
    }
    if definition.description.trim().is_empty() {
        problems.push("LANG_DESC must not be empty".into());
    }
    if definition.keywords.is_empty() {
        problems.push("KEYWORDS must contain at least one entry".into());
    }
    if definition.operators.is_empty() {
        problems.push("OPERATORS must contain at least one entry".into());
    }

    rules::check_unique_non_empty(&definition.keywords, "KEYWORDS", &mut problems);
    rules::check_unique_non_empty(&definition.comments, "COMMENTS", &mut problems);
    rules::check_unique_non_empty(&definition.operators, "OPERATORS", &mut problems);
    rules::check_lexical_conflicts(definition, &mut problems);
    rules::check_comments(definition, &mut problems);
    rules::check_punctuation(definition, &mut problems);
    rules::check_punctuation_names(definition, &mut problems);

    if problems.is_empty() {
        Ok(())
    } else {
        Err(ValidationError { problems })
    }
}

/// Runs validation and also returns non-fatal recommendations.
pub fn validate_with_warnings(
    definition: &LanguageDefinition,
) -> (Result<(), ValidationError>, Vec<String>) {
    let result = validate(definition);
    let mut warnings = Vec::new();

    if definition.comments.is_empty() {
        warnings.push("no comment delimiters are declared".into());
    }
    if definition.punctuation.is_empty() {
        warnings.push("no punctuation rules are declared".into());
    }
    if definition.keywords.iter().any(|word| word.contains(' ')) {
        warnings.push("a keyword contains whitespace and may be difficult to tokenize".into());
    }

    (result, warnings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::LanguageVersion;

    #[test]
    fn rejects_empty_name() {
        let definition = LanguageDefinition::new(
            "",
            "Description",
            LanguageVersion::new(vec![1, 0]).unwrap(),
        );
        assert!(validate(&definition).is_err());
    }

    #[test]
    fn rejects_invalid_regex() {
        let mut definition = LanguageDefinition::default();
        definition.name = "Sample".into();
        definition.description = "A sample language".into();
        definition.keywords.push("fn".into());
        definition.operators.push("=".into());
        definition.punctuation.push(
            crate::model::PunctuationDefinition::new("CUSTOM", "(")
        );
        assert!(validate(&definition).is_err());
    }
}
