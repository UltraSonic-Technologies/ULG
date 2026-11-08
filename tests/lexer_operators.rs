use ulg::{parse, tokenize, TokenKind};

const LANGUAGE: &str = include_str!("../examples/ultimatex/ultimatex.ulg");

fn definition() -> ulg::LanguageDefinition {
    parse(LANGUAGE).unwrap()
}

#[test]
fn recognizes_compound_equality_before_assignment() {
    let tokens = tokenize(&definition(), "a==b").unwrap();
    assert_eq!(tokens[1].lexeme, "==");
    assert_eq!(tokens[1].kind, TokenKind::Operator);
}

#[test]
fn recognizes_arrow_operator() {
    let tokens = tokenize(&definition(), "value->member").unwrap();
    assert_eq!(tokens[1].lexeme, "->");
}

#[test]
fn recognizes_compound_assignment_operators() {
    let tokens = tokenize(&definition(), "a+=1 b-=2 c*=3 d/=4 e%=5").unwrap();
    let values: Vec<&str> = tokens.iter()
        .filter(|token| token.kind == TokenKind::Operator)
        .map(|token| token.lexeme.as_str())
        .collect();

    // Maximal munch must produce each compound assignment as one token.
    assert_eq!(values, ["+=", "-=", "*=", "/=", "%="]);
}

#[test]
fn recognizes_single_character_operators() {
    let tokens = tokenize(&definition(), "a+b-c*d/e%f").unwrap();
    assert!(tokens.iter().any(|token| token.lexeme == "+"));
    assert!(tokens.iter().any(|token| token.lexeme == "-"));
    assert!(tokens.iter().any(|token| token.lexeme == "*"));
}

#[test]
fn matches_longest_operator_at_each_position() {
    let tokens = tokenize(&definition(), "a>=b").unwrap();
    assert_eq!(tokens[1].lexeme, ">=");
}

#[test]
fn operators_do_not_require_surrounding_whitespace() {
    let tokens = tokenize(&definition(), "left=right").unwrap();
    assert_eq!(tokens[1].lexeme, "=");
}

#[test]
fn operators_remain_separate_from_identifiers() {
    let tokens = tokenize(&definition(), "first+second").unwrap();
    assert_eq!(tokens[0].kind, TokenKind::Identifier);
    assert_eq!(tokens[1].kind, TokenKind::Operator);
    assert_eq!(tokens[2].kind, TokenKind::Identifier);
}

#[test]
fn ampersand_is_a_declared_operator() {
    let tokens = tokenize(&definition(), "left&right").unwrap();
    assert_eq!(tokens[1].lexeme, "&");
}

#[test]
fn unknown_symbol_is_reported() {
    let error = tokenize(&definition(), "a ? b").unwrap_err();
    assert!(error.message.contains("unexpected character"));
}

#[test]
fn tokenization_order_matches_source_order() {
    let tokens = tokenize(&definition(), "a==b->c").unwrap();
    let lexemes: Vec<&str> = tokens.iter().map(|token| token.lexeme.as_str()).collect();
    assert_eq!(lexemes, ["a", "==", "b", "->", "c"]);
}

#[test]
fn line_and_column_point_to_operator_start() {
    let tokens = tokenize(&definition(), "\n  a==b").unwrap();
    assert_eq!(tokens[1].span.line, 2);
    assert_eq!(tokens[1].span.column, 4);
}
