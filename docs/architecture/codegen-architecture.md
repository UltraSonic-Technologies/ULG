# Code Generation Architecture

The initial Rust backend emits deterministic lexical constants and pure lookup helpers. Future backends can share the same validated model.

This design note is part of the ULG engineering handbook. It describes responsibilities
for contributors and consumers of the library. Public behavior should be specified
independently from implementation details so multiple backends can share one model.

## Backend contract

- Backend contract: The parser records this concern as structured data and never evaluates the text as executable code.
- Backend contract: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Backend contract: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Backend contract: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Backend contract: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Backend contract: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating backend contract in a normal language definition.
- Review error behavior and compatibility notes whenever backend contract changes.

## Rust module output

- Rust module output: The parser records this concern as structured data and never evaluates the text as executable code.
- Rust module output: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Rust module output: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Rust module output: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Rust module output: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Rust module output: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating rust module output in a normal language definition.
- Review error behavior and compatibility notes whenever rust module output changes.

## String literal escaping

- String literal escaping: The parser records this concern as structured data and never evaluates the text as executable code.
- String literal escaping: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- String literal escaping: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- String literal escaping: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- String literal escaping: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- String literal escaping: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating string literal escaping in a normal language definition.
- Review error behavior and compatibility notes whenever string literal escaping changes.

## Keyword arrays

- Keyword arrays: The parser records this concern as structured data and never evaluates the text as executable code.
- Keyword arrays: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Keyword arrays: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Keyword arrays: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Keyword arrays: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Keyword arrays: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating keyword arrays in a normal language definition.
- Review error behavior and compatibility notes whenever keyword arrays changes.

## Operator arrays

- Operator arrays: The parser records this concern as structured data and never evaluates the text as executable code.
- Operator arrays: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Operator arrays: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Operator arrays: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Operator arrays: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Operator arrays: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating operator arrays in a normal language definition.
- Review error behavior and compatibility notes whenever operator arrays changes.

## Comment arrays

- Comment arrays: The parser records this concern as structured data and never evaluates the text as executable code.
- Comment arrays: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Comment arrays: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Comment arrays: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Comment arrays: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Comment arrays: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating comment arrays in a normal language definition.
- Review error behavior and compatibility notes whenever comment arrays changes.

## Punctuation arrays

- Punctuation arrays: The parser records this concern as structured data and never evaluates the text as executable code.
- Punctuation arrays: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Punctuation arrays: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Punctuation arrays: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Punctuation arrays: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Punctuation arrays: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating punctuation arrays in a normal language definition.
- Review error behavior and compatibility notes whenever punctuation arrays changes.

## Pure helper functions

- Pure helper functions: The parser records this concern as structured data and never evaluates the text as executable code.
- Pure helper functions: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Pure helper functions: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Pure helper functions: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Pure helper functions: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Pure helper functions: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating pure helper functions in a normal language definition.
- Review error behavior and compatibility notes whenever pure helper functions changes.

## Output determinism

- Output determinism: The parser records this concern as structured data and never evaluates the text as executable code.
- Output determinism: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Output determinism: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Output determinism: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Output determinism: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Output determinism: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating output determinism in a normal language definition.
- Review error behavior and compatibility notes whenever output determinism changes.

## Module naming

- Module naming: The parser records this concern as structured data and never evaluates the text as executable code.
- Module naming: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Module naming: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Module naming: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Module naming: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Module naming: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating module naming in a normal language definition.
- Review error behavior and compatibility notes whenever module naming changes.

## Output directory

- Output directory: The parser records this concern as structured data and never evaluates the text as executable code.
- Output directory: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Output directory: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Output directory: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Output directory: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Output directory: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating output directory in a normal language definition.
- Review error behavior and compatibility notes whenever output directory changes.

## File replacement policy

- File replacement policy: The parser records this concern as structured data and never evaluates the text as executable code.
- File replacement policy: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- File replacement policy: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- File replacement policy: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- File replacement policy: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- File replacement policy: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating file replacement policy in a normal language definition.
- Review error behavior and compatibility notes whenever file replacement policy changes.

## Generated file ownership

- Generated file ownership: The parser records this concern as structured data and never evaluates the text as executable code.
- Generated file ownership: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Generated file ownership: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Generated file ownership: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Generated file ownership: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Generated file ownership: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating generated file ownership in a normal language definition.
- Review error behavior and compatibility notes whenever generated file ownership changes.

## Future backends

- Future backends: The parser records this concern as structured data and never evaluates the text as executable code.
- Future backends: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Future backends: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Future backends: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Future backends: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Future backends: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating future backends in a normal language definition.
- Review error behavior and compatibility notes whenever future backends changes.

## Review checklist

- Does the change have a documented public contract?
- Are edge cases covered by automated tests?
- Does the command-line interface expose useful errors?
- Is generated output reproducible?

## Related implementation

The implementation is organized under src so each stage can be tested independently. See the module-level documentation and integration tests before changing a shared contract.
