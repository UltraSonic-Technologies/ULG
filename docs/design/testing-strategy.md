# Testing Strategy

Testing verifies accepted definitions and rejected inputs. Golden files are important when parser changes could silently alter language-package interpretation.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## Unit testing

- Unit testing: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Unit testing: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Unit testing: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Unit testing: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Unit testing: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Unit testing: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected unit testing behavior.
- Record compatibility and migration consequences before changing unit testing.

## Integration testing

- Integration testing: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Integration testing: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Integration testing: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Integration testing: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Integration testing: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Integration testing: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected integration testing behavior.
- Record compatibility and migration consequences before changing integration testing.

## Golden testing

- Golden testing: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Golden testing: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Golden testing: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Golden testing: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Golden testing: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Golden testing: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected golden testing behavior.
- Record compatibility and migration consequences before changing golden testing.

## Fuzz testing

- Fuzz testing: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Fuzz testing: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Fuzz testing: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Fuzz testing: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Fuzz testing: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Fuzz testing: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected fuzz testing behavior.
- Record compatibility and migration consequences before changing fuzz testing.

## Property testing

- Property testing: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Property testing: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Property testing: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Property testing: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Property testing: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Property testing: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected property testing behavior.
- Record compatibility and migration consequences before changing property testing.

## Unicode testing

- Unicode testing: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Unicode testing: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Unicode testing: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Unicode testing: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Unicode testing: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Unicode testing: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected unicode testing behavior.
- Record compatibility and migration consequences before changing unicode testing.

## Malformed source

- Malformed source: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Malformed source: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Malformed source: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Malformed source: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Malformed source: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Malformed source: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected malformed source behavior.
- Record compatibility and migration consequences before changing malformed source.

## Lexer boundaries

- Lexer boundaries: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Lexer boundaries: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Lexer boundaries: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Lexer boundaries: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Lexer boundaries: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Lexer boundaries: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected lexer boundaries behavior.
- Record compatibility and migration consequences before changing lexer boundaries.

## Codegen compilation

- Codegen compilation: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Codegen compilation: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Codegen compilation: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Codegen compilation: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Codegen compilation: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Codegen compilation: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected codegen compilation behavior.
- Record compatibility and migration consequences before changing codegen compilation.

## CLI snapshots

- CLI snapshots: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- CLI snapshots: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- CLI snapshots: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- CLI snapshots: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- CLI snapshots: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- CLI snapshots: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected cli snapshots behavior.
- Record compatibility and migration consequences before changing cli snapshots.

## Cross-platform testing

- Cross-platform testing: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Cross-platform testing: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Cross-platform testing: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Cross-platform testing: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Cross-platform testing: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Cross-platform testing: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected cross-platform testing behavior.
- Record compatibility and migration consequences before changing cross-platform testing.

## Regression ownership

- Regression ownership: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Regression ownership: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Regression ownership: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Regression ownership: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Regression ownership: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Regression ownership: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected regression ownership behavior.
- Record compatibility and migration consequences before changing regression ownership.

## Test fixtures

- Test fixtures: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Test fixtures: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Test fixtures: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Test fixtures: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Test fixtures: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Test fixtures: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected test fixtures behavior.
- Record compatibility and migration consequences before changing test fixtures.

## CI enforcement

- CI enforcement: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- CI enforcement: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- CI enforcement: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- CI enforcement: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- CI enforcement: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- CI enforcement: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected ci enforcement behavior.
- Record compatibility and migration consequences before changing ci enforcement.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
