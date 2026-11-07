use ulg::parse;

const VALID: &str = include_str!("../examples/minimal/minimal.ulg");

#[test]
fn rejects_missing_root_opening() {
    let malformed = VALID.replacen("{", "", 1);
    assert!(parse(&malformed).is_err());
}

#[test]
fn rejects_missing_initialization_marker() {
    let malformed = VALID.replace("[!INIT]", "[INIT]");
    assert!(parse(&malformed).is_err());
}

#[test]
fn rejects_missing_language_name() {
    let malformed = VALID.replace("(LANG_NAME \"Minimal\")", "(LANG_TITLE \"Minimal\")");
    assert!(parse(&malformed).is_err());
}

#[test]
fn rejects_unterminated_metadata_string() {
    let malformed = VALID.replace("\"Minimal\"", "\"Minimal)");
    assert!(parse(&malformed).is_err());
}

#[test]
fn rejects_invalid_numeric_version() {
    let malformed = VALID.replace("1.0", "one.two");
    assert!(parse(&malformed).is_err());
}

#[test]
fn rejects_missing_list_closer() {
    let malformed = VALID.replace("])", "]");
    assert!(parse(&malformed).is_err());
}

#[test]
fn rejects_missing_section_end() {
    let malformed = VALID.replacen("(!END)", "", 1);
    assert!(parse(&malformed).is_err());
}

#[test]
fn rejects_duplicate_sections() {
    let duplicate = r#"
    {
      [!INIT]
      (LANG_NAME "Dup")
      (LANG_DESC "Duplicate section")
      (LANG_VER 1.0)
      ({
        [KEYWORDS]:(!INIT)(LIST["if"])(!END)
        [KEYWORDS]:(!INIT)(LIST["else"])(!END)
        [COMMENTS]:(!INIT)(LIST["//"])(!END)
        [OPERATORS]:(!INIT)(LIST["="])(!END)
        [PUNCTUATION]:(!INIT)(MATCH{PARENS -> "\\(\\)" })(!END)
      })
      [!END]
    }
    "#;
    assert!(parse(duplicate).is_err());
}

#[test]
fn rejects_unknown_sections() {
    let unknown = VALID.replace("[KEYWORDS]", "[RESERVED]");
    assert!(parse(&unknown).is_err());
}

#[test]
fn rejects_trailing_content() {
    let malformed = format!("{VALID}\nnot part of the document");
    assert!(parse(&malformed).is_err());
}

#[test]
fn rejects_unterminated_match_block() {
    let malformed = VALID.replacen("})", "}", 1);
    assert!(parse(&malformed).is_err());
}

#[test]
fn accepts_whitespace_between_list_delimiters() {
    let formatted = VALID.replace("(LIST[", "( LIST [");
    assert!(parse(&formatted).is_ok());
}

#[test]
fn accepts_whitespace_between_match_delimiters() {
    let formatted = VALID.replace("(MATCH{", "( MATCH {");
    assert!(parse(&formatted).is_ok());
}

#[test]
fn rejects_missing_document_end_marker() {
    let malformed = VALID.replace("[!END]", "[END]");
    assert!(parse(&malformed).is_err());
}

#[test]
fn rejects_missing_container_closer() {
    let malformed = VALID.replacen("})", "}", 1);
    assert!(parse(&malformed).is_err());
}
