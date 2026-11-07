# Diagnostic Presentation

Diagnostics are data first and terminal text second. Stable codes and source spans allow IDEs to render squiggles and quick explanations without parsing human wording.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## Stable identifiers

- Stable identifiers: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Stable identifiers: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Stable identifiers: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Stable identifiers: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Stable identifiers: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Stable identifiers: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected stable identifiers behavior.
- Record compatibility and migration consequences before changing stable identifiers.

## Severity level

- Severity level: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Severity level: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Severity level: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Severity level: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Severity level: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Severity level: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected severity level behavior.
- Record compatibility and migration consequences before changing severity level.

## Short message

- Short message: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Short message: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Short message: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Short message: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Short message: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Short message: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected short message behavior.
- Record compatibility and migration consequences before changing short message.

## Detailed note

- Detailed note: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Detailed note: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Detailed note: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Detailed note: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Detailed note: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Detailed note: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected detailed note behavior.
- Record compatibility and migration consequences before changing detailed note.

## Source span

- Source span: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Source span: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Source span: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Source span: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Source span: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Source span: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected source span behavior.
- Record compatibility and migration consequences before changing source span.

## Primary label

- Primary label: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Primary label: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Primary label: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Primary label: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Primary label: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Primary label: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected primary label behavior.
- Record compatibility and migration consequences before changing primary label.

## Related location

- Related location: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Related location: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Related location: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Related location: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Related location: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Related location: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected related location behavior.
- Record compatibility and migration consequences before changing related location.

## Terminal color

- Terminal color: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Terminal color: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Terminal color: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Terminal color: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Terminal color: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Terminal color: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected terminal color behavior.
- Record compatibility and migration consequences before changing terminal color.

## JSON output

- JSON output: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- JSON output: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- JSON output: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- JSON output: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- JSON output: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- JSON output: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected json output behavior.
- Record compatibility and migration consequences before changing json output.

## Editor protocol conversion

- Editor protocol conversion: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Editor protocol conversion: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Editor protocol conversion: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Editor protocol conversion: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Editor protocol conversion: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Editor protocol conversion: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected editor protocol conversion behavior.
- Record compatibility and migration consequences before changing editor protocol conversion.

## Diagnostic grouping

- Diagnostic grouping: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Diagnostic grouping: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Diagnostic grouping: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Diagnostic grouping: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Diagnostic grouping: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Diagnostic grouping: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected diagnostic grouping behavior.
- Record compatibility and migration consequences before changing diagnostic grouping.

## Suppression policy

- Suppression policy: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Suppression policy: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Suppression policy: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Suppression policy: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Suppression policy: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Suppression policy: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected suppression policy behavior.
- Record compatibility and migration consequences before changing suppression policy.

## Localization

- Localization: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Localization: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Localization: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Localization: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Localization: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Localization: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected localization behavior.
- Record compatibility and migration consequences before changing localization.

## Message compatibility

- Message compatibility: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Message compatibility: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Message compatibility: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Message compatibility: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Message compatibility: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Message compatibility: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected message compatibility behavior.
- Record compatibility and migration consequences before changing message compatibility.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
