# Diagnostics Architecture

Diagnostics carry stable codes, severity, messages, and optional spans. Byte offsets support safe slicing while line and column values support editor display.

This design note is part of the ULG engineering handbook. It describes responsibilities
for contributors and consumers of the library. Public behavior should be specified
independently from implementation details so multiple backends can share one model.

## Diagnostic code

- Diagnostic code: The parser records this concern as structured data and never evaluates the text as executable code.
- Diagnostic code: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Diagnostic code: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Diagnostic code: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Diagnostic code: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Diagnostic code: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating diagnostic code in a normal language definition.
- Review error behavior and compatibility notes whenever diagnostic code changes.

## Severity

- Severity: The parser records this concern as structured data and never evaluates the text as executable code.
- Severity: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Severity: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Severity: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Severity: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Severity: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating severity in a normal language definition.
- Review error behavior and compatibility notes whenever severity changes.

## Message text

- Message text: The parser records this concern as structured data and never evaluates the text as executable code.
- Message text: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Message text: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Message text: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Message text: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Message text: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating message text in a normal language definition.
- Review error behavior and compatibility notes whenever message text changes.

## Source span

- Source span: The parser records this concern as structured data and never evaluates the text as executable code.
- Source span: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Source span: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Source span: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Source span: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Source span: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating source span in a normal language definition.
- Review error behavior and compatibility notes whenever source span changes.

## Byte offset

- Byte offset: The parser records this concern as structured data and never evaluates the text as executable code.
- Byte offset: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Byte offset: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Byte offset: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Byte offset: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Byte offset: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating byte offset in a normal language definition.
- Review error behavior and compatibility notes whenever byte offset changes.

## Line coordinate

- Line coordinate: The parser records this concern as structured data and never evaluates the text as executable code.
- Line coordinate: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Line coordinate: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Line coordinate: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Line coordinate: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Line coordinate: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating line coordinate in a normal language definition.
- Review error behavior and compatibility notes whenever line coordinate changes.

## Column coordinate

- Column coordinate: The parser records this concern as structured data and never evaluates the text as executable code.
- Column coordinate: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Column coordinate: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Column coordinate: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Column coordinate: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Column coordinate: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating column coordinate in a normal language definition.
- Review error behavior and compatibility notes whenever column coordinate changes.

## Notes

- Notes: The parser records this concern as structured data and never evaluates the text as executable code.
- Notes: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Notes: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Notes: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Notes: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Notes: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating notes in a normal language definition.
- Review error behavior and compatibility notes whenever notes changes.

## CLI rendering

- CLI rendering: The parser records this concern as structured data and never evaluates the text as executable code.
- CLI rendering: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- CLI rendering: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- CLI rendering: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- CLI rendering: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- CLI rendering: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating cli rendering in a normal language definition.
- Review error behavior and compatibility notes whenever cli rendering changes.

## Editor rendering

- Editor rendering: The parser records this concern as structured data and never evaluates the text as executable code.
- Editor rendering: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Editor rendering: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Editor rendering: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Editor rendering: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Editor rendering: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating editor rendering in a normal language definition.
- Review error behavior and compatibility notes whenever editor rendering changes.

## Stable codes

- Stable codes: The parser records this concern as structured data and never evaluates the text as executable code.
- Stable codes: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Stable codes: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Stable codes: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Stable codes: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Stable codes: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating stable codes in a normal language definition.
- Review error behavior and compatibility notes whenever stable codes changes.

## Error aggregation

- Error aggregation: The parser records this concern as structured data and never evaluates the text as executable code.
- Error aggregation: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Error aggregation: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Error aggregation: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Error aggregation: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Error aggregation: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating error aggregation in a normal language definition.
- Review error behavior and compatibility notes whenever error aggregation changes.

## Diagnostic tests

- Diagnostic tests: The parser records this concern as structured data and never evaluates the text as executable code.
- Diagnostic tests: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Diagnostic tests: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Diagnostic tests: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Diagnostic tests: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Diagnostic tests: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating diagnostic tests in a normal language definition.
- Review error behavior and compatibility notes whenever diagnostic tests changes.

## Localization direction

- Localization direction: The parser records this concern as structured data and never evaluates the text as executable code.
- Localization direction: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Localization direction: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Localization direction: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Localization direction: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Localization direction: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating localization direction in a normal language definition.
- Review error behavior and compatibility notes whenever localization direction changes.

## Review checklist

- Does the change have a documented public contract?
- Are edge cases covered by automated tests?
- Does the command-line interface expose useful errors?
- Is generated output reproducible?

## Related implementation

The implementation is organized under src so each stage can be tested independently. See the module-level documentation and integration tests before changing a shared contract.
