# Generated Artifact Policy

Generated files should be deterministic, traceable to their source definition, and safe to update. Hand edits should not be lost without an explicit overwrite decision.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## Output naming

- Output naming: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Output naming: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Output naming: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Output naming: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Output naming: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Output naming: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected output naming behavior.
- Record compatibility and migration consequences before changing output naming.

## Output directory

- Output directory: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Output directory: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Output directory: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Output directory: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Output directory: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Output directory: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected output directory behavior.
- Record compatibility and migration consequences before changing output directory.

## Module identifiers

- Module identifiers: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Module identifiers: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Module identifiers: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Module identifiers: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Module identifiers: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Module identifiers: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected module identifiers behavior.
- Record compatibility and migration consequences before changing module identifiers.

## Stable ordering

- Stable ordering: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Stable ordering: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Stable ordering: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Stable ordering: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Stable ordering: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Stable ordering: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected stable ordering behavior.
- Record compatibility and migration consequences before changing stable ordering.

## String escaping

- String escaping: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- String escaping: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- String escaping: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- String escaping: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- String escaping: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- String escaping: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected string escaping behavior.
- Record compatibility and migration consequences before changing string escaping.

## Generated header

- Generated header: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Generated header: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Generated header: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Generated header: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Generated header: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Generated header: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected generated header behavior.
- Record compatibility and migration consequences before changing generated header.

## Source checksum

- Source checksum: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Source checksum: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Source checksum: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Source checksum: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Source checksum: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Source checksum: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected source checksum behavior.
- Record compatibility and migration consequences before changing source checksum.

## Overwrite policy

- Overwrite policy: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Overwrite policy: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Overwrite policy: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Overwrite policy: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Overwrite policy: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Overwrite policy: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected overwrite policy behavior.
- Record compatibility and migration consequences before changing overwrite policy.

## Atomic write strategy

- Atomic write strategy: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Atomic write strategy: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Atomic write strategy: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Atomic write strategy: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Atomic write strategy: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Atomic write strategy: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected atomic write strategy behavior.
- Record compatibility and migration consequences before changing atomic write strategy.

## Build integration

- Build integration: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Build integration: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Build integration: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Build integration: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Build integration: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Build integration: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected build integration behavior.
- Record compatibility and migration consequences before changing build integration.

## Checked-in output

- Checked-in output: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Checked-in output: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Checked-in output: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Checked-in output: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Checked-in output: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Checked-in output: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected checked-in output behavior.
- Record compatibility and migration consequences before changing checked-in output.

## Clean operation

- Clean operation: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Clean operation: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Clean operation: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Clean operation: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Clean operation: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Clean operation: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected clean operation behavior.
- Record compatibility and migration consequences before changing clean operation.

## Reproducibility

- Reproducibility: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Reproducibility: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Reproducibility: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Reproducibility: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Reproducibility: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Reproducibility: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected reproducibility behavior.
- Record compatibility and migration consequences before changing reproducibility.

## Generated API stability

- Generated API stability: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Generated API stability: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Generated API stability: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Generated API stability: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Generated API stability: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Generated API stability: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected generated api stability behavior.
- Record compatibility and migration consequences before changing generated api stability.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
