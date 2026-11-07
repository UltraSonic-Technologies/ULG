use ulg::{parse, tokenize, TokenKind};

const LANGUAGE: &str = include_str!("../examples/ultimatex/ultimatex.ulg");

fn definition() -> ulg::LanguageDefinition {
    parse(LANGUAGE).unwrap()
}

#[test]
fn recognizes_parentheses_as_two_tokens() {
    let tokens = tokenize(&definition(), "()").unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::Punctuation);
    assert_eq!(tokens[0].lexeme, "(");
    assert_eq!(tokens[1].lexeme, ")");
}

#[test]
fn recognizes_brackets_as_two_tokens() {
    let tokens = tokenize(&definition(), "[]").unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].lexeme, "[");
    assert_eq!(tokens[1].lexeme, "]");
}

#[test]
fn recognizes_braces_as_two_tokens() {
    let tokens = tokenize(&definition(), "{}").unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].lexeme, "{");
    assert_eq!(tokens[1].lexeme, "}");
}

#[test]
fn recognizes_semicolon_rule() {
    let tokens = tokenize(&definition(), ";").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Punctuation);
    assert_eq!(tokens[0].rule.as_deref(), Some("SEMICOLON"));
}

#[test]
fn recognizes_comma_rule() {
    let tokens = tokenize(&definition(), ",").unwrap();
    assert_eq!(tokens[0].rule.as_deref(), Some("COMMA"));
}

#[test]
fn recognizes_colon_rule() {
    let tokens = tokenize(&definition(), ":").unwrap();
    assert_eq!(tokens[0].rule.as_deref(), Some("COLON"));
}

#[test]
fn recognizes_dot_rule() {
    let tokens = tokenize(&definition(), ".").unwrap();
    assert_eq!(tokens[0].rule.as_deref(), Some("DOT"));
}

#[test]
fn punctuation_rules_compose_with_operators() {
    let tokens = tokenize(&definition(), "a->b;").unwrap();
    assert_eq!(tokens[1].kind, TokenKind::Operator);
    assert_eq!(tokens[2].kind, TokenKind::Identifier);
    assert_eq!(tokens[3].kind, TokenKind::Punctuation);
}

#[test]
fn reports_invalid_custom_regular_expression() {
    let mut language = definition();
    language.punctuation.push(
        ulg::PunctuationDefinition::new("BROKEN", "(")
    );
    assert!(tokenize(&language, "x").is_err());
}

#[test]
fn reports_zero_width_custom_pattern() {
    let mut language = definition();
    language.punctuation.push(
        ulg::PunctuationDefinition::new("EMPTY", "a*")
    );
    assert!(tokenize(&language, "x").is_err());
}
