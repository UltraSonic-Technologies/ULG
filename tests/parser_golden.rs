use ulg::{parse, validate};

const ULTIMATEX: &str = include_str!("../examples/ultimatex/ultimatex.ulg");

#[test]
fn reads_language_name() {
    let language = parse(ULTIMATEX).unwrap();
    assert_eq!(language.name, "UltimateX");
}

#[test]
fn reads_language_description() {
    let language = parse(ULTIMATEX).unwrap();
    assert_eq!(
        language.description,
        "The language for games and GUIs using Ultra2D-X"
    );
}

#[test]
fn reads_dotted_version() {
    let language = parse(ULTIMATEX).unwrap();
    assert_eq!(language.version.to_string(), "1.0.0");
    assert_eq!(language.version.major(), 1);
    assert_eq!(language.version.minor(), 0);
    assert_eq!(language.version.patch(), 0);
}

#[test]
fn extracts_keyword_list_in_source_order() {
    let language = parse(ULTIMATEX).unwrap();
    assert_eq!(language.keywords.first().map(String::as_str), Some("var"));
    assert_eq!(language.keywords.last().map(String::as_str), Some("global"));
    assert!(language.keywords.contains(&"namespace".to_owned()));
    assert!(language.keywords.contains(&"@import".to_owned()));
}

#[test]
fn extracts_comment_declarations() {
    let language = parse(ULTIMATEX).unwrap();
    assert_eq!(language.comments, ["@c", "@mc @mc"]);
}

#[test]
fn extracts_all_eighteen_operators() {
    let language = parse(ULTIMATEX).unwrap();
    assert_eq!(language.operators.len(), 18);
    assert!(language.operators.contains(&"==".to_owned()));
    assert!(language.operators.contains(&"->".to_owned()));
    assert!(language.operators.contains(&"%=".to_owned()));
}

#[test]
fn extracts_named_punctuation_patterns() {
    let language = parse(ULTIMATEX).unwrap();
    let names: Vec<&str> = language.punctuation
        .iter()
        .map(|item| item.name.as_str())
        .collect();
    assert_eq!(
        names,
        ["PARENS", "BRACKETS", "BRACES", "SEMICOLON", "COMMA", "COLON", "DOT"]
    );
}

#[test]
fn parses_regular_expression_escapes() {
    let language = parse(ULTIMATEX).unwrap();
    assert_eq!(language.punctuation[0].pattern, r"\(\)");
    assert_eq!(language.punctuation[1].pattern, r"\[\]");
    assert_eq!(language.punctuation[2].pattern, r"\{\}");
}

#[test]
fn definition_passes_semantic_validation() {
    let language = parse(ULTIMATEX).unwrap();
    assert!(validate(&language).is_ok());
}

#[test]
fn summary_reports_declaration_counts() {
    let language = parse(ULTIMATEX).unwrap();
    let summary = language.summary();
    assert_eq!(summary.name, "UltimateX");
    assert_eq!(summary.keyword_count, 51);
    assert_eq!(summary.comment_count, 2);
    assert_eq!(summary.operator_count, 18);
    assert_eq!(summary.punctuation_count, 7);
}

#[test]
fn keywords_are_case_sensitive() {
    let language = parse(ULTIMATEX).unwrap();
    assert!(language.is_keyword("True"));
    assert!(!language.is_keyword("true"));
}

#[test]
fn maximal_munch_order_is_stable() {
    let language = parse(ULTIMATEX).unwrap();
    let operators = language.operators_longest_first();
    assert_eq!(operators[0].len(), 2);
    assert!(operators.windows(2).all(|pair| pair[0].len() >= pair[1].len()));
}
