# Naming and Identifiers

Names affect diagnostics, generated modules, editor display, and package lookup. Preserve author spelling while applying stricter constraints where needed.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## Language display name

- Language display name: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Language display name: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Language display name: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Language display name: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Language display name: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Language display name: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected language display name behavior.
- Record compatibility and migration consequences before changing language display name.

## Package identifier

- Package identifier: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Package identifier: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Package identifier: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Package identifier: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Package identifier: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Package identifier: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected package identifier behavior.
- Record compatibility and migration consequences before changing package identifier.

## Generated module name

- Generated module name: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Generated module name: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Generated module name: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Generated module name: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Generated module name: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Generated module name: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected generated module name behavior.
- Record compatibility and migration consequences before changing generated module name.

## Keyword spelling

- Keyword spelling: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Keyword spelling: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Keyword spelling: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Keyword spelling: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Keyword spelling: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Keyword spelling: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected keyword spelling behavior.
- Record compatibility and migration consequences before changing keyword spelling.

## Punctuation rule names

- Punctuation rule names: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Punctuation rule names: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Punctuation rule names: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Punctuation rule names: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Punctuation rule names: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Punctuation rule names: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected punctuation rule names behavior.
- Record compatibility and migration consequences before changing punctuation rule names.

## Case sensitivity

- Case sensitivity: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Case sensitivity: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Case sensitivity: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Case sensitivity: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Case sensitivity: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Case sensitivity: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected case sensitivity behavior.
- Record compatibility and migration consequences before changing case sensitivity.

## Unicode policy

- Unicode policy: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Unicode policy: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Unicode policy: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Unicode policy: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Unicode policy: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Unicode policy: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected unicode policy behavior.
- Record compatibility and migration consequences before changing unicode policy.

## Reserved names

- Reserved names: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Reserved names: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Reserved names: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Reserved names: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Reserved names: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Reserved names: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected reserved names behavior.
- Record compatibility and migration consequences before changing reserved names.

## File extension aliases

- File extension aliases: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- File extension aliases: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- File extension aliases: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- File extension aliases: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- File extension aliases: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- File extension aliases: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected file extension aliases behavior.
- Record compatibility and migration consequences before changing file extension aliases.

## Collision handling

- Collision handling: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Collision handling: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Collision handling: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Collision handling: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Collision handling: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Collision handling: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected collision handling behavior.
- Record compatibility and migration consequences before changing collision handling.

## Stable naming

- Stable naming: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Stable naming: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Stable naming: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Stable naming: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Stable naming: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Stable naming: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected stable naming behavior.
- Record compatibility and migration consequences before changing stable naming.

## Display normalization

- Display normalization: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Display normalization: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Display normalization: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Display normalization: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Display normalization: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Display normalization: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected display normalization behavior.
- Record compatibility and migration consequences before changing display normalization.

## Error messages

- Error messages: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Error messages: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Error messages: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Error messages: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Error messages: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Error messages: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected error messages behavior.
- Record compatibility and migration consequences before changing error messages.

## Migration policy

- Migration policy: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Migration policy: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Migration policy: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Migration policy: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Migration policy: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Migration policy: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected migration policy behavior.
- Record compatibility and migration consequences before changing migration policy.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
