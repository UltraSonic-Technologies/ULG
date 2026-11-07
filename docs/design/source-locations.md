# Source Location Model

Every emitted token and parser error should point to the original UTF-8 text. Byte positions are used for slicing; line and column positions are for people.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## UTF-8 byte offsets

- UTF-8 byte offsets: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- UTF-8 byte offsets: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- UTF-8 byte offsets: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- UTF-8 byte offsets: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- UTF-8 byte offsets: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- UTF-8 byte offsets: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected utf-8 byte offsets behavior.
- Record compatibility and migration consequences before changing utf-8 byte offsets.

## Unicode scalar columns

- Unicode scalar columns: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Unicode scalar columns: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Unicode scalar columns: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Unicode scalar columns: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Unicode scalar columns: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Unicode scalar columns: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected unicode scalar columns behavior.
- Record compatibility and migration consequences before changing unicode scalar columns.

## One-based line numbers

- One-based line numbers: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- One-based line numbers: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- One-based line numbers: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- One-based line numbers: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- One-based line numbers: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- One-based line numbers: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected one-based line numbers behavior.
- Record compatibility and migration consequences before changing one-based line numbers.

## One-based columns

- One-based columns: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- One-based columns: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- One-based columns: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- One-based columns: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- One-based columns: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- One-based columns: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected one-based columns behavior.
- Record compatibility and migration consequences before changing one-based columns.

## Newline normalization

- Newline normalization: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Newline normalization: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Newline normalization: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Newline normalization: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Newline normalization: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Newline normalization: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected newline normalization behavior.
- Record compatibility and migration consequences before changing newline normalization.

## Carriage return handling

- Carriage return handling: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Carriage return handling: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Carriage return handling: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Carriage return handling: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Carriage return handling: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Carriage return handling: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected carriage return handling behavior.
- Record compatibility and migration consequences before changing carriage return handling.

## Tabs and display width

- Tabs and display width: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Tabs and display width: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Tabs and display width: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Tabs and display width: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Tabs and display width: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Tabs and display width: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected tabs and display width behavior.
- Record compatibility and migration consequences before changing tabs and display width.

## Span start

- Span start: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Span start: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Span start: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Span start: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Span start: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Span start: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected span start behavior.
- Record compatibility and migration consequences before changing span start.

## Span end

- Span end: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Span end: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Span end: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Span end: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Span end: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Span end: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected span end behavior.
- Record compatibility and migration consequences before changing span end.

## Half-open ranges

- Half-open ranges: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Half-open ranges: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Half-open ranges: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Half-open ranges: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Half-open ranges: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Half-open ranges: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected half-open ranges behavior.
- Record compatibility and migration consequences before changing half-open ranges.

## Multi-line tokens

- Multi-line tokens: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Multi-line tokens: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Multi-line tokens: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Multi-line tokens: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Multi-line tokens: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Multi-line tokens: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected multi-line tokens behavior.
- Record compatibility and migration consequences before changing multi-line tokens.

## Error position

- Error position: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Error position: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Error position: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Error position: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Error position: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Error position: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected error position behavior.
- Record compatibility and migration consequences before changing error position.

## Editor coordinate conversion

- Editor coordinate conversion: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Editor coordinate conversion: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Editor coordinate conversion: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Editor coordinate conversion: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Editor coordinate conversion: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Editor coordinate conversion: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected editor coordinate conversion behavior.
- Record compatibility and migration consequences before changing editor coordinate conversion.

## Source-map extension

- Source-map extension: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Source-map extension: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Source-map extension: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Source-map extension: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Source-map extension: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Source-map extension: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected source-map extension behavior.
- Record compatibility and migration consequences before changing source-map extension.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
