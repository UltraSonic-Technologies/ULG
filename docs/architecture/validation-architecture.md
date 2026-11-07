# Validation Architecture

Validation checks semantic consistency after structural parsing. It aggregates problems so authors can fix several definition mistakes in one iteration.

This design note is part of the ULG engineering handbook. It describes responsibilities
for contributors and consumers of the library. Public behavior should be specified
independently from implementation details so multiple backends can share one model.

## Required metadata

- Required metadata: The parser records this concern as structured data and never evaluates the text as executable code.
- Required metadata: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Required metadata: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Required metadata: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Required metadata: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Required metadata: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating required metadata in a normal language definition.
- Review error behavior and compatibility notes whenever required metadata changes.

## Language names

- Language names: The parser records this concern as structured data and never evaluates the text as executable code.
- Language names: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Language names: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Language names: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Language names: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Language names: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating language names in a normal language definition.
- Review error behavior and compatibility notes whenever language names changes.

## Keyword uniqueness

- Keyword uniqueness: The parser records this concern as structured data and never evaluates the text as executable code.
- Keyword uniqueness: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Keyword uniqueness: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Keyword uniqueness: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Keyword uniqueness: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Keyword uniqueness: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating keyword uniqueness in a normal language definition.
- Review error behavior and compatibility notes whenever keyword uniqueness changes.

## Operator uniqueness

- Operator uniqueness: The parser records this concern as structured data and never evaluates the text as executable code.
- Operator uniqueness: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Operator uniqueness: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Operator uniqueness: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Operator uniqueness: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Operator uniqueness: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating operator uniqueness in a normal language definition.
- Review error behavior and compatibility notes whenever operator uniqueness changes.

## Comment syntax

- Comment syntax: The parser records this concern as structured data and never evaluates the text as executable code.
- Comment syntax: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Comment syntax: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Comment syntax: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Comment syntax: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Comment syntax: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating comment syntax in a normal language definition.
- Review error behavior and compatibility notes whenever comment syntax changes.

## Punctuation names

- Punctuation names: The parser records this concern as structured data and never evaluates the text as executable code.
- Punctuation names: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Punctuation names: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Punctuation names: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Punctuation names: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Punctuation names: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating punctuation names in a normal language definition.
- Review error behavior and compatibility notes whenever punctuation names changes.

## Regex compilation

- Regex compilation: The parser records this concern as structured data and never evaluates the text as executable code.
- Regex compilation: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Regex compilation: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Regex compilation: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Regex compilation: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Regex compilation: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating regex compilation in a normal language definition.
- Review error behavior and compatibility notes whenever regex compilation changes.

## Keyword conflicts

- Keyword conflicts: The parser records this concern as structured data and never evaluates the text as executable code.
- Keyword conflicts: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Keyword conflicts: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Keyword conflicts: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Keyword conflicts: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Keyword conflicts: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating keyword conflicts in a normal language definition.
- Review error behavior and compatibility notes whenever keyword conflicts changes.

## Empty declarations

- Empty declarations: The parser records this concern as structured data and never evaluates the text as executable code.
- Empty declarations: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Empty declarations: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Empty declarations: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Empty declarations: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Empty declarations: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating empty declarations in a normal language definition.
- Review error behavior and compatibility notes whenever empty declarations changes.

## Warnings

- Warnings: The parser records this concern as structured data and never evaluates the text as executable code.
- Warnings: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Warnings: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Warnings: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Warnings: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Warnings: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating warnings in a normal language definition.
- Review error behavior and compatibility notes whenever warnings changes.

## Error aggregation

- Error aggregation: The parser records this concern as structured data and never evaluates the text as executable code.
- Error aggregation: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Error aggregation: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Error aggregation: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Error aggregation: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Error aggregation: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating error aggregation in a normal language definition.
- Review error behavior and compatibility notes whenever error aggregation changes.

## Stable diagnostic codes

- Stable diagnostic codes: The parser records this concern as structured data and never evaluates the text as executable code.
- Stable diagnostic codes: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Stable diagnostic codes: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Stable diagnostic codes: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Stable diagnostic codes: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Stable diagnostic codes: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating stable diagnostic codes in a normal language definition.
- Review error behavior and compatibility notes whenever stable diagnostic codes changes.

## Editor integration

- Editor integration: The parser records this concern as structured data and never evaluates the text as executable code.
- Editor integration: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Editor integration: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Editor integration: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Editor integration: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Editor integration: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating editor integration in a normal language definition.
- Review error behavior and compatibility notes whenever editor integration changes.

## Validation tests

- Validation tests: The parser records this concern as structured data and never evaluates the text as executable code.
- Validation tests: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Validation tests: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Validation tests: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Validation tests: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Validation tests: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating validation tests in a normal language definition.
- Review error behavior and compatibility notes whenever validation tests changes.

## Review checklist

- Does the change have a documented public contract?
- Are edge cases covered by automated tests?
- Does the command-line interface expose useful errors?
- Is generated output reproducible?

## Related implementation

The implementation is organized under src so each stage can be tested independently. See the module-level documentation and integration tests before changing a shared contract.
