# Performance and Resource Limits

The reference implementation favors deterministic scans and predictable regular-expression behavior. Embedders should still impose source and declaration size limits.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## Definition byte limit

- Definition byte limit: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Definition byte limit: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Definition byte limit: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Definition byte limit: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Definition byte limit: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Definition byte limit: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected definition byte limit behavior.
- Record compatibility and migration consequences before changing definition byte limit.

## Source byte limit

- Source byte limit: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Source byte limit: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Source byte limit: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Source byte limit: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Source byte limit: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Source byte limit: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected source byte limit behavior.
- Record compatibility and migration consequences before changing source byte limit.

## Token count limit

- Token count limit: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Token count limit: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Token count limit: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Token count limit: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Token count limit: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Token count limit: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected token count limit behavior.
- Record compatibility and migration consequences before changing token count limit.

## Keyword lookup table

- Keyword lookup table: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Keyword lookup table: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Keyword lookup table: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Keyword lookup table: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Keyword lookup table: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Keyword lookup table: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected keyword lookup table behavior.
- Record compatibility and migration consequences before changing keyword lookup table.

## Operator sorting

- Operator sorting: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Operator sorting: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Operator sorting: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Operator sorting: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Operator sorting: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Operator sorting: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected operator sorting behavior.
- Record compatibility and migration consequences before changing operator sorting.

## Regex compilation cost

- Regex compilation cost: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Regex compilation cost: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Regex compilation cost: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Regex compilation cost: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Regex compilation cost: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Regex compilation cost: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected regex compilation cost behavior.
- Record compatibility and migration consequences before changing regex compilation cost.

## Repeated scans

- Repeated scans: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Repeated scans: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Repeated scans: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Repeated scans: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Repeated scans: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Repeated scans: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected repeated scans behavior.
- Record compatibility and migration consequences before changing repeated scans.

## Memory allocation

- Memory allocation: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Memory allocation: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Memory allocation: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Memory allocation: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Memory allocation: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Memory allocation: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected memory allocation behavior.
- Record compatibility and migration consequences before changing memory allocation.

## Token ownership

- Token ownership: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Token ownership: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Token ownership: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Token ownership: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Token ownership: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Token ownership: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected token ownership behavior.
- Record compatibility and migration consequences before changing token ownership.

## Streaming alternatives

- Streaming alternatives: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Streaming alternatives: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Streaming alternatives: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Streaming alternatives: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Streaming alternatives: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Streaming alternatives: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected streaming alternatives behavior.
- Record compatibility and migration consequences before changing streaming alternatives.

## Incremental editor mode

- Incremental editor mode: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Incremental editor mode: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Incremental editor mode: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Incremental editor mode: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Incremental editor mode: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Incremental editor mode: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected incremental editor mode behavior.
- Record compatibility and migration consequences before changing incremental editor mode.

## Cache invalidation

- Cache invalidation: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Cache invalidation: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Cache invalidation: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Cache invalidation: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Cache invalidation: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Cache invalidation: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected cache invalidation behavior.
- Record compatibility and migration consequences before changing cache invalidation.

## Benchmark fixtures

- Benchmark fixtures: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Benchmark fixtures: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Benchmark fixtures: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Benchmark fixtures: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Benchmark fixtures: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Benchmark fixtures: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected benchmark fixtures behavior.
- Record compatibility and migration consequences before changing benchmark fixtures.

## Performance regression

- Performance regression: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Performance regression: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Performance regression: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Performance regression: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Performance regression: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Performance regression: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected performance regression behavior.
- Record compatibility and migration consequences before changing performance regression.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
