use ulg::{parse, tokenize, TokenKind};

const LANGUAGE: &str = include_str!("../examples/ultimatex/ultimatex.ulg");

fn definition() -> ulg::LanguageDefinition {
    parse(LANGUAGE).unwrap()
}

#[test]
fn recognizes_integer_literal() {
    let tokens = tokenize(&definition(), "12345").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Number);
}

#[test]
fn recognizes_decimal_literal() {
    let tokens = tokenize(&definition(), "12.375").unwrap();
    assert_eq!(tokens[0].lexeme, "12.375");
    assert_eq!(tokens[0].kind, TokenKind::Number);
}

#[test]
fn recognizes_exponent_literal() {
    let tokens = tokenize(&definition(), "2.5e-3").unwrap();
    assert_eq!(tokens[0].lexeme, "2.5e-3");
}

#[test]
fn recognizes_hexadecimal_literal() {
    let tokens = tokenize(&definition(), "0xCAFE").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Number);
}

#[test]
fn recognizes_binary_literal() {
    let tokens = tokenize(&definition(), "0b101010").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Number);
}

#[test]
fn preserves_underscores_in_number() {
    let tokens = tokenize(&definition(), "1_000_000").unwrap();
    assert_eq!(tokens[0].lexeme, "1_000_000");
}

#[test]
fn recognizes_double_quoted_string() {
    let tokens = tokenize(&definition(), "\"hello world\"").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::String);
}

#[test]
fn recognizes_single_quoted_character() {
    let tokens = tokenize(&definition(), "'x'").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Character);
}

#[test]
fn permits_escaped_quote_inside_string() {
    let tokens = tokenize(&definition(), "\"say \\\"hello\\\"\"").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::String);
}

#[test]
fn rejects_unterminated_string() {
    let error = tokenize(&definition(), "\"not closed").unwrap_err();
    assert!(error.message.contains("unterminated"));
}

#[test]
fn rejects_newline_inside_string() {
    let error = tokenize(&definition(), "\"first\nsecond\"").unwrap_err();
    assert!(error.message.contains("newline"));
}

#[test]
fn rejects_incomplete_numeric_exponent() {
    let error = tokenize(&definition(), "4.2e").unwrap_err();
    assert!(error.message.contains("exponent"));
}

#[test]
fn recognizes_uppercase_hexadecimal_prefix() {
    let tokens = tokenize(&definition(), "0XCAFE").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Number);
}

#[test]
fn recognizes_escaped_character_literal() {
    let tokens = tokenize(&definition(), "'\\n'").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Character);
}

#[test]
fn number_span_tracks_start_column() {
    let tokens = tokenize(&definition(), "   42").unwrap();
    assert_eq!(tokens[0].span.column, 4);
}

#[test]
fn accepts_empty_string_literal() {
    let tokens = tokenize(&definition(), "\"\"").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::String);
}
