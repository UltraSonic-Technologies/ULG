# Generated Rust lexer

This guide describes the Rust source emitted by the ULG Rust backend.

## Purpose

The generated module is a reusable lexical front end for one language
definition. It embeds metadata and lexical declarations from the input
ULG file. It does not need the ULG crate at runtime.

The generated module depends on the Rust regex crate for custom punctuation
patterns. Add regex version 1.11 or a compatible later release to the
consuming project's dependencies.

## Generate an artifact

Run:

    ulg check path/to/language.ulg
    ulg generate rust path/to/language.ulg --out generated

The default module filename is language_definition.rs. Supply
--module my_language to change the module label and output filename.

The generator validates the definition before producing output. It refuses
to overwrite an existing destination. Select a new path when regenerating.

## Public API

The generated module exposes these items:

- LANGUAGE_NAME, LANGUAGE_DESCRIPTION, and LANGUAGE_VERSION.
- KEYWORDS, COMMENTS, OPERATORS, and PUNCTUATION.
- TokenKind, the category assigned to each lexical token.
- Span, a UTF-8 byte range and one-based source coordinate.
- Token, the exact source spelling, category, span, and optional rule.
- LexError, an error message and the location where scanning failed.
- tokenize(source), the main scanner entry point.
- is_keyword(value), a case-sensitive keyword lookup.
- longest_operator(value), a maximal-munch operator lookup.
- format_tokens(tokens), a stable diagnostic table.

## Token categories

The generated scanner can produce keyword, identifier, number, string,
character, comment, operator, and punctuation tokens. Whitespace is not
emitted as a token. Original lexemes are retained exactly.

Identifiers use Unicode alphabetic characters for their first character,
followed by Unicode alphanumeric characters or underscores. An at-sign is
accepted as an identifier prefix for macro-like names. A declared keyword
takes precedence over a general identifier only when the following character
does not continue an identifier.

## Numeric literals

The numeric scanner recognizes decimal integers, decimal fractions,
scientific notation, hexadecimal values with a 0x prefix, binary values
with a 0b prefix, and octal values with a 0o prefix. Underscores are
preserved in numeric lexemes. The scanner reports missing digits after a
base prefix and missing digits in an exponent.

This is a lexical convention, not a numeric type system. Range checking,
suffixes, arbitrary precision, and language-specific numeric semantics are
future extension points.

## Strings and characters

Double quotes create string tokens and single quotes create character tokens.
Escaped characters are consumed as part of the literal. A newline before the
closing quote is reported as an error. The lexer preserves the original
literal spelling rather than interpreting escape values.

The lexer does not yet enforce that a character literal contains exactly one
Unicode scalar value. That validation belongs in a later literal-semantics
phase.

## Comments

A single marker in the ULG COMMENTS list denotes a line comment. A two-marker
declaration denotes a block comment. The scanner selects the longest matching
opening marker, preserves the entire comment lexeme, and reports an
unterminated block comment.

Nested block comments are not implied by the current ULG comment syntax.
A future version may add an explicit nesting policy rather than guessing it.

## Operators

Operators are checked in descending byte length. This implements maximal
munch: when both = and == are declared, the two-character operator is
recognized first. The generated lookup helper uses the same rule.

## Punctuation

The names PARENS, BRACKETS, and BRACES are recognized as conventional pairs
and emitted as individual opening and closing punctuation tokens. Other
punctuation patterns are compiled as anchored regular expressions.

A punctuation expression must consume at least one character. Zero-width
patterns are rejected because they could cause a scanner loop that never
advances. The regex crate is used rather than an unrestricted backtracking
engine.

## Source coordinates

Span offsets are byte offsets into the original UTF-8 string and can be used
for safe source slicing. Line and column coordinates are one-based. Columns
count Unicode scalar values, not terminal display cells; combining marks and
wide glyphs may occupy different visual widths.

## Error handling

Tokenization returns Result<Vec<Token>, LexError>. The first invalid
character or malformed literal stops scanning and returns a location-aware
error. The current API is fail-fast; recovery and multiple lexical diagnostics
are future work.

## Reproducibility

Generated output is deterministic for a fixed definition and generator
version. Keep the source ULG file under version control and regenerate the
module when the definition changes. Review generated diffs like any other
source change.

## Scope boundary

A lexer identifies lexical units. It does not determine whether expressions,
blocks, declarations, or control-flow constructs are grammatically valid.
ULG parser generation requires a separate grammar section and parser backend.
The generated lexer is therefore a complete lexical component, not a complete
programming-language compiler or runtime.
