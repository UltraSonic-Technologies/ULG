# Token Kind Reference

Token kinds categorize lexical units without imposing a semantic type system. The lexeme preserves source spelling for later parser and editor stages.

This handbook page is part of the ULG documentation set. It is written for language
authors, library users, and maintainers. Keep public behavior separate from
implementation details so future backends can reuse the same language model.

## Keyword token

- Keyword token: The parser records this concern as structured data and never evaluates the text as executable code.
- Keyword token: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Keyword token: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Keyword token: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Keyword token: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Keyword token: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing keyword token in a valid language definition.
- Document migration behavior before changing keyword token.

## Identifier token

- Identifier token: The parser records this concern as structured data and never evaluates the text as executable code.
- Identifier token: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Identifier token: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Identifier token: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Identifier token: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Identifier token: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing identifier token in a valid language definition.
- Document migration behavior before changing identifier token.

## Number token

- Number token: The parser records this concern as structured data and never evaluates the text as executable code.
- Number token: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Number token: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Number token: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Number token: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Number token: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing number token in a valid language definition.
- Document migration behavior before changing number token.

## String token

- String token: The parser records this concern as structured data and never evaluates the text as executable code.
- String token: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- String token: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- String token: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- String token: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- String token: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing string token in a valid language definition.
- Document migration behavior before changing string token.

## Character token

- Character token: The parser records this concern as structured data and never evaluates the text as executable code.
- Character token: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Character token: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Character token: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Character token: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Character token: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing character token in a valid language definition.
- Document migration behavior before changing character token.

## Comment token

- Comment token: The parser records this concern as structured data and never evaluates the text as executable code.
- Comment token: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Comment token: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Comment token: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Comment token: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Comment token: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing comment token in a valid language definition.
- Document migration behavior before changing comment token.

## Operator token

- Operator token: The parser records this concern as structured data and never evaluates the text as executable code.
- Operator token: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Operator token: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Operator token: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Operator token: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Operator token: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing operator token in a valid language definition.
- Document migration behavior before changing operator token.

## Punctuation token

- Punctuation token: The parser records this concern as structured data and never evaluates the text as executable code.
- Punctuation token: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Punctuation token: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Punctuation token: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Punctuation token: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Punctuation token: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing punctuation token in a valid language definition.
- Document migration behavior before changing punctuation token.

## Token span

- Token span: The parser records this concern as structured data and never evaluates the text as executable code.
- Token span: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Token span: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Token span: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Token span: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Token span: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing token span in a valid language definition.
- Document migration behavior before changing token span.

## Named punctuation rule

- Named punctuation rule: The parser records this concern as structured data and never evaluates the text as executable code.
- Named punctuation rule: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Named punctuation rule: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Named punctuation rule: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Named punctuation rule: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Named punctuation rule: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing named punctuation rule in a valid language definition.
- Document migration behavior before changing named punctuation rule.

## Lexeme preservation

- Lexeme preservation: The parser records this concern as structured data and never evaluates the text as executable code.
- Lexeme preservation: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Lexeme preservation: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Lexeme preservation: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Lexeme preservation: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Lexeme preservation: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing lexeme preservation in a valid language definition.
- Document migration behavior before changing lexeme preservation.

## Token ordering

- Token ordering: The parser records this concern as structured data and never evaluates the text as executable code.
- Token ordering: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Token ordering: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Token ordering: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Token ordering: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Token ordering: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing token ordering in a valid language definition.
- Document migration behavior before changing token ordering.

## EOF handling

- EOF handling: The parser records this concern as structured data and never evaluates the text as executable code.
- EOF handling: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- EOF handling: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- EOF handling: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- EOF handling: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- EOF handling: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing eof handling in a valid language definition.
- Document migration behavior before changing eof handling.

## Future token categories

- Future token categories: The parser records this concern as structured data and never evaluates the text as executable code.
- Future token categories: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Future token categories: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Future token categories: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Future token categories: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Future token categories: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing future token categories in a valid language definition.
- Document migration behavior before changing future token categories.

## Operational checklist

- Validate every definition before invoking code generation.
- Preserve source locations in every diagnostic path.
- Keep generated output deterministic and safe to overwrite only by explicit request.
- Run the full test suite after changing a shared contract.
