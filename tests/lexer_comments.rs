use ulg::{parse, tokenize, TokenKind};

const LANGUAGE: &str = include_str!("../examples/ultimatex/ultimatex.ulg");

fn definition() -> ulg::LanguageDefinition {
    parse(LANGUAGE).unwrap()
}

#[test]
fn recognizes_line_comment() {
    let tokens = tokenize(&definition(), "@c hello").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Comment);
    assert_eq!(tokens[0].lexeme, "@c hello");
}

#[test]
fn recognizes_block_comment() {
    let tokens = tokenize(&definition(), "@mc hidden @mc").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Comment);
}

#[test]
fn line_comment_stops_before_newline() {
    let tokens = tokenize(&definition(), "@c note\nvar").unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].kind, TokenKind::Comment);
    assert_eq!(tokens[1].kind, TokenKind::Keyword);
}

#[test]
fn block_comment_can_contain_newlines() {
    let tokens = tokenize(&definition(), "@mc first\nsecond @mc var").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Comment);
    assert_eq!(tokens[1].kind, TokenKind::Keyword);
}

#[test]
fn rejects_unterminated_block_comment() {
    let error = tokenize(&definition(), "@mc never closed").unwrap_err();
    assert!(error.message.contains("unterminated block comment"));
}

#[test]
fn comment_markers_inside_strings_are_not_comments() {
    let tokens = tokenize(&definition(), "\"@c text\"").unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::String);
}

#[test]
fn comments_preserve_their_original_text() {
    let tokens = tokenize(&definition(), "@c keep this text").unwrap();
    assert_eq!(tokens[0].lexeme, "@c keep this text");
}

#[test]
fn block_comment_consumes_closing_marker() {
    let tokens = tokenize(&definition(), "@mc x @mc var").unwrap();
    assert_eq!(tokens[0].lexeme, "@mc x @mc");
    assert_eq!(tokens[1].lexeme, "var");
}

#[test]
fn comments_are_skipped_only_as_whitespace_by_callers() {
    let tokens = tokenize(&definition(), "var @c note\nname").unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].kind, TokenKind::Keyword);
    assert_eq!(tokens[1].kind, TokenKind::Comment);
    assert_eq!(tokens[2].kind, TokenKind::Identifier);
}

#[test]
fn source_line_updates_after_multiline_comment() {
    let tokens = tokenize(&definition(), "@mc one\ntwo @mc\nvar").unwrap();
    assert_eq!(tokens[1].span.line, 3);
}
