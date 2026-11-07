# Regular Expression Policy

Custom punctuation patterns are compiled as regular expressions. Patterns must not match an empty string and must remain bounded by input-size policy.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## Regex engine choice

- Regex engine choice: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Regex engine choice: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Regex engine choice: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Regex engine choice: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Regex engine choice: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Regex engine choice: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected regex engine choice behavior.
- Record compatibility and migration consequences before changing regex engine choice.

## Anchored matching

- Anchored matching: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Anchored matching: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Anchored matching: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Anchored matching: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Anchored matching: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Anchored matching: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected anchored matching behavior.
- Record compatibility and migration consequences before changing anchored matching.

## Pattern source retention

- Pattern source retention: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Pattern source retention: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Pattern source retention: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Pattern source retention: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Pattern source retention: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Pattern source retention: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected pattern source retention behavior.
- Record compatibility and migration consequences before changing pattern source retention.

## Invalid pattern errors

- Invalid pattern errors: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Invalid pattern errors: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Invalid pattern errors: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Invalid pattern errors: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Invalid pattern errors: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Invalid pattern errors: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected invalid pattern errors behavior.
- Record compatibility and migration consequences before changing invalid pattern errors.

## Empty match rejection

- Empty match rejection: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Empty match rejection: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Empty match rejection: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Empty match rejection: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Empty match rejection: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Empty match rejection: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected empty match rejection behavior.
- Record compatibility and migration consequences before changing empty match rejection.

## Greedy alternatives

- Greedy alternatives: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Greedy alternatives: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Greedy alternatives: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Greedy alternatives: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Greedy alternatives: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Greedy alternatives: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected greedy alternatives behavior.
- Record compatibility and migration consequences before changing greedy alternatives.

## Capture group semantics

- Capture group semantics: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Capture group semantics: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Capture group semantics: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Capture group semantics: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Capture group semantics: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Capture group semantics: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected capture group semantics behavior.
- Record compatibility and migration consequences before changing capture group semantics.

## Unicode behavior

- Unicode behavior: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Unicode behavior: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Unicode behavior: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Unicode behavior: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Unicode behavior: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Unicode behavior: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected unicode behavior behavior.
- Record compatibility and migration consequences before changing unicode behavior.

## Regex compilation cache

- Regex compilation cache: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Regex compilation cache: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Regex compilation cache: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Regex compilation cache: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Regex compilation cache: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Regex compilation cache: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected regex compilation cache behavior.
- Record compatibility and migration consequences before changing regex compilation cache.

## Pattern count limits

- Pattern count limits: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Pattern count limits: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Pattern count limits: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Pattern count limits: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Pattern count limits: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Pattern count limits: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected pattern count limits behavior.
- Record compatibility and migration consequences before changing pattern count limits.

## Source length limits

- Source length limits: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Source length limits: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Source length limits: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Source length limits: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Source length limits: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Source length limits: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected source length limits behavior.
- Record compatibility and migration consequences before changing source length limits.

## Rule precedence

- Rule precedence: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Rule precedence: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Rule precedence: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Rule precedence: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Rule precedence: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Rule precedence: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected rule precedence behavior.
- Record compatibility and migration consequences before changing rule precedence.

## Diagnostic location

- Diagnostic location: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Diagnostic location: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Diagnostic location: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Diagnostic location: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Diagnostic location: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Diagnostic location: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected diagnostic location behavior.
- Record compatibility and migration consequences before changing diagnostic location.

## Future pattern dialects

- Future pattern dialects: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Future pattern dialects: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Future pattern dialects: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Future pattern dialects: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Future pattern dialects: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Future pattern dialects: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected future pattern dialects behavior.
- Record compatibility and migration consequences before changing future pattern dialects.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
