//! Character cursor used by the hand-written ULG parser.
//!
//! The cursor advances only on UTF-8 character boundaries. Offsets are byte
//! offsets, while columns count Unicode scalar values. Parsing helpers keep
//! delimiter recognition in one place so sections share error behavior.

use super::ParseError;

/// Mutable read position inside a UTF-8 document.
pub(super) struct Cursor<'a> {
    source: &'a str,
    offset: usize,
    line: usize,
    column: usize,
}

impl<'a> Cursor<'a> {
    /// Creates a cursor at the beginning of the source.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            offset: 0,
            line: 1,
            column: 1,
        }
    }

    /// Returns the unconsumed suffix of the source.
    pub fn remaining(&self) -> &'a str {
        &self.source[self.offset..]
    }

    /// Returns true after all source characters have been consumed.
    pub fn is_eof(&self) -> bool {
        self.offset >= self.source.len()
    }

    /// Returns the next Unicode scalar value.
    pub fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    /// Consumes one character and updates line and column coordinates.
    pub fn advance(&mut self) -> Option<char> {
        let character = self.peek()?;
        self.offset += character.len_utf8();
        if character == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(character)
    }

    /// Consumes consecutive whitespace characters.
    pub fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.advance();
        }
    }

    /// Returns a location-aware error at the current position.
    pub fn error(&self, message: impl Into<String>) -> ParseError {
        ParseError::new(message, self.line, self.column, self.offset)
    }

    /// Consumes an exact delimiter after optional whitespace.
    pub fn expect(&mut self, expected: &str) -> Result<(), ParseError> {
        self.skip_whitespace();
        if self.remaining().starts_with(expected) {
            for _ in expected.chars() {
                self.advance();
            }
            Ok(())
        } else {
            Err(self.error(format!("expected {expected:?}")))
        }
    }

    /// Consumes a delimiter without skipping leading whitespace.
    pub fn expect_adjacent(&mut self, expected: &str) -> Result<(), ParseError> {
        if self.remaining().starts_with(expected) {
            for _ in expected.chars() {
                self.advance();
            }
            Ok(())
        } else {
            Err(self.error(format!("expected {expected:?}")))
        }
    }

    /// Reads a quoted string and preserves unknown regular-expression escapes.
    pub fn read_string(&mut self) -> Result<String, ParseError> {
        self.skip_whitespace();
        self.expect_adjacent("\"")?;
        let mut output = String::new();
        loop {
            let Some(character) = self.advance() else {
                return Err(self.error("unterminated string literal"));
            };
            match character {
                '"' => return Ok(output),
                '\\' => {
                    let Some(escaped) = self.advance() else {
                        return Err(self.error("incomplete escape sequence"));
                    };
                    match escaped {
                        '"' => output.push('"'),
                        '\\' => output.push('\\'),
                        'n' => output.push('\n'),
                        'r' => output.push('\r'),
                        't' => output.push('\t'),
                        other => {
                            output.push('\\');
                            output.push(other);
                        }
                    }
                }
                '\n' | '\r' => {
                    return Err(self.error("newline inside quoted string"));
                }
                other => output.push(other),
            }
        }
    }

    /// Reads a version token up to whitespace or a closing parenthesis.
    pub fn read_atom(&mut self) -> Result<String, ParseError> {
        self.skip_whitespace();
        let start = self.offset;
        while let Some(character) = self.peek() {
            if character.is_whitespace() || character == ')' {
                break;
            }
            self.advance();
        }
        if self.offset == start {
            Err(self.error("expected a value"))
        } else {
            Ok(self.source[start..self.offset].to_owned())
        }
    }

    /// Reads an uppercase section identifier.
    pub fn read_identifier(&mut self) -> Result<String, ParseError> {
        self.skip_whitespace();
        let start = self.offset;
        while self.peek().is_some_and(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_') {
            self.advance();
        }
        if self.offset == start {
            Err(self.error("expected an uppercase identifier"))
        } else {
            Ok(self.source[start..self.offset].to_owned())
        }
    }

    /// Consumes a comma if present after optional whitespace.
    pub fn consume_comma(&mut self) -> bool {
        self.skip_whitespace();
        if self.peek() == Some(',') {
            self.advance();
            true
        } else {
            false
        }
    }
}
