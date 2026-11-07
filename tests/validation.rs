use ulg::{parse, validate, LanguageDefinition, PunctuationDefinition};
use ulg::model::LanguageVersion;

const LANGUAGE: &str = include_str!("../examples/ultimatex/ultimatex.ulg");

fn minimal() -> LanguageDefinition {
    let mut value = LanguageDefinition::new(
        "TestLanguage",
        "Validation fixture",
        LanguageVersion::new(vec![1, 0]).unwrap(),
    );
    value.keywords.push("fn".into());
    value.operators.push("=".into());
    value.comments.push("//".into());
    value
}

#[test]
fn ultimatex_definition_is_valid() {
    let definition = parse(LANGUAGE).unwrap();
    assert!(validate(&definition).is_ok());
}

#[test]
fn rejects_empty_name() {
    let mut value = minimal();
    value.name.clear();
    assert!(validate(&value).is_err());
}

#[test]
fn rejects_empty_description() {
    let mut value = minimal();
    value.description.clear();
    assert!(validate(&value).is_err());
}

#[test]
fn rejects_empty_keyword() {
    let mut value = minimal();
    value.keywords.push(String::new());
    assert!(validate(&value).is_err());
}

#[test]
fn rejects_duplicate_keyword_case_insensitively() {
    let mut value = minimal();
    value.keywords.push("FN".into());
    assert!(validate(&value).is_err());
}

#[test]
fn rejects_duplicate_operators() {
    let mut value = minimal();
    value.operators.push("=".into());
    assert!(validate(&value).is_err());
}

#[test]
fn rejects_keyword_that_is_also_operator() {
    let mut value = minimal();
    value.operators.push("fn".into());
    assert!(validate(&value).is_err());
}

#[test]
fn rejects_malformed_comment_definition() {
    let mut value = minimal();
    value.comments.push("one two three".into());
    assert!(validate(&value).is_err());
}

#[test]
fn rejects_invalid_custom_regex() {
    let mut value = minimal();
    value.punctuation.push(PunctuationDefinition::new("CUSTOM", "("));
    assert!(validate(&value).is_err());
}

#[test]
fn accepts_known_delimiter_pair() {
    let mut value = minimal();
    value.punctuation.push(PunctuationDefinition::new("PARENS", r"\(\)"));
    assert!(validate(&value).is_ok());
}

#[test]
fn rejects_lowercase_punctuation_name() {
    let mut value = minimal();
    value.punctuation.push(PunctuationDefinition::new("comma", ","));
    assert!(validate(&value).is_err());
}
