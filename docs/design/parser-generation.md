# Parser Generation Design

A syntax parser generator requires a grammar model beyond lexical declarations. Future ULG syntax productions should be explicit, typed, and independently versioned.

This design reference is maintained with the implementation. It is intended for
contributors who change the parser, validation model, lexer, code generator, or
public API. User-visible changes require a specification update and test coverage.

## Grammar representation

- Grammar representation: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Grammar representation: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Grammar representation: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Grammar representation: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Grammar representation: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Grammar representation: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected grammar representation behavior.
- Record compatibility and migration consequences before changing grammar representation.

## Terminal symbols

- Terminal symbols: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Terminal symbols: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Terminal symbols: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Terminal symbols: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Terminal symbols: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Terminal symbols: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected terminal symbols behavior.
- Record compatibility and migration consequences before changing terminal symbols.

## Nonterminal symbols

- Nonterminal symbols: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Nonterminal symbols: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Nonterminal symbols: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Nonterminal symbols: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Nonterminal symbols: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Nonterminal symbols: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected nonterminal symbols behavior.
- Record compatibility and migration consequences before changing nonterminal symbols.

## Production alternatives

- Production alternatives: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Production alternatives: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Production alternatives: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Production alternatives: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Production alternatives: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Production alternatives: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected production alternatives behavior.
- Record compatibility and migration consequences before changing production alternatives.

## Precedence declarations

- Precedence declarations: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Precedence declarations: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Precedence declarations: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Precedence declarations: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Precedence declarations: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Precedence declarations: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected precedence declarations behavior.
- Record compatibility and migration consequences before changing precedence declarations.

## Associativity

- Associativity: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Associativity: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Associativity: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Associativity: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Associativity: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Associativity: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected associativity behavior.
- Record compatibility and migration consequences before changing associativity.

## Optional productions

- Optional productions: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Optional productions: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Optional productions: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Optional productions: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Optional productions: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Optional productions: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected optional productions behavior.
- Record compatibility and migration consequences before changing optional productions.

## Repeated productions

- Repeated productions: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Repeated productions: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Repeated productions: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Repeated productions: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Repeated productions: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Repeated productions: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected repeated productions behavior.
- Record compatibility and migration consequences before changing repeated productions.

## Left recursion

- Left recursion: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Left recursion: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Left recursion: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Left recursion: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Left recursion: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Left recursion: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected left recursion behavior.
- Record compatibility and migration consequences before changing left recursion.

## Ambiguity analysis

- Ambiguity analysis: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Ambiguity analysis: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Ambiguity analysis: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Ambiguity analysis: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Ambiguity analysis: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Ambiguity analysis: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected ambiguity analysis behavior.
- Record compatibility and migration consequences before changing ambiguity analysis.

## AST construction

- AST construction: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- AST construction: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- AST construction: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- AST construction: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- AST construction: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- AST construction: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected ast construction behavior.
- Record compatibility and migration consequences before changing ast construction.

## Error recovery

- Error recovery: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Error recovery: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Error recovery: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Error recovery: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Error recovery: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Error recovery: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected error recovery behavior.
- Record compatibility and migration consequences before changing error recovery.

## Generated parser API

- Generated parser API: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Generated parser API: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Generated parser API: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Generated parser API: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Generated parser API: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Generated parser API: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected generated parser api behavior.
- Record compatibility and migration consequences before changing generated parser api.

## Grammar compatibility

- Grammar compatibility: The parser treats this declaration as input data and records its authored spelling without evaluating it.
- Grammar compatibility: Validation checks whether the declaration is complete, consistent, and safe for the next compiler stage.
- Grammar compatibility: The runtime lexer uses only validated rule data and preserves a source span for every emitted token.
- Grammar compatibility: CLI operations share the same library pipeline so local use and continuous integration behave consistently.
- Grammar compatibility: Generated Rust output must be deterministic and must not contain shell commands or executable ULG expressions.
- Grammar compatibility: Acceptance tests should include a valid case, invalid case, boundary case, and at least one Unicode case.
- Add a fixture that demonstrates the expected grammar compatibility behavior.
- Record compatibility and migration consequences before changing grammar compatibility.

## Maintainer review

- Is the behavior specified independently from its current implementation?
- Are malformed inputs rejected without a panic?
- Are generated artifacts reproducible?
- Are tests and examples updated together?

## Related work

See the other design references for neighboring responsibilities. Keep the boundaries between parsing, validation, tokenization, and code generation explicit so consumers can test and embed each stage independently.
