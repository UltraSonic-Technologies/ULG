use ulg::{parse, tokenize, TokenKind};

const LANGUAGE: &str = include_str!("../examples/ultimatex/ultimatex.ulg");

fn definition() -> ulg::LanguageDefinition {
    parse(LANGUAGE).unwrap()
}

#[test]
fn recognizes_declared_keyword() {
    let tokens = tokenize(&definition(), "fn").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Keyword);
}

#[test]
fn recognizes_ultimatex_macro_keywords() {
    let tokens = tokenize(&definition(), "@import @define @require").unwrap();
    assert_eq!(tokens.len(), 3);
    assert!(tokens.iter().all(|token| token.kind == TokenKind::Keyword));
}

#[test]
fn recognizes_boolean_keywords_with_original_case() {
    let tokens = tokenize(&definition(), "True False Null").unwrap();
    assert!(tokens.iter().all(|token| token.kind == TokenKind::Keyword));
    assert_eq!(tokens[0].lexeme, "True");
}

#[test]
fn recognizes_identifier_after_keyword() {
    let tokens = tokenize(&definition(), "fn functionName").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Keyword);
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
}

#[test]
fn does_not_split_keyword_prefix_inside_identifier() {
    let tokens = tokenize(&definition(), "namespaceTools").unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
}

#[test]
fn keyword_matching_is_case_sensitive() {
    let tokens = tokenize(&definition(), "True true").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Keyword);
    assert_eq!(tokens[1].kind, TokenKind::Identifier);
}

#[test]
fn recognizes_unicode_identifier() {
    let tokens = tokenize(&definition(), "café").unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
}

#[test]
fn recognizes_underscore_identifier() {
    let tokens = tokenize(&definition(), "_private_value").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
}

#[test]
fn recognizes_at_prefixed_identifier_when_not_keyword() {
    let tokens = tokenize(&definition(), "@local_macro").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
}

#[test]
fn preserves_original_identifier_spelling() {
    let tokens = tokenize(&definition(), "MixedCase").unwrap();
    assert_eq!(tokens[0].lexeme, "MixedCase");
}

#[test]
fn identifiers_keep_source_order() {
    let tokens = tokenize(&definition(), "first second third").unwrap();
    let names: Vec<&str> = tokens.iter().map(|token| token.lexeme.as_str()).collect();
    assert_eq!(names, ["first", "second", "third"]);
}

#[test]
fn keyword_tokens_have_source_spans() {
    let tokens = tokenize(&definition(), "\n  fn").unwrap();
    assert_eq!(tokens[0].span.line, 2);
    assert_eq!(tokens[0].span.column, 3);
}
