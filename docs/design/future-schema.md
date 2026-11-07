# Future ULG Schema Evolution

New sections should extend the language model without redefining existing declarations. Every schema addition needs a clear parser rule and validation contract.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## New section introduction

- New section introduction: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- New section introduction: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- New section introduction: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- New section introduction: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- New section introduction: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- New section introduction: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected new section introduction behavior.
- Record compatibility and migration consequences before changing new section introduction.

## Required versus optional data

- Required versus optional data: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Required versus optional data: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Required versus optional data: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Required versus optional data: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Required versus optional data: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Required versus optional data: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected required versus optional data behavior.
- Record compatibility and migration consequences before changing required versus optional data.

## Unknown section policy

- Unknown section policy: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Unknown section policy: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Unknown section policy: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Unknown section policy: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Unknown section policy: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Unknown section policy: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected unknown section policy behavior.
- Record compatibility and migration consequences before changing unknown section policy.

## Schema compatibility

- Schema compatibility: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Schema compatibility: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Schema compatibility: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Schema compatibility: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Schema compatibility: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Schema compatibility: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected schema compatibility behavior.
- Record compatibility and migration consequences before changing schema compatibility.

## Default values

- Default values: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Default values: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Default values: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Default values: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Default values: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Default values: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected default values behavior.
- Record compatibility and migration consequences before changing default values.

## Duplicate field policy

- Duplicate field policy: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Duplicate field policy: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Duplicate field policy: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Duplicate field policy: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Duplicate field policy: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Duplicate field policy: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected duplicate field policy behavior.
- Record compatibility and migration consequences before changing duplicate field policy.

## Section ordering

- Section ordering: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Section ordering: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Section ordering: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Section ordering: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Section ordering: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Section ordering: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected section ordering behavior.
- Record compatibility and migration consequences before changing section ordering.

## Version negotiation

- Version negotiation: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Version negotiation: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Version negotiation: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Version negotiation: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Version negotiation: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Version negotiation: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected version negotiation behavior.
- Record compatibility and migration consequences before changing version negotiation.

## Migration tools

- Migration tools: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Migration tools: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Migration tools: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Migration tools: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Migration tools: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Migration tools: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected migration tools behavior.
- Record compatibility and migration consequences before changing migration tools.

## Formatter support

- Formatter support: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Formatter support: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Formatter support: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Formatter support: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Formatter support: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Formatter support: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected formatter support behavior.
- Record compatibility and migration consequences before changing formatter support.

## Codegen support

- Codegen support: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Codegen support: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Codegen support: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Codegen support: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Codegen support: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Codegen support: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected codegen support behavior.
- Record compatibility and migration consequences before changing codegen support.

## Editor support

- Editor support: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Editor support: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Editor support: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Editor support: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Editor support: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Editor support: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected editor support behavior.
- Record compatibility and migration consequences before changing editor support.

## Acceptance fixtures

- Acceptance fixtures: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Acceptance fixtures: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Acceptance fixtures: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Acceptance fixtures: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Acceptance fixtures: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Acceptance fixtures: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected acceptance fixtures behavior.
- Record compatibility and migration consequences before changing acceptance fixtures.

## Deprecation schedule

- Deprecation schedule: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Deprecation schedule: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Deprecation schedule: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Deprecation schedule: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Deprecation schedule: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Deprecation schedule: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected deprecation schedule behavior.
- Record compatibility and migration consequences before changing deprecation schedule.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
