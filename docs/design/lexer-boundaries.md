# Lexer Boundary Reference

Lexical correctness depends on boundary decisions. A keyword, comment marker, or operator must not consume a prefix of a longer token unless its rule allows it.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## Identifier boundary

- Identifier boundary: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Identifier boundary: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Identifier boundary: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Identifier boundary: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Identifier boundary: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Identifier boundary: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected identifier boundary behavior.
- Record compatibility and migration consequences before changing identifier boundary.

## Keyword prefix

- Keyword prefix: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Keyword prefix: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Keyword prefix: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Keyword prefix: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Keyword prefix: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Keyword prefix: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected keyword prefix behavior.
- Record compatibility and migration consequences before changing keyword prefix.

## Operator prefix

- Operator prefix: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Operator prefix: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Operator prefix: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Operator prefix: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Operator prefix: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Operator prefix: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected operator prefix behavior.
- Record compatibility and migration consequences before changing operator prefix.

## Comment marker boundary

- Comment marker boundary: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Comment marker boundary: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Comment marker boundary: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Comment marker boundary: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Comment marker boundary: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Comment marker boundary: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected comment marker boundary behavior.
- Record compatibility and migration consequences before changing comment marker boundary.

## String delimiter

- String delimiter: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- String delimiter: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- String delimiter: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- String delimiter: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- String delimiter: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- String delimiter: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected string delimiter behavior.
- Record compatibility and migration consequences before changing string delimiter.

## Escape sequence

- Escape sequence: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Escape sequence: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Escape sequence: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Escape sequence: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Escape sequence: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Escape sequence: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected escape sequence behavior.
- Record compatibility and migration consequences before changing escape sequence.

## Number decimal point

- Number decimal point: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Number decimal point: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Number decimal point: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Number decimal point: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Number decimal point: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Number decimal point: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected number decimal point behavior.
- Record compatibility and migration consequences before changing number decimal point.

## Exponent marker

- Exponent marker: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Exponent marker: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Exponent marker: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Exponent marker: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Exponent marker: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Exponent marker: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected exponent marker behavior.
- Record compatibility and migration consequences before changing exponent marker.

## Unicode category

- Unicode category: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Unicode category: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Unicode category: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Unicode category: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Unicode category: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Unicode category: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected unicode category behavior.
- Record compatibility and migration consequences before changing unicode category.

## Punctuation regex

- Punctuation regex: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Punctuation regex: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Punctuation regex: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Punctuation regex: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Punctuation regex: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Punctuation regex: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected punctuation regex behavior.
- Record compatibility and migration consequences before changing punctuation regex.

## Empty match

- Empty match: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Empty match: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Empty match: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Empty match: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Empty match: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Empty match: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected empty match behavior.
- Record compatibility and migration consequences before changing empty match.

## Longest match

- Longest match: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Longest match: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Longest match: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Longest match: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Longest match: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Longest match: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected longest match behavior.
- Record compatibility and migration consequences before changing longest match.

## Whitespace transition

- Whitespace transition: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Whitespace transition: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Whitespace transition: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Whitespace transition: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Whitespace transition: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Whitespace transition: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected whitespace transition behavior.
- Record compatibility and migration consequences before changing whitespace transition.

## Unexpected character

- Unexpected character: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Unexpected character: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Unexpected character: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Unexpected character: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Unexpected character: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Unexpected character: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected unexpected character behavior.
- Record compatibility and migration consequences before changing unexpected character.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
