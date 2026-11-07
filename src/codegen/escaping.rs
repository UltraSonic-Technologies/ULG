//! Literal escaping helpers for generated Rust source.
//!
//! Rust debug formatting for string slices follows Rust's quoted-string
//! syntax and escapes control characters. This module keeps the policy in
//! one place so each backend section emits identical representations.

/// Converts a string into a valid Rust string literal.
pub fn rust_string(value: &str) -> String {
    format!("{value:?}")
}

/// Converts a string slice into a Rust array expression.
pub fn rust_string_array(values: &[String]) -> String {
    let entries = values
        .iter()
        .map(|value| format!("    {},", rust_string(value)))
        .collect::<Vec<_>>()
        .join("\n");
    format!("[\n{entries}\n]")
}

/// Escapes a string for use in a Rust documentation comment.
pub fn rust_doc_text(value: &str) -> String {
    value
        .lines()
        .map(|line| format!("/// {}", line.trim_end()))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_quotes_and_newlines() {
        let literal = rust_string("a\"b\nc");
        assert!(literal.starts_with('"'));
        assert!(literal.contains("\\n"));
    }

    #[test]
    fn empty_array_is_valid_rust() {
        assert_eq!(rust_string_array(&[]), "[\n\n]");
    }
}
