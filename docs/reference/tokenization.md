# Tokenization Design Notes

Tokenization resolves lexical boundaries but does not determine whether a sequence forms a valid program. The parser stage owns syntactic validity.

This handbook page is part of the ULG documentation set. It is written for language
authors, library users, and maintainers. Keep public behavior separate from
implementation details so future backends can reuse the same language model.

## Lexical versus syntactic validity

- Lexical versus syntactic validity: The parser records this concern as structured data and never evaluates the text as executable code.
- Lexical versus syntactic validity: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Lexical versus syntactic validity: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Lexical versus syntactic validity: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Lexical versus syntactic validity: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Lexical versus syntactic validity: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing lexical versus syntactic validity in a valid language definition.
- Document migration behavior before changing lexical versus syntactic validity.

## Maximal munch

- Maximal munch: The parser records this concern as structured data and never evaluates the text as executable code.
- Maximal munch: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Maximal munch: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Maximal munch: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Maximal munch: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Maximal munch: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing maximal munch in a valid language definition.
- Document migration behavior before changing maximal munch.

## Keyword boundaries

- Keyword boundaries: The parser records this concern as structured data and never evaluates the text as executable code.
- Keyword boundaries: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Keyword boundaries: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Keyword boundaries: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Keyword boundaries: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Keyword boundaries: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing keyword boundaries in a valid language definition.
- Document migration behavior before changing keyword boundaries.

## Identifier policy

- Identifier policy: The parser records this concern as structured data and never evaluates the text as executable code.
- Identifier policy: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Identifier policy: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Identifier policy: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Identifier policy: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Identifier policy: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing identifier policy in a valid language definition.
- Document migration behavior before changing identifier policy.

## Numeric syntax

- Numeric syntax: The parser records this concern as structured data and never evaluates the text as executable code.
- Numeric syntax: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Numeric syntax: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Numeric syntax: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Numeric syntax: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Numeric syntax: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing numeric syntax in a valid language definition.
- Document migration behavior before changing numeric syntax.

## Literal policy

- Literal policy: The parser records this concern as structured data and never evaluates the text as executable code.
- Literal policy: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Literal policy: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Literal policy: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Literal policy: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Literal policy: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing literal policy in a valid language definition.
- Document migration behavior before changing literal policy.

## Comment precedence

- Comment precedence: The parser records this concern as structured data and never evaluates the text as executable code.
- Comment precedence: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Comment precedence: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Comment precedence: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Comment precedence: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Comment precedence: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing comment precedence in a valid language definition.
- Document migration behavior before changing comment precedence.

## Delimiter recognition

- Delimiter recognition: The parser records this concern as structured data and never evaluates the text as executable code.
- Delimiter recognition: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Delimiter recognition: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Delimiter recognition: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Delimiter recognition: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Delimiter recognition: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing delimiter recognition in a valid language definition.
- Document migration behavior before changing delimiter recognition.

## Operator conflict

- Operator conflict: The parser records this concern as structured data and never evaluates the text as executable code.
- Operator conflict: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Operator conflict: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Operator conflict: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Operator conflict: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Operator conflict: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing operator conflict in a valid language definition.
- Document migration behavior before changing operator conflict.

## Whitespace removal

- Whitespace removal: The parser records this concern as structured data and never evaluates the text as executable code.
- Whitespace removal: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Whitespace removal: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Whitespace removal: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Whitespace removal: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Whitespace removal: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing whitespace removal in a valid language definition.
- Document migration behavior before changing whitespace removal.

## Span tracking

- Span tracking: The parser records this concern as structured data and never evaluates the text as executable code.
- Span tracking: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Span tracking: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Span tracking: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Span tracking: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Span tracking: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing span tracking in a valid language definition.
- Document migration behavior before changing span tracking.

## Error recovery

- Error recovery: The parser records this concern as structured data and never evaluates the text as executable code.
- Error recovery: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Error recovery: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Error recovery: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Error recovery: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Error recovery: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing error recovery in a valid language definition.
- Document migration behavior before changing error recovery.

## Incremental scanning

- Incremental scanning: The parser records this concern as structured data and never evaluates the text as executable code.
- Incremental scanning: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Incremental scanning: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Incremental scanning: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Incremental scanning: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Incremental scanning: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing incremental scanning in a valid language definition.
- Document migration behavior before changing incremental scanning.

## Future lexer plugins

- Future lexer plugins: The parser records this concern as structured data and never evaluates the text as executable code.
- Future lexer plugins: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Future lexer plugins: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Future lexer plugins: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Future lexer plugins: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Future lexer plugins: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing future lexer plugins in a valid language definition.
- Document migration behavior before changing future lexer plugins.

## Operational checklist

- Validate every definition before invoking code generation.
- Preserve source locations in every diagnostic path.
- Keep generated output deterministic and safe to overwrite only by explicit request.
- Run the full test suite after changing a shared contract.
