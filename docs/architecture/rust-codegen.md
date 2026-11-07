# Rust Code Generation

The Rust backend produces a module that downstream projects can include without depending on the ULG command-line application.

This design note is part of the ULG engineering handbook. It describes responsibilities
for contributors and consumers of the library. Public behavior should be specified
independently from implementation details so multiple backends can share one model.

## Generated module boundary

- Generated module boundary: The parser records this concern as structured data and never evaluates the text as executable code.
- Generated module boundary: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Generated module boundary: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Generated module boundary: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Generated module boundary: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Generated module boundary: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating generated module boundary in a normal language definition.
- Review error behavior and compatibility notes whenever generated module boundary changes.

## Module name

- Module name: The parser records this concern as structured data and never evaluates the text as executable code.
- Module name: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Module name: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Module name: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Module name: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Module name: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating module name in a normal language definition.
- Review error behavior and compatibility notes whenever module name changes.

## Metadata constants

- Metadata constants: The parser records this concern as structured data and never evaluates the text as executable code.
- Metadata constants: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Metadata constants: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Metadata constants: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Metadata constants: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Metadata constants: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating metadata constants in a normal language definition.
- Review error behavior and compatibility notes whenever metadata constants changes.

## Keyword slice

- Keyword slice: The parser records this concern as structured data and never evaluates the text as executable code.
- Keyword slice: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Keyword slice: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Keyword slice: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Keyword slice: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Keyword slice: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating keyword slice in a normal language definition.
- Review error behavior and compatibility notes whenever keyword slice changes.

## Operator slice

- Operator slice: The parser records this concern as structured data and never evaluates the text as executable code.
- Operator slice: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Operator slice: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Operator slice: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Operator slice: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Operator slice: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating operator slice in a normal language definition.
- Review error behavior and compatibility notes whenever operator slice changes.

## Comment slice

- Comment slice: The parser records this concern as structured data and never evaluates the text as executable code.
- Comment slice: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Comment slice: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Comment slice: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Comment slice: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Comment slice: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating comment slice in a normal language definition.
- Review error behavior and compatibility notes whenever comment slice changes.

## Punctuation slice

- Punctuation slice: The parser records this concern as structured data and never evaluates the text as executable code.
- Punctuation slice: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Punctuation slice: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Punctuation slice: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Punctuation slice: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Punctuation slice: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating punctuation slice in a normal language definition.
- Review error behavior and compatibility notes whenever punctuation slice changes.

## String escaping

- String escaping: The parser records this concern as structured data and never evaluates the text as executable code.
- String escaping: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- String escaping: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- String escaping: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- String escaping: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- String escaping: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating string escaping in a normal language definition.
- Review error behavior and compatibility notes whenever string escaping changes.

## Pure lookup functions

- Pure lookup functions: The parser records this concern as structured data and never evaluates the text as executable code.
- Pure lookup functions: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Pure lookup functions: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Pure lookup functions: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Pure lookup functions: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Pure lookup functions: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating pure lookup functions in a normal language definition.
- Review error behavior and compatibility notes whenever pure lookup functions changes.

## Deterministic order

- Deterministic order: The parser records this concern as structured data and never evaluates the text as executable code.
- Deterministic order: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Deterministic order: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Deterministic order: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Deterministic order: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Deterministic order: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating deterministic order in a normal language definition.
- Review error behavior and compatibility notes whenever deterministic order changes.

## Source control strategy

- Source control strategy: The parser records this concern as structured data and never evaluates the text as executable code.
- Source control strategy: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Source control strategy: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Source control strategy: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Source control strategy: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Source control strategy: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating source control strategy in a normal language definition.
- Review error behavior and compatibility notes whenever source control strategy changes.

## Regeneration workflow

- Regeneration workflow: The parser records this concern as structured data and never evaluates the text as executable code.
- Regeneration workflow: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Regeneration workflow: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Regeneration workflow: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Regeneration workflow: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Regeneration workflow: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating regeneration workflow in a normal language definition.
- Review error behavior and compatibility notes whenever regeneration workflow changes.

## Compiling generated output

- Compiling generated output: The parser records this concern as structured data and never evaluates the text as executable code.
- Compiling generated output: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Compiling generated output: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Compiling generated output: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Compiling generated output: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Compiling generated output: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating compiling generated output in a normal language definition.
- Review error behavior and compatibility notes whenever compiling generated output changes.

## Backend extension

- Backend extension: The parser records this concern as structured data and never evaluates the text as executable code.
- Backend extension: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Backend extension: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Backend extension: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Backend extension: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Backend extension: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating backend extension in a normal language definition.
- Review error behavior and compatibility notes whenever backend extension changes.

## Review checklist

- Does the change have a documented public contract?
- Are edge cases covered by automated tests?
- Does the command-line interface expose useful errors?
- Is generated output reproducible?

## Related implementation

The implementation is organized under src so each stage can be tested independently. See the module-level documentation and integration tests before changing a shared contract.
