# Release Process

A release should have tested source, stable artifacts, provenance, and compatibility notes. The initial CI workflow is a foundation rather than a complete release system.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## Version selection

- Version selection: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Version selection: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Version selection: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Version selection: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Version selection: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Version selection: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected version selection behavior.
- Record compatibility and migration consequences before changing version selection.

## Changelog entries

- Changelog entries: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Changelog entries: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Changelog entries: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Changelog entries: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Changelog entries: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Changelog entries: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected changelog entries behavior.
- Record compatibility and migration consequences before changing changelog entries.

## Tag naming

- Tag naming: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Tag naming: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Tag naming: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Tag naming: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Tag naming: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Tag naming: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected tag naming behavior.
- Record compatibility and migration consequences before changing tag naming.

## Stable toolchain

- Stable toolchain: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Stable toolchain: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Stable toolchain: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Stable toolchain: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Stable toolchain: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Stable toolchain: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected stable toolchain behavior.
- Record compatibility and migration consequences before changing stable toolchain.

## Cross-platform targets

- Cross-platform targets: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Cross-platform targets: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Cross-platform targets: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Cross-platform targets: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Cross-platform targets: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Cross-platform targets: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected cross-platform targets behavior.
- Record compatibility and migration consequences before changing cross-platform targets.

## Test matrix

- Test matrix: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Test matrix: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Test matrix: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Test matrix: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Test matrix: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Test matrix: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected test matrix behavior.
- Record compatibility and migration consequences before changing test matrix.

## Generated artifacts

- Generated artifacts: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Generated artifacts: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Generated artifacts: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Generated artifacts: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Generated artifacts: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Generated artifacts: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected generated artifacts behavior.
- Record compatibility and migration consequences before changing generated artifacts.

## Checksums

- Checksums: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Checksums: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Checksums: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Checksums: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Checksums: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Checksums: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected checksums behavior.
- Record compatibility and migration consequences before changing checksums.

## Release notes

- Release notes: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Release notes: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Release notes: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Release notes: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Release notes: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Release notes: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected release notes behavior.
- Record compatibility and migration consequences before changing release notes.

## Package publication

- Package publication: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Package publication: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Package publication: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Package publication: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Package publication: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Package publication: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected package publication behavior.
- Record compatibility and migration consequences before changing package publication.

## Rollback process

- Rollback process: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Rollback process: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Rollback process: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Rollback process: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Rollback process: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Rollback process: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected rollback process behavior.
- Record compatibility and migration consequences before changing rollback process.

## Security fixes

- Security fixes: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Security fixes: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Security fixes: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Security fixes: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Security fixes: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Security fixes: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected security fixes behavior.
- Record compatibility and migration consequences before changing security fixes.

## Support policy

- Support policy: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Support policy: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Support policy: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Support policy: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Support policy: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Support policy: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected support policy behavior.
- Record compatibility and migration consequences before changing support policy.

## Post-release verification

- Post-release verification: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Post-release verification: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Post-release verification: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Post-release verification: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Post-release verification: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Post-release verification: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected post-release verification behavior.
- Record compatibility and migration consequences before changing post-release verification.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
