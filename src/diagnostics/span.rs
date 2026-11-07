//! Source locations and byte ranges.
//!
//! A span stores byte offsets because Rust string slicing is byte-based,
//! alongside user-facing one-based line and column coordinates. Columns
//! count Unicode scalar values rather than UTF-8 bytes.

/// A half-open range in a source document.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl SourceSpan {
    /// Creates a span with explicit source coordinates.
    pub const fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self { start, end, line, column }
    }

    /// Creates a zero-width span at a source position.
    pub const fn point(offset: usize, line: usize, column: usize) -> Self {
        Self::new(offset, offset, line, column)
    }

    /// Returns the byte length of the range.
    pub const fn len(self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Returns true when the span has no content.
    pub const fn is_empty(self) -> bool {
        self.start >= self.end
    }

    /// Returns true when a byte offset lies inside this half-open range.
    pub const fn contains(self, offset: usize) -> bool {
        self.start <= offset && offset < self.end
    }

    /// Returns a span that covers both input ranges.
    pub fn cover(self, other: Self) -> Self {
        let (line, column) = if self.start <= other.start {
            (self.line, self.column)
        } else {
            (other.line, other.column)
        };
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line,
            column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_span_is_empty() {
        assert!(SourceSpan::point(4, 2, 3).is_empty());
    }

    #[test]
    fn cover_uses_outer_byte_range() {
        let left = SourceSpan::new(3, 5, 1, 4);
        let right = SourceSpan::new(8, 12, 1, 9);
        assert_eq!(left.cover(right).start, 3);
        assert_eq!(left.cover(right).end, 12);
    }
}
