# ULG Architecture

The reference implementation separates parsing, modeling, validation, lexing, and code generation. Each stage has a narrow contract and can be embedded independently.

This design note is part of the ULG engineering handbook. It describes responsibilities
for contributors and consumers of the library. Public behavior should be specified
independently from implementation details so multiple backends can share one model.

## Input boundary

- Input boundary: The parser records this concern as structured data and never evaluates the text as executable code.
- Input boundary: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Input boundary: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Input boundary: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Input boundary: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Input boundary: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating input boundary in a normal language definition.
- Review error behavior and compatibility notes whenever input boundary changes.

## Parser stage

- Parser stage: The parser records this concern as structured data and never evaluates the text as executable code.
- Parser stage: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Parser stage: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Parser stage: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Parser stage: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Parser stage: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating parser stage in a normal language definition.
- Review error behavior and compatibility notes whenever parser stage changes.

## Model layer

- Model layer: The parser records this concern as structured data and never evaluates the text as executable code.
- Model layer: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Model layer: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Model layer: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Model layer: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Model layer: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating model layer in a normal language definition.
- Review error behavior and compatibility notes whenever model layer changes.

## Validation stage

- Validation stage: The parser records this concern as structured data and never evaluates the text as executable code.
- Validation stage: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Validation stage: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Validation stage: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Validation stage: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Validation stage: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating validation stage in a normal language definition.
- Review error behavior and compatibility notes whenever validation stage changes.

## Lexical engine

- Lexical engine: The parser records this concern as structured data and never evaluates the text as executable code.
- Lexical engine: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Lexical engine: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Lexical engine: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Lexical engine: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Lexical engine: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating lexical engine in a normal language definition.
- Review error behavior and compatibility notes whenever lexical engine changes.

## Token stream

- Token stream: The parser records this concern as structured data and never evaluates the text as executable code.
- Token stream: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Token stream: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Token stream: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Token stream: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Token stream: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating token stream in a normal language definition.
- Review error behavior and compatibility notes whenever token stream changes.

## Code-generation backend

- Code-generation backend: The parser records this concern as structured data and never evaluates the text as executable code.
- Code-generation backend: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Code-generation backend: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Code-generation backend: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Code-generation backend: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Code-generation backend: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating code-generation backend in a normal language definition.
- Review error behavior and compatibility notes whenever code-generation backend changes.

## Command-line adapter

- Command-line adapter: The parser records this concern as structured data and never evaluates the text as executable code.
- Command-line adapter: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Command-line adapter: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Command-line adapter: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Command-line adapter: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Command-line adapter: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating command-line adapter in a normal language definition.
- Review error behavior and compatibility notes whenever command-line adapter changes.

## Diagnostics layer

- Diagnostics layer: The parser records this concern as structured data and never evaluates the text as executable code.
- Diagnostics layer: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Diagnostics layer: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Diagnostics layer: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Diagnostics layer: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Diagnostics layer: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating diagnostics layer in a normal language definition.
- Review error behavior and compatibility notes whenever diagnostics layer changes.

## Library API

- Library API: The parser records this concern as structured data and never evaluates the text as executable code.
- Library API: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Library API: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Library API: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Library API: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Library API: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating library api in a normal language definition.
- Review error behavior and compatibility notes whenever library api changes.

## Build pipeline

- Build pipeline: The parser records this concern as structured data and never evaluates the text as executable code.
- Build pipeline: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Build pipeline: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Build pipeline: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Build pipeline: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Build pipeline: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating build pipeline in a normal language definition.
- Review error behavior and compatibility notes whenever build pipeline changes.

## Trust boundary

- Trust boundary: The parser records this concern as structured data and never evaluates the text as executable code.
- Trust boundary: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Trust boundary: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Trust boundary: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Trust boundary: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Trust boundary: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating trust boundary in a normal language definition.
- Review error behavior and compatibility notes whenever trust boundary changes.

## Testing strategy

- Testing strategy: The parser records this concern as structured data and never evaluates the text as executable code.
- Testing strategy: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Testing strategy: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Testing strategy: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Testing strategy: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Testing strategy: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating testing strategy in a normal language definition.
- Review error behavior and compatibility notes whenever testing strategy changes.

## Extension points

- Extension points: The parser records this concern as structured data and never evaluates the text as executable code.
- Extension points: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Extension points: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Extension points: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Extension points: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Extension points: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating extension points in a normal language definition.
- Review error behavior and compatibility notes whenever extension points changes.

## Review checklist

- Does the change have a documented public contract?
- Are edge cases covered by automated tests?
- Does the command-line interface expose useful errors?
- Is generated output reproducible?

## Related implementation

The implementation is organized under src so each stage can be tested independently. See the module-level documentation and integration tests before changing a shared contract.
