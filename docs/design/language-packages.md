# Language Package Design

A language package distributes a ULG definition, extension mapping, optional generated artifacts, and metadata. Package content remains declarative and verifiable.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## Package identity

- Package identity: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Package identity: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Package identity: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Package identity: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Package identity: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Package identity: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected package identity behavior.
- Record compatibility and migration consequences before changing package identity.

## Definition file

- Definition file: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Definition file: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Definition file: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Definition file: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Definition file: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Definition file: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected definition file behavior.
- Record compatibility and migration consequences before changing definition file.

## File extensions

- File extensions: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- File extensions: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- File extensions: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- File extensions: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- File extensions: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- File extensions: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected file extensions behavior.
- Record compatibility and migration consequences before changing file extensions.

## Package version

- Package version: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Package version: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Package version: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Package version: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Package version: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Package version: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected package version behavior.
- Record compatibility and migration consequences before changing package version.

## Minimum ULG version

- Minimum ULG version: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Minimum ULG version: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Minimum ULG version: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Minimum ULG version: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Minimum ULG version: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Minimum ULG version: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected minimum ulg version behavior.
- Record compatibility and migration consequences before changing minimum ulg version.

## Optional artifacts

- Optional artifacts: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Optional artifacts: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Optional artifacts: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Optional artifacts: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Optional artifacts: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Optional artifacts: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected optional artifacts behavior.
- Record compatibility and migration consequences before changing optional artifacts.

## Integrity hashes

- Integrity hashes: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Integrity hashes: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Integrity hashes: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Integrity hashes: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Integrity hashes: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Integrity hashes: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected integrity hashes behavior.
- Record compatibility and migration consequences before changing integrity hashes.

## Installation layout

- Installation layout: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Installation layout: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Installation layout: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Installation layout: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Installation layout: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Installation layout: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected installation layout behavior.
- Record compatibility and migration consequences before changing installation layout.

## Registry metadata

- Registry metadata: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Registry metadata: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Registry metadata: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Registry metadata: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Registry metadata: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Registry metadata: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected registry metadata behavior.
- Record compatibility and migration consequences before changing registry metadata.

## Offline installation

- Offline installation: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Offline installation: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Offline installation: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Offline installation: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Offline installation: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Offline installation: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected offline installation behavior.
- Record compatibility and migration consequences before changing offline installation.

## Update policy

- Update policy: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Update policy: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Update policy: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Update policy: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Update policy: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Update policy: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected update policy behavior.
- Record compatibility and migration consequences before changing update policy.

## Signature model

- Signature model: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Signature model: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Signature model: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Signature model: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Signature model: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Signature model: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected signature model behavior.
- Record compatibility and migration consequences before changing signature model.

## Uninstall behavior

- Uninstall behavior: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Uninstall behavior: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Uninstall behavior: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Uninstall behavior: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Uninstall behavior: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Uninstall behavior: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected uninstall behavior behavior.
- Record compatibility and migration consequences before changing uninstall behavior.

## Package trust

- Package trust: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Package trust: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Package trust: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Package trust: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Package trust: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Package trust: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected package trust behavior.
- Record compatibility and migration consequences before changing package trust.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
