# Language Extension Model

Language authors need a way to associate definitions with source extensions and editor identifiers. Extension metadata is a future ULG section, not inferred from language names.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## File suffix

- File suffix: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- File suffix: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- File suffix: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- File suffix: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- File suffix: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- File suffix: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected file suffix behavior.
- Record compatibility and migration consequences before changing file suffix.

## Compound suffix

- Compound suffix: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Compound suffix: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Compound suffix: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Compound suffix: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Compound suffix: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Compound suffix: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected compound suffix behavior.
- Record compatibility and migration consequences before changing compound suffix.

## Case handling

- Case handling: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Case handling: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Case handling: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Case handling: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Case handling: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Case handling: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected case handling behavior.
- Record compatibility and migration consequences before changing case handling.

## Alias mapping

- Alias mapping: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Alias mapping: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Alias mapping: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Alias mapping: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Alias mapping: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Alias mapping: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected alias mapping behavior.
- Record compatibility and migration consequences before changing alias mapping.

## Conflict detection

- Conflict detection: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Conflict detection: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Conflict detection: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Conflict detection: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Conflict detection: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Conflict detection: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected conflict detection behavior.
- Record compatibility and migration consequences before changing conflict detection.

## Editor identifier

- Editor identifier: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Editor identifier: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Editor identifier: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Editor identifier: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Editor identifier: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Editor identifier: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected editor identifier behavior.
- Record compatibility and migration consequences before changing editor identifier.

## Build tool identifier

- Build tool identifier: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Build tool identifier: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Build tool identifier: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Build tool identifier: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Build tool identifier: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Build tool identifier: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected build tool identifier behavior.
- Record compatibility and migration consequences before changing build tool identifier.

## Language server identifier

- Language server identifier: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Language server identifier: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Language server identifier: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Language server identifier: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Language server identifier: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Language server identifier: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected language server identifier behavior.
- Record compatibility and migration consequences before changing language server identifier.

## Package registry

- Package registry: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Package registry: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Package registry: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Package registry: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Package registry: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Package registry: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected package registry behavior.
- Record compatibility and migration consequences before changing package registry.

## Workspace overrides

- Workspace overrides: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Workspace overrides: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Workspace overrides: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Workspace overrides: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Workspace overrides: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Workspace overrides: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected workspace overrides behavior.
- Record compatibility and migration consequences before changing workspace overrides.

## Per-project settings

- Per-project settings: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Per-project settings: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Per-project settings: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Per-project settings: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Per-project settings: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Per-project settings: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected per-project settings behavior.
- Record compatibility and migration consequences before changing per-project settings.

## Fallback behavior

- Fallback behavior: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Fallback behavior: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Fallback behavior: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Fallback behavior: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Fallback behavior: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Fallback behavior: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected fallback behavior behavior.
- Record compatibility and migration consequences before changing fallback behavior.

## Extension diagnostics

- Extension diagnostics: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Extension diagnostics: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Extension diagnostics: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Extension diagnostics: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Extension diagnostics: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Extension diagnostics: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected extension diagnostics behavior.
- Record compatibility and migration consequences before changing extension diagnostics.

## Schema evolution

- Schema evolution: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Schema evolution: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Schema evolution: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Schema evolution: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Schema evolution: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Schema evolution: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected schema evolution behavior.
- Record compatibility and migration consequences before changing schema evolution.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
