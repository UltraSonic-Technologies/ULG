# CLI Architecture

The command-line application exposes check, inspect, lex, and generate operations. It is an adapter over the library rather than the location of compiler logic.

This design note is part of the ULG engineering handbook. It describes responsibilities
for contributors and consumers of the library. Public behavior should be specified
independently from implementation details so multiple backends can share one model.

## Argument parsing

- Argument parsing: The parser records this concern as structured data and never evaluates the text as executable code.
- Argument parsing: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Argument parsing: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Argument parsing: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Argument parsing: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Argument parsing: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating argument parsing in a normal language definition.
- Review error behavior and compatibility notes whenever argument parsing changes.

## Check command

- Check command: The parser records this concern as structured data and never evaluates the text as executable code.
- Check command: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Check command: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Check command: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Check command: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Check command: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating check command in a normal language definition.
- Review error behavior and compatibility notes whenever check command changes.

## Inspect command

- Inspect command: The parser records this concern as structured data and never evaluates the text as executable code.
- Inspect command: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Inspect command: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Inspect command: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Inspect command: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Inspect command: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating inspect command in a normal language definition.
- Review error behavior and compatibility notes whenever inspect command changes.

## Lex command

- Lex command: The parser records this concern as structured data and never evaluates the text as executable code.
- Lex command: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Lex command: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Lex command: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Lex command: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Lex command: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating lex command in a normal language definition.
- Review error behavior and compatibility notes whenever lex command changes.

## Generate command

- Generate command: The parser records this concern as structured data and never evaluates the text as executable code.
- Generate command: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Generate command: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Generate command: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Generate command: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Generate command: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating generate command in a normal language definition.
- Review error behavior and compatibility notes whenever generate command changes.

## Backend selection

- Backend selection: The parser records this concern as structured data and never evaluates the text as executable code.
- Backend selection: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Backend selection: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Backend selection: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Backend selection: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Backend selection: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating backend selection in a normal language definition.
- Review error behavior and compatibility notes whenever backend selection changes.

## Output path

- Output path: The parser records this concern as structured data and never evaluates the text as executable code.
- Output path: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Output path: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Output path: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Output path: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Output path: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating output path in a normal language definition.
- Review error behavior and compatibility notes whenever output path changes.

## Exit status

- Exit status: The parser records this concern as structured data and never evaluates the text as executable code.
- Exit status: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Exit status: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Exit status: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Exit status: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Exit status: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating exit status in a normal language definition.
- Review error behavior and compatibility notes whenever exit status changes.

## Error presentation

- Error presentation: The parser records this concern as structured data and never evaluates the text as executable code.
- Error presentation: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Error presentation: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Error presentation: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Error presentation: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Error presentation: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating error presentation in a normal language definition.
- Review error behavior and compatibility notes whenever error presentation changes.

## Script automation

- Script automation: The parser records this concern as structured data and never evaluates the text as executable code.
- Script automation: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Script automation: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Script automation: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Script automation: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Script automation: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating script automation in a normal language definition.
- Review error behavior and compatibility notes whenever script automation changes.

## Stable command names

- Stable command names: The parser records this concern as structured data and never evaluates the text as executable code.
- Stable command names: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Stable command names: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Stable command names: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Stable command names: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Stable command names: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating stable command names in a normal language definition.
- Review error behavior and compatibility notes whenever stable command names changes.

## Path handling

- Path handling: The parser records this concern as structured data and never evaluates the text as executable code.
- Path handling: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Path handling: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Path handling: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Path handling: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Path handling: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating path handling in a normal language definition.
- Review error behavior and compatibility notes whenever path handling changes.

## CLI integration tests

- CLI integration tests: The parser records this concern as structured data and never evaluates the text as executable code.
- CLI integration tests: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- CLI integration tests: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- CLI integration tests: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- CLI integration tests: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- CLI integration tests: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating cli integration tests in a normal language definition.
- Review error behavior and compatibility notes whenever cli integration tests changes.

## Future package commands

- Future package commands: The parser records this concern as structured data and never evaluates the text as executable code.
- Future package commands: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Future package commands: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Future package commands: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Future package commands: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Future package commands: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating future package commands in a normal language definition.
- Review error behavior and compatibility notes whenever future package commands changes.

## Review checklist

- Does the change have a documented public contract?
- Are edge cases covered by automated tests?
- Does the command-line interface expose useful errors?
- Is generated output reproducible?

## Related implementation

The implementation is organized under src so each stage can be tested independently. See the module-level documentation and integration tests before changing a shared contract.
