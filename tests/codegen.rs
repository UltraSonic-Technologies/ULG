use ulg::codegen::{generate_rust, RustGenerationOptions};
use ulg::{parse, LanguageDefinition};

const LANGUAGE: &str = include_str!("../examples/ultimatex/ultimatex.ulg");

#[test]
fn generates_language_identity() {
    let definition = parse(LANGUAGE).unwrap();
    let output = generate_rust(&definition, &RustGenerationOptions::default());
    assert!(output.contains("LANGUAGE_NAME"));
    assert!(output.contains("UltimateX"));
}

#[test]
fn generates_metadata_version() {
    let definition = parse(LANGUAGE).unwrap();
    let output = generate_rust(&definition, &RustGenerationOptions::default());
    assert!(output.contains("LANGUAGE_VERSION"));
    assert!(output.contains("1.0.0"));
}

#[test]
fn generates_keyword_table() {
    let definition = parse(LANGUAGE).unwrap();
    let output = generate_rust(&definition, &RustGenerationOptions::default());
    assert!(output.contains("KEYWORDS"));
    assert!(output.contains("namespace"));
}

#[test]
fn generates_operator_table() {
    let definition = parse(LANGUAGE).unwrap();
    let output = generate_rust(&definition, &RustGenerationOptions::default());
    assert!(output.contains("OPERATORS"));
    assert!(output.contains("+="));
}

#[test]
fn generates_comment_table() {
    let definition = parse(LANGUAGE).unwrap();
    let output = generate_rust(&definition, &RustGenerationOptions::default());
    assert!(output.contains("@c"));
    assert!(output.contains("@mc @mc"));
}

#[test]
fn generates_punctuation_table() {
    let definition = parse(LANGUAGE).unwrap();
    let output = generate_rust(&definition, &RustGenerationOptions::default());
    assert!(output.contains("PUNCTUATION"));
    assert!(output.contains("PARENS"));
}

#[test]
fn emits_longest_operator_helper() {
    let definition = parse(LANGUAGE).unwrap();
    let output = generate_rust(&definition, &RustGenerationOptions::default());
    assert!(output.contains("longest_operator"));
    assert!(output.contains("max_by_key"));
}

#[test]
fn emits_keyword_lookup_helper() {
    let definition = parse(LANGUAGE).unwrap();
    let output = generate_rust(&definition, &RustGenerationOptions::default());
    assert!(output.contains("is_keyword"));
}

#[test]
fn output_is_deterministic() {
    let definition = parse(LANGUAGE).unwrap();
    let options = RustGenerationOptions::default();
    assert_eq!(
        generate_rust(&definition, &options),
        generate_rust(&definition, &options)
    );
}

#[test]
fn custom_module_name_is_preserved() {
    let definition = LanguageDefinition::default();
    let options = RustGenerationOptions {
        module_name: "my_language".into(),
        include_documentation: true,
    };
    let output = generate_rust(&definition, &options);
    assert!(output.contains("my_language"));
}

#[test]
fn can_disable_generated_documentation() {
    let definition = LanguageDefinition::default();
    let options = RustGenerationOptions {
        module_name: "lang".into(),
        include_documentation: false,
    };
    let output = generate_rust(&definition, &options);
    assert!(output.starts_with("#![allow(dead_code)]"));
}
