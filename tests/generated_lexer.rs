use ulg::codegen::{generate_rust_lexer, RustGenerationOptions};
use ulg::{parse, validate};

const ULTIMATEX: &str = include_str!("../examples/ultimatex/ultimatex.ulg");

fn generated() -> String {
    let language = parse(ULTIMATEX).expect("the checked-in UltimateX definition parses");
    validate(&language).expect("the checked-in UltimateX definition validates");
    generate_rust_lexer(&language, &RustGenerationOptions {
        module_name: "ultimatex".into(),
        include_documentation: true,
    }).expect("the Rust lexer is generated")
}

#[test]
fn generated_file_contains_public_tokenizer_api() {
    let source = generated();
    for symbol in [
        "pub enum TokenKind",
        "pub struct Span",
        "pub struct Token",
        "pub struct LexError",
        "pub fn tokenize(source: &str)",
        "pub fn is_keyword(value: &str)",
        "pub fn longest_operator(value: &str)",
        "pub fn format_tokens(tokens: &[Token])",
    ] {
        assert!(source.contains(symbol), "missing generated API: {symbol}");
    }
}

#[test]
fn generated_file_embeds_all_ultimatex_declarations() {
    let source = generated();
    for declaration in [
        "UltimateX",
        "string_path",
        "@ifnotdefined",
        "@mc",
        "==",
        "PARENS",
        "SEMICOLON",
        "DOT",
    ] {
        assert!(source.contains(declaration), "missing declaration: {declaration}");
    }
}

#[test]
fn generated_operator_table_is_longest_first() {
    let source = generated();
    let double_equals = source.find(r#""==""#).expect("== is generated");
    let single_equals = source.find(r#"    "=","#).expect("= is generated");
    assert!(double_equals < single_equals);
}

#[test]
fn generated_module_has_no_unresolved_template_markers() {
    assert!(!generated().contains("@@"));
}

#[test]
fn generated_source_is_reproducible() {
    assert_eq!(generated(), generated());
}

#[test]
fn generated_source_documents_external_regex_dependency() {
    let source = generated();
    assert!(source.contains("use regex::Regex;"));
    assert!(source.contains("PUNCTUATION_MATCHERS"));
}
