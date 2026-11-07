//! Concrete ULG document reader.
//!
//! The document reader matches the syntax used by language authors:
//! initialization markers, metadata tuples, named sections, LIST arrays,
//! MATCH maps, section endings, and the document ending. Whitespace and
//! indentation are insignificant between structural tokens. String values
//! retain regular-expression escapes such as backslash parentheses.

use std::str::FromStr;

use crate::model::{LanguageDefinition, LanguageVersion, PunctuationDefinition};

use super::{cursor::Cursor, ParseError};

/// Parses one complete ULG document.
pub fn parse(source: &str) -> Result<LanguageDefinition, ParseError> {
    let mut cursor = Cursor::new(source);
    cursor.expect("{")?;
    cursor.expect("[!INIT]")?;

    let name = parse_metadata_string(&mut cursor, "LANG_NAME")?;
    let description = parse_metadata_string(&mut cursor, "LANG_DESC")?;
    let version_text = parse_metadata_atom(&mut cursor, "LANG_VER")?;
    let version = LanguageVersion::from_str(&version_text)
        .map_err(|error| cursor.error(error.to_string()))?;

    cursor.expect("(")?;
    cursor.expect("{")?;

    let mut definition = LanguageDefinition::new(name, description, version);
    let mut sections = Vec::new();

    loop {
        cursor.skip_whitespace();
        if cursor.remaining().starts_with('}') {
            cursor.expect("}")?;
            break;
        }
        if cursor.is_eof() {
            return Err(cursor.error("unterminated sections container"));
        }
        let section_name = parse_section_name(&mut cursor)?;
        if sections.iter().any(|seen| seen == &section_name) {
            return Err(cursor.error(format!("duplicate section {section_name}")));
        }
        sections.push(section_name.clone());
        match section_name.as_str() {
            "KEYWORDS" => definition.keywords = parse_list_section(&mut cursor)?,
            "COMMENTS" => definition.comments = parse_list_section(&mut cursor)?,
            "OPERATORS" => definition.operators = parse_list_section(&mut cursor)?,
            "PUNCTUATION" => definition.punctuation = parse_punctuation_section(&mut cursor)?,
            other => return Err(cursor.error(format!("unknown section {other}"))),
        }
    }

    cursor.expect(")")?;
    cursor.expect("[!END]")?;
    cursor.expect("}")?;
    cursor.skip_whitespace();
    if !cursor.is_eof() {
        return Err(cursor.error("unexpected content after document end"));
    }
    Ok(definition)
}

fn parse_metadata_string(
    cursor: &mut Cursor<'_>,
    key: &str,
) -> Result<String, ParseError> {
    cursor.expect("(")?;
    cursor.expect(key)?;
    let value = cursor.read_string()?;
    cursor.expect(")")?;
    Ok(value)
}

fn parse_metadata_atom(
    cursor: &mut Cursor<'_>,
    key: &str,
) -> Result<String, ParseError> {
    cursor.expect("(")?;
    cursor.expect(key)?;
    let value = cursor.read_atom()?;
    cursor.expect(")")?;
    Ok(value)
}

fn parse_section_name(cursor: &mut Cursor<'_>) -> Result<String, ParseError> {
    cursor.expect("[")?;
    let name = cursor.read_identifier()?;
    cursor.expect("]")?;
    cursor.expect(":")?;
    cursor.expect("(!INIT)")?;
    Ok(name)
}

fn parse_list_section(cursor: &mut Cursor<'_>) -> Result<Vec<String>, ParseError> {
    cursor.expect("(")?;
    cursor.expect("LIST")?;
    cursor.expect("[")?;
    let mut values = Vec::new();
    cursor.skip_whitespace();

    if !cursor.remaining().starts_with(']') {
        loop {
            values.push(cursor.read_string()?);
            if !cursor.consume_comma() {
                break;
            }
            cursor.skip_whitespace();
            if cursor.remaining().starts_with(']') {
                break;
            }
        }
    }

    cursor.expect("]")?;
    cursor.expect(")")?;
    cursor.expect("(!END)")?;
    Ok(values)
}

fn parse_punctuation_section(
    cursor: &mut Cursor<'_>,
) -> Result<Vec<PunctuationDefinition>, ParseError> {
    cursor.expect("(")?;
    cursor.expect("MATCH")?;
    cursor.expect("{")?;
    let mut rules = Vec::new();
    loop {
        cursor.skip_whitespace();
        if cursor.remaining().starts_with('}') {
            cursor.expect("}")?;
            cursor.expect(")")?;
            break;
        }
        if cursor.is_eof() {
            return Err(cursor.error("unterminated MATCH block"));
        }
        let name = cursor.read_identifier()?;
        cursor.expect("->")?;
        let pattern = cursor.read_string()?;
        rules.push(PunctuationDefinition::new(name, pattern));
        let _ = cursor.consume_comma();
    }
    cursor.expect("(!END)")?;
    Ok(rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINIMAL: &str = r#"
    {
        [!INIT]
        (LANG_NAME "Tiny")
        (LANG_DESC "A tiny language")
        (LANG_VER 1.0.0)
        ({
            [KEYWORDS]:(!INIT)
            (LIST[
                "fn", "return",
            ])
            (!END)
            [COMMENTS]:(!INIT)
            (LIST["//", "/* */"])
            (!END)
            [OPERATORS]:(!INIT)
            (LIST["+", "=", "=="])
            (!END)
            [PUNCTUATION]:(!INIT)
            (MATCH{
                PARENS -> "\\(\\)",
                BRACES -> "\\{\\}"
            })
            (!END)
        })
        [!END]
    }
    "#;

    #[test]
    fn parses_all_sections() {
        let definition = parse(MINIMAL).unwrap();
        assert_eq!(definition.name, "Tiny");
        assert_eq!(definition.keywords, ["fn", "return"]);
        assert_eq!(definition.comments, ["//", "/* */"]);
        assert_eq!(definition.operators, ["+", "=", "=="]);
        assert_eq!(definition.punctuation.len(), 2);
    }

    #[test]
    fn preserves_unknown_regular_expression_escapes() {
        let definition = parse(MINIMAL).unwrap();
        assert_eq!(definition.punctuation[0].pattern, r"\(\)");
    }

    #[test]
    fn rejects_trailing_input() {
        assert!(parse(&format!("{MINIMAL} trailing")).is_err());
    }

    #[test]
    fn allows_whitespace_between_list_delimiters() {
        let formatted = MINIMAL.replace("(LIST[", "( LIST [");
        assert!(parse(&formatted).is_ok());
    }

    #[test]
    fn allows_whitespace_between_match_delimiters() {
        let formatted = MINIMAL.replace("(MATCH{", "( MATCH {");
        assert!(parse(&formatted).is_ok());
    }
}
