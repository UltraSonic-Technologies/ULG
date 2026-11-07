# Lexer Architecture

The lexer consumes a validated definition and source text. Recognition order prevents comments and quoted literals from being reinterpreted as operators or identifiers.

This design note is part of the ULG engineering handbook. It describes responsibilities
for contributors and consumers of the library. Public behavior should be specified
independently from implementation details so multiple backends can share one model.

## Scanner state

- Scanner state: The parser records this concern as structured data and never evaluates the text as executable code.
- Scanner state: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Scanner state: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Scanner state: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Scanner state: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Scanner state: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating scanner state in a normal language definition.
- Review error behavior and compatibility notes whenever scanner state changes.

## Whitespace skipping

- Whitespace skipping: The parser records this concern as structured data and never evaluates the text as executable code.
- Whitespace skipping: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Whitespace skipping: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Whitespace skipping: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Whitespace skipping: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Whitespace skipping: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating whitespace skipping in a normal language definition.
- Review error behavior and compatibility notes whenever whitespace skipping changes.

## Comment precedence

- Comment precedence: The parser records this concern as structured data and never evaluates the text as executable code.
- Comment precedence: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Comment precedence: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Comment precedence: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Comment precedence: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Comment precedence: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating comment precedence in a normal language definition.
- Review error behavior and compatibility notes whenever comment precedence changes.

## Quoted literals

- Quoted literals: The parser records this concern as structured data and never evaluates the text as executable code.
- Quoted literals: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Quoted literals: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Quoted literals: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Quoted literals: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Quoted literals: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating quoted literals in a normal language definition.
- Review error behavior and compatibility notes whenever quoted literals changes.

## Number recognition

- Number recognition: The parser records this concern as structured data and never evaluates the text as executable code.
- Number recognition: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Number recognition: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Number recognition: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Number recognition: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Number recognition: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating number recognition in a normal language definition.
- Review error behavior and compatibility notes whenever number recognition changes.

## Keyword boundaries

- Keyword boundaries: The parser records this concern as structured data and never evaluates the text as executable code.
- Keyword boundaries: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Keyword boundaries: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Keyword boundaries: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Keyword boundaries: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Keyword boundaries: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating keyword boundaries in a normal language definition.
- Review error behavior and compatibility notes whenever keyword boundaries changes.

## Identifier recognition

- Identifier recognition: The parser records this concern as structured data and never evaluates the text as executable code.
- Identifier recognition: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Identifier recognition: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Identifier recognition: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Identifier recognition: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Identifier recognition: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating identifier recognition in a normal language definition.
- Review error behavior and compatibility notes whenever identifier recognition changes.

## Operator precedence

- Operator precedence: The parser records this concern as structured data and never evaluates the text as executable code.
- Operator precedence: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Operator precedence: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Operator precedence: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Operator precedence: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Operator precedence: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating operator precedence in a normal language definition.
- Review error behavior and compatibility notes whenever operator precedence changes.

## Punctuation matching

- Punctuation matching: The parser records this concern as structured data and never evaluates the text as executable code.
- Punctuation matching: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Punctuation matching: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Punctuation matching: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Punctuation matching: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Punctuation matching: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating punctuation matching in a normal language definition.
- Review error behavior and compatibility notes whenever punctuation matching changes.

## Maximal munch

- Maximal munch: The parser records this concern as structured data and never evaluates the text as executable code.
- Maximal munch: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Maximal munch: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Maximal munch: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Maximal munch: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Maximal munch: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating maximal munch in a normal language definition.
- Review error behavior and compatibility notes whenever maximal munch changes.

## Source spans

- Source spans: The parser records this concern as structured data and never evaluates the text as executable code.
- Source spans: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Source spans: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Source spans: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Source spans: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Source spans: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating source spans in a normal language definition.
- Review error behavior and compatibility notes whenever source spans changes.

## Lexical errors

- Lexical errors: The parser records this concern as structured data and never evaluates the text as executable code.
- Lexical errors: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Lexical errors: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Lexical errors: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Lexical errors: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Lexical errors: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating lexical errors in a normal language definition.
- Review error behavior and compatibility notes whenever lexical errors changes.

## Unicode behavior

- Unicode behavior: The parser records this concern as structured data and never evaluates the text as executable code.
- Unicode behavior: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unicode behavior: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unicode behavior: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unicode behavior: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unicode behavior: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating unicode behavior in a normal language definition.
- Review error behavior and compatibility notes whenever unicode behavior changes.

## Runtime limits

- Runtime limits: The parser records this concern as structured data and never evaluates the text as executable code.
- Runtime limits: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Runtime limits: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Runtime limits: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Runtime limits: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Runtime limits: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating runtime limits in a normal language definition.
- Review error behavior and compatibility notes whenever runtime limits changes.

## Review checklist

- Does the change have a documented public contract?
- Are edge cases covered by automated tests?
- Does the command-line interface expose useful errors?
- Is generated output reproducible?

## Related implementation

The implementation is organized under src so each stage can be tested independently. See the module-level documentation and integration tests before changing a shared contract.
