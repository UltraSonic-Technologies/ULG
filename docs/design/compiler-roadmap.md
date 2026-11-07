# Compiler Roadmap

A language definition provides lexical information. A compiler also needs syntax, semantics, a type system, intermediate representation, and a backend.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## Lexical stage

- Lexical stage: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Lexical stage: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Lexical stage: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Lexical stage: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Lexical stage: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Lexical stage: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected lexical stage behavior.
- Record compatibility and migration consequences before changing lexical stage.

## Grammar stage

- Grammar stage: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Grammar stage: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Grammar stage: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Grammar stage: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Grammar stage: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Grammar stage: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected grammar stage behavior.
- Record compatibility and migration consequences before changing grammar stage.

## AST stage

- AST stage: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- AST stage: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- AST stage: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- AST stage: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- AST stage: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- AST stage: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected ast stage behavior.
- Record compatibility and migration consequences before changing ast stage.

## Name resolution

- Name resolution: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Name resolution: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Name resolution: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Name resolution: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Name resolution: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Name resolution: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected name resolution behavior.
- Record compatibility and migration consequences before changing name resolution.

## Type checking

- Type checking: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Type checking: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Type checking: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Type checking: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Type checking: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Type checking: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected type checking behavior.
- Record compatibility and migration consequences before changing type checking.

## Semantic diagnostics

- Semantic diagnostics: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Semantic diagnostics: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Semantic diagnostics: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Semantic diagnostics: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Semantic diagnostics: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Semantic diagnostics: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected semantic diagnostics behavior.
- Record compatibility and migration consequences before changing semantic diagnostics.

## Intermediate representation

- Intermediate representation: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Intermediate representation: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Intermediate representation: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Intermediate representation: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Intermediate representation: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Intermediate representation: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected intermediate representation behavior.
- Record compatibility and migration consequences before changing intermediate representation.

## Optimization passes

- Optimization passes: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Optimization passes: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Optimization passes: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Optimization passes: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Optimization passes: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Optimization passes: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected optimization passes behavior.
- Record compatibility and migration consequences before changing optimization passes.

## Code generation

- Code generation: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Code generation: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Code generation: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Code generation: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Code generation: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Code generation: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected code generation behavior.
- Record compatibility and migration consequences before changing code generation.

## Runtime ABI

- Runtime ABI: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Runtime ABI: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Runtime ABI: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Runtime ABI: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Runtime ABI: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Runtime ABI: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected runtime abi behavior.
- Record compatibility and migration consequences before changing runtime abi.

## Standard library

- Standard library: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Standard library: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Standard library: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Standard library: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Standard library: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Standard library: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected standard library behavior.
- Record compatibility and migration consequences before changing standard library.

## Native toolchain

- Native toolchain: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Native toolchain: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Native toolchain: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Native toolchain: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Native toolchain: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Native toolchain: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected native toolchain behavior.
- Record compatibility and migration consequences before changing native toolchain.

## Debug information

- Debug information: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Debug information: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Debug information: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Debug information: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Debug information: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Debug information: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected debug information behavior.
- Record compatibility and migration consequences before changing debug information.

## Compiler testing

- Compiler testing: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Compiler testing: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Compiler testing: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Compiler testing: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Compiler testing: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Compiler testing: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected compiler testing behavior.
- Record compatibility and migration consequences before changing compiler testing.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
