# Parser Architecture

The parser recognizes ULG delimiters directly and produces a typed model. It is a reader, not an evaluator, and never executes declaration content.

This design note is part of the ULG engineering handbook. It describes responsibilities
for contributors and consumers of the library. Public behavior should be specified
independently from implementation details so multiple backends can share one model.

## Cursor abstraction

- Cursor abstraction: The parser records this concern as structured data and never evaluates the text as executable code.
- Cursor abstraction: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Cursor abstraction: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Cursor abstraction: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Cursor abstraction: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Cursor abstraction: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating cursor abstraction in a normal language definition.
- Review error behavior and compatibility notes whenever cursor abstraction changes.

## UTF-8 safety

- UTF-8 safety: The parser records this concern as structured data and never evaluates the text as executable code.
- UTF-8 safety: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- UTF-8 safety: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- UTF-8 safety: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- UTF-8 safety: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- UTF-8 safety: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating utf-8 safety in a normal language definition.
- Review error behavior and compatibility notes whenever utf-8 safety changes.

## Delimiter matching

- Delimiter matching: The parser records this concern as structured data and never evaluates the text as executable code.
- Delimiter matching: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Delimiter matching: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Delimiter matching: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Delimiter matching: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Delimiter matching: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating delimiter matching in a normal language definition.
- Review error behavior and compatibility notes whenever delimiter matching changes.

## Metadata parsing

- Metadata parsing: The parser records this concern as structured data and never evaluates the text as executable code.
- Metadata parsing: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Metadata parsing: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Metadata parsing: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Metadata parsing: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Metadata parsing: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating metadata parsing in a normal language definition.
- Review error behavior and compatibility notes whenever metadata parsing changes.

## List parsing

- List parsing: The parser records this concern as structured data and never evaluates the text as executable code.
- List parsing: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- List parsing: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- List parsing: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- List parsing: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- List parsing: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating list parsing in a normal language definition.
- Review error behavior and compatibility notes whenever list parsing changes.

## MATCH parsing

- MATCH parsing: The parser records this concern as structured data and never evaluates the text as executable code.
- MATCH parsing: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- MATCH parsing: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- MATCH parsing: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- MATCH parsing: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- MATCH parsing: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating match parsing in a normal language definition.
- Review error behavior and compatibility notes whenever match parsing changes.

## String decoding

- String decoding: The parser records this concern as structured data and never evaluates the text as executable code.
- String decoding: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- String decoding: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- String decoding: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- String decoding: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- String decoding: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating string decoding in a normal language definition.
- Review error behavior and compatibility notes whenever string decoding changes.

## Section dispatch

- Section dispatch: The parser records this concern as structured data and never evaluates the text as executable code.
- Section dispatch: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Section dispatch: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Section dispatch: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Section dispatch: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Section dispatch: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating section dispatch in a normal language definition.
- Review error behavior and compatibility notes whenever section dispatch changes.

## Duplicate section detection

- Duplicate section detection: The parser records this concern as structured data and never evaluates the text as executable code.
- Duplicate section detection: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Duplicate section detection: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Duplicate section detection: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Duplicate section detection: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Duplicate section detection: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating duplicate section detection in a normal language definition.
- Review error behavior and compatibility notes whenever duplicate section detection changes.

## Unexpected trailing input

- Unexpected trailing input: The parser records this concern as structured data and never evaluates the text as executable code.
- Unexpected trailing input: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unexpected trailing input: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unexpected trailing input: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unexpected trailing input: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unexpected trailing input: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating unexpected trailing input in a normal language definition.
- Review error behavior and compatibility notes whenever unexpected trailing input changes.

## Error locations

- Error locations: The parser records this concern as structured data and never evaluates the text as executable code.
- Error locations: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Error locations: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Error locations: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Error locations: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Error locations: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating error locations in a normal language definition.
- Review error behavior and compatibility notes whenever error locations changes.

## Parser tests

- Parser tests: The parser records this concern as structured data and never evaluates the text as executable code.
- Parser tests: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Parser tests: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Parser tests: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Parser tests: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Parser tests: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating parser tests in a normal language definition.
- Review error behavior and compatibility notes whenever parser tests changes.

## Resource boundaries

- Resource boundaries: The parser records this concern as structured data and never evaluates the text as executable code.
- Resource boundaries: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Resource boundaries: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Resource boundaries: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Resource boundaries: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Resource boundaries: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating resource boundaries in a normal language definition.
- Review error behavior and compatibility notes whenever resource boundaries changes.

## Grammar evolution

- Grammar evolution: The parser records this concern as structured data and never evaluates the text as executable code.
- Grammar evolution: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Grammar evolution: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Grammar evolution: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Grammar evolution: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Grammar evolution: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating grammar evolution in a normal language definition.
- Review error behavior and compatibility notes whenever grammar evolution changes.

## Review checklist

- Does the change have a documented public contract?
- Are edge cases covered by automated tests?
- Does the command-line interface expose useful errors?
- Is generated output reproducible?

## Related implementation

The implementation is organized under src so each stage can be tested independently. See the module-level documentation and integration tests before changing a shared contract.
