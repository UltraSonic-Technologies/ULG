//! Comment delimiter declarations.
//!
//! A single marker describes a line comment. Two whitespace-separated
//! markers describe a block comment. The original spelling is retained so
//! inspection and code generation remain reproducible.

/// Operational form of a comment declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommentKind {
    /// Starts at the marker and continues to the end of the line.
    Line,
    /// Starts at the opening marker and ends at the closing marker.
    Block,
}

/// Parsed comment delimiters, retaining the source declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentDefinition {
    pub source: String,
    pub kind: CommentKind,
    pub opener: String,
    pub closer: Option<String>,
}

impl CommentDefinition {
    /// Parses a one-marker line comment or two-marker block comment.
    pub fn parse(source: &str) -> Result<Self, String> {
        let parts: Vec<&str> = source.split_whitespace().collect();
        match parts.as_slice() {
            [opener] if !opener.is_empty() => Ok(Self {
                source: source.to_owned(),
                kind: CommentKind::Line,
                opener: (*opener).to_owned(),
                closer: None,
            }),
            [opener, closer] if !opener.is_empty() && !closer.is_empty() => {
                Ok(Self {
                    source: source.to_owned(),
                    kind: CommentKind::Block,
                    opener: (*opener).to_owned(),
                    closer: Some((*closer).to_owned()),
                })
            }
            _ => Err(format!(
                "comment declaration must contain one or two markers: {source}"
            )),
        }
    }

    /// Indicates whether the declaration is a line comment.
    pub const fn is_line(&self) -> bool {
        matches!(self.kind, CommentKind::Line)
    }

    /// Indicates whether the declaration is a block comment.
    pub const fn is_block(&self) -> bool {
        matches!(self.kind, CommentKind::Block)
    }

    /// Returns the byte length of the opening delimiter.
    pub fn opener_len(&self) -> usize {
        self.opener.len()
    }

    /// Returns the closing delimiter length, if present.
    pub fn closer_len(&self) -> usize {
        self.closer.as_ref().map_or(0, String::len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_line_comment() {
        let comment = CommentDefinition::parse("@c").unwrap();
        assert_eq!(comment.kind, CommentKind::Line);
        assert_eq!(comment.opener, "@c");
        assert!(comment.closer.is_none());
    }

    #[test]
    fn parses_block_comment() {
        let comment = CommentDefinition::parse("@mc @mc").unwrap();
        assert_eq!(comment.kind, CommentKind::Block);
        assert_eq!(comment.closer.as_deref(), Some("@mc"));
    }

    #[test]
    fn rejects_empty_comment() {
        assert!(CommentDefinition::parse(" ").is_err());
    }
}
