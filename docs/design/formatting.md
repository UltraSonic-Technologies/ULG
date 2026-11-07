# Formatting and Canonicalization

Formatting should improve readability without changing definition meaning. A future formatter should preserve string contents and keep diffs stable.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## Indentation

- Indentation: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Indentation: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Indentation: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Indentation: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Indentation: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Indentation: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected indentation behavior.
- Record compatibility and migration consequences before changing indentation.

## Section order

- Section order: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Section order: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Section order: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Section order: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Section order: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Section order: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected section order behavior.
- Record compatibility and migration consequences before changing section order.

## List wrapping

- List wrapping: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- List wrapping: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- List wrapping: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- List wrapping: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- List wrapping: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- List wrapping: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected list wrapping behavior.
- Record compatibility and migration consequences before changing list wrapping.

## Trailing commas

- Trailing commas: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Trailing commas: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Trailing commas: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Trailing commas: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Trailing commas: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Trailing commas: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected trailing commas behavior.
- Record compatibility and migration consequences before changing trailing commas.

## String escaping

- String escaping: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- String escaping: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- String escaping: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- String escaping: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- String escaping: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- String escaping: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected string escaping behavior.
- Record compatibility and migration consequences before changing string escaping.

## MATCH alignment

- MATCH alignment: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- MATCH alignment: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- MATCH alignment: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- MATCH alignment: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- MATCH alignment: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- MATCH alignment: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected match alignment behavior.
- Record compatibility and migration consequences before changing match alignment.

## Blank lines

- Blank lines: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Blank lines: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Blank lines: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Blank lines: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Blank lines: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Blank lines: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected blank lines behavior.
- Record compatibility and migration consequences before changing blank lines.

## Line endings

- Line endings: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Line endings: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Line endings: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Line endings: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Line endings: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Line endings: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected line endings behavior.
- Record compatibility and migration consequences before changing line endings.

## Preserving comments

- Preserving comments: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Preserving comments: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Preserving comments: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Preserving comments: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Preserving comments: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Preserving comments: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected preserving comments behavior.
- Record compatibility and migration consequences before changing preserving comments.

## Canonical output

- Canonical output: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Canonical output: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Canonical output: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Canonical output: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Canonical output: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Canonical output: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected canonical output behavior.
- Record compatibility and migration consequences before changing canonical output.

## Diff stability

- Diff stability: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Diff stability: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Diff stability: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Diff stability: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Diff stability: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Diff stability: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected diff stability behavior.
- Record compatibility and migration consequences before changing diff stability.

## Formatter idempotence

- Formatter idempotence: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Formatter idempotence: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Formatter idempotence: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Formatter idempotence: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Formatter idempotence: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Formatter idempotence: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected formatter idempotence behavior.
- Record compatibility and migration consequences before changing formatter idempotence.

## Formatting tests

- Formatting tests: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Formatting tests: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Formatting tests: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Formatting tests: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Formatting tests: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Formatting tests: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected formatting tests behavior.
- Record compatibility and migration consequences before changing formatting tests.

## Source maps

- Source maps: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Source maps: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Source maps: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Source maps: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Source maps: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Source maps: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected source maps behavior.
- Record compatibility and migration consequences before changing source maps.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
