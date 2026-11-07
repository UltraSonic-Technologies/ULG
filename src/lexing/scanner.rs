//! Source scanner implementing maximal-munch lexical recognition.
//!
//! Scanner order is deliberate: whitespace, comments, quoted literals,
//! numbers, keywords, identifiers, operators, and punctuation. A comment
//! prefix is checked before identifiers so language-specific markers can
//! begin with punctuation. Operators are sorted by descending byte length.

use regex::Regex;

use crate::diagnostics::SourceSpan;
use crate::model::{CommentDefinition, LanguageDefinition};
use super::{LexError, Token, TokenKind};

struct PunctuationMatcher {
    name: String,
    regex: Regex,
}

#[derive(Clone, Copy)]
struct Mark {
    offset: usize,
    line: usize,
    column: usize,
}

/// Stateful scanner over one UTF-8 source string.
pub(super) struct Scanner<'a> {
    definition: &'a LanguageDefinition,
    source: &'a str,
    offset: usize,
    line: usize,
    column: usize,
    comments: Vec<CommentDefinition>,
    operators: Vec<String>,
    punctuation: Vec<PunctuationMatcher>,
}

impl<'a> Scanner<'a> {
    /// Builds matcher tables from a validated language definition.
    pub fn new(
        definition: &'a LanguageDefinition,
        source: &'a str,
    ) -> Result<Self, LexError> {
        let comments = definition.comment_rules().map_err(|message| {
            LexError::new(message, SourceSpan::point(0, 1, 1))
        })?;
        let mut operators = definition.operators.clone();
        operators.sort_by(|left, right| {
            right.len().cmp(&left.len()).then_with(|| left.cmp(right))
        });
        let punctuation = compile_punctuation(definition)?;
        Ok(Self {
            definition,
            source,
            offset: 0,
            line: 1,
            column: 1,
            comments,
            operators,
            punctuation,
        })
    }

    fn remaining(&self) -> &'a str {
        &self.source[self.offset..]
    }

    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    fn peek_second(&self) -> Option<char> {
        self.remaining().chars().nth(1)
    }

    fn mark(&self) -> Mark {
        Mark {
            offset: self.offset,
            line: self.line,
            column: self.column,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.offset += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn advance_bytes(&mut self, bytes: usize) {
        let target = self.offset + bytes;
        while self.offset < target {
            self.advance();
        }
    }

    fn span(&self, mark: Mark) -> SourceSpan {
        SourceSpan::new(
            mark.offset,
            self.offset,
            mark.line,
            mark.column,
        )
    }

    fn emit(
        &self,
        tokens: &mut Vec<Token>,
        kind: TokenKind,
        mark: Mark,
        rule: Option<String>,
    ) {
        let lexeme = self.source[mark.offset..self.offset].to_owned();
        let mut token = Token::new(kind, lexeme, self.span(mark));
        token.rule = rule;
        tokens.push(token);
    }

    fn is_identifier_start(ch: char) -> bool {
        ch == '_' || ch == '@' || ch.is_alphabetic()
    }

    fn is_identifier_continue(ch: char) -> bool {
        ch == '_' || ch.is_alphanumeric()
    }

    fn keyword_at_cursor(&self) -> Option<String> {
        for keyword in &self.definition.keywords {
            if !self.remaining().starts_with(keyword) {
                continue;
            }
            let next = self.remaining()[keyword.len()..].chars().next();
            if next.is_some_and(Self::is_identifier_continue) {
                continue;
            }
            return Some(keyword.clone());
        }
        None
    }

    fn comment_boundary(&self, marker: &str) -> bool {
        let Some(last) = marker.chars().last() else {
            return false;
        };
        if !Self::is_identifier_continue(last) {
            return true;
        }
        self.remaining()[marker.len()..]
            .chars()
            .next()
            .is_none_or(|next| !Self::is_identifier_continue(next))
    }

    fn scan_comment(&mut self, tokens: &mut Vec<Token>) -> Result<bool, LexError> {
        let matched = self.comments.iter().find(|comment| {
            self.remaining().starts_with(&comment.opener)
                && self.comment_boundary(&comment.opener)
        }).cloned();
        let Some(comment) = matched else {
            return Ok(false);
        };
        let mark = self.mark();
        self.advance_bytes(comment.opener.len());
        if let Some(closer) = comment.closer {
            let Some(end) = self.remaining().find(&closer) else {
                return Err(LexError::new(
                    format!("unterminated block comment; expected {closer:?}"),
                    self.span(mark),
                ));
            };
            self.advance_bytes(end + closer.len());
        } else {
            while let Some(ch) = self.peek() {
                if ch == '\n' {
                    break;
                }
                self.advance();
            }
        }
        self.emit(tokens, TokenKind::Comment, mark, None);
        Ok(true)
    }

    fn scan_quoted(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexError> {
        let mark = self.mark();
        let quote = self.peek().expect("caller checked quote");
        self.advance();
        loop {
            let Some(ch) = self.peek() else {
                return Err(LexError::new("unterminated quoted literal", self.span(mark)));
            };
            if ch == quote {
                self.advance();
                break;
            }
            if ch == '\n' || ch == '\r' {
                return Err(LexError::new("newline in quoted literal", self.span(mark)));
            }
            if ch == '\\' {
                self.advance();
                if self.peek().is_none() {
                    return Err(LexError::new("incomplete escape sequence", self.span(mark)));
                }
            }
            self.advance();
        }
        let kind = if quote == '"' { TokenKind::String } else { TokenKind::Character };
        self.emit(tokens, kind, mark, None);
        Ok(())
    }

    fn scan_number(&mut self, tokens: &mut Vec<Token>) -> Result<(), LexError> {
        let mark = self.mark();
        if self.remaining().starts_with("0x") || self.remaining().starts_with("0X") {
            self.advance_bytes(2);
            let start = self.offset;
            while self.peek().is_some_and(|ch| ch.is_ascii_hexdigit() || ch == '_') {
                self.advance();
            }
            if self.offset == start {
                return Err(LexError::new("hexadecimal literal has no digits", self.span(mark)));
            }
        } else if self.remaining().starts_with("0b") || self.remaining().starts_with("0B") {
            self.advance_bytes(2);
            let start = self.offset;
            while self.peek().is_some_and(|ch| matches!(ch, '0' | '1' | '_')) {
                self.advance();
            }
            if self.offset == start {
                return Err(LexError::new("binary literal has no digits", self.span(mark)));
            }
        } else {
            while self.peek().is_some_and(|ch| ch.is_ascii_digit() || ch == '_') {
                self.advance();
            }
            if self.peek() == Some('.') && self.peek_second().is_some_and(|ch| ch.is_ascii_digit()) {
                self.advance();
                while self.peek().is_some_and(|ch| ch.is_ascii_digit() || ch == '_') {
                    self.advance();
                }
            }
            if matches!(self.peek(), Some('e' | 'E')) {
                self.advance();
                if matches!(self.peek(), Some('+' | '-')) {
                    self.advance();
                }
                let start = self.offset;
                while self.peek().is_some_and(|ch| ch.is_ascii_digit() || ch == '_') {
                    self.advance();
                }
                if self.offset == start {
                    return Err(LexError::new("numeric exponent has no digits", self.span(mark)));
                }
            }
        }
        self.emit(tokens, TokenKind::Number, mark, None);
        Ok(())
    }

    fn scan_identifier(&mut self, tokens: &mut Vec<Token>) {
        let mark = self.mark();
        self.advance();
        while self.peek().is_some_and(Self::is_identifier_continue) {
            self.advance();
        }
        self.emit(tokens, TokenKind::Identifier, mark, None);
    }

    fn scan_operator(&mut self, tokens: &mut Vec<Token>) -> bool {
        let Some(operator) = self.operators.iter()
            .find(|item| self.remaining().starts_with(item.as_str()))
            .cloned()
        else {
            return false;
        };
        let mark = self.mark();
        self.advance_bytes(operator.len());
        self.emit(tokens, TokenKind::Operator, mark, None);
        true
    }

    fn scan_punctuation(&mut self, tokens: &mut Vec<Token>) -> bool {
        let mut best: Option<(usize, String)> = None;
        for matcher in &self.punctuation {
            if let Some(found) = matcher.regex.find(self.remaining()) {
                if found.start() == 0 && found.end() > 0
                    && best.as_ref().is_none_or(|(length, _)| found.end() > *length)
                {
                    best = Some((found.end(), matcher.name.clone()));
                }
            }
        }
        let Some((length, rule)) = best else {
            return false;
        };
        let mark = self.mark();
        self.advance_bytes(length);
        self.emit(tokens, TokenKind::Punctuation, mark, Some(rule));
        true
    }

    /// Scans the complete source and returns tokens in source order.
    pub fn scan(mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
                continue;
            }
            if self.scan_comment(&mut tokens)? {
                continue;
            }
            if ch == '"' || ch == '\'' {
                self.scan_quoted(&mut tokens)?;
                continue;
            }
            if ch.is_ascii_digit() {
                self.scan_number(&mut tokens)?;
                continue;
            }
            if let Some(keyword) = self.keyword_at_cursor() {
                let mark = self.mark();
                self.advance_bytes(keyword.len());
                self.emit(&mut tokens, TokenKind::Keyword, mark, None);
                continue;
            }
            if Self::is_identifier_start(ch) {
                self.scan_identifier(&mut tokens);
                continue;
            }
            if self.scan_operator(&mut tokens) || self.scan_punctuation(&mut tokens) {
                continue;
            }
            return Err(LexError::new(
                format!("unexpected character {ch:?}"),
                SourceSpan::point(self.offset, self.line, self.column),
            ));
        }
        Ok(tokens)
    }
}

fn compile_punctuation(
    definition: &LanguageDefinition,
) -> Result<Vec<PunctuationMatcher>, LexError> {
    let mut result = Vec::new();
    for rule in &definition.punctuation {
        if let Some((open, close)) = rule.delimiter_pair() {
            for (suffix, character) in [("OPEN", open), ("CLOSE", close)] {
                let regex = Regex::new(&format!("\\A{}", regex::escape(&character.to_string())))
                    .map_err(|error| LexError::new(error.to_string(), SourceSpan::point(0, 1, 1)))?;
                result.push(PunctuationMatcher {
                    name: format!("{}_{}", rule.name, suffix),
                    regex,
                });
            }
        } else {
            let regex = Regex::new(&format!("\\A(?:{})", rule.pattern))
                .map_err(|error| LexError::new(
                    format!("invalid punctuation regex {}: {error}", rule.name),
                    SourceSpan::point(0, 1, 1),
                ))?;
            if regex.find("").is_some_and(|found| found.end() == 0) {
                return Err(LexError::new(
                    format!("punctuation regex {} matches empty text", rule.name),
                    SourceSpan::point(0, 1, 1),
                ));
            }
            result.push(PunctuationMatcher {
                name: rule.name.clone(),
                regex,
            });
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{LanguageVersion, PunctuationDefinition};

    fn definition() -> LanguageDefinition {
        let mut value = LanguageDefinition::new(
            "Tiny", "Test language", LanguageVersion::new(vec![1, 0]).unwrap()
        );
        value.keywords = vec!["fn".into(), "return".into()];
        value.comments = vec!["//".into(), "/* */".into()];
        value.operators = vec!["=".into(), "==".into(), "+".into()];
        value.punctuation = vec![
            PunctuationDefinition::new("PARENS", r"\(\)"),
            PunctuationDefinition::new("BRACES", r"\{\}"),
            PunctuationDefinition::new("SEMICOLON", ";"),
        ];
        value
    }

    #[test]
    fn recognizes_keyword_identifier_and_operator() {
        let tokens = tokenize(&definition(), "fn main() { return x==1; }").unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Keyword);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert!(tokens.iter().any(|token| token.lexeme == "=="));
    }

    #[test]
    fn recognizes_comments_and_quoted_values() {
        let tokens = tokenize(&definition(), "var x = \"ok\"; // note").unwrap();
        assert!(tokens.iter().any(|token| token.kind == TokenKind::String));
        assert!(tokens.iter().any(|token| token.kind == TokenKind::Comment));
    }

    #[test]
    fn rejects_unterminated_block_comment() {
        assert!(tokenize(&definition(), "/* never closed").is_err());
    }
}
