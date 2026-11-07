# Debugging Definitions

When a definition fails, isolate the stage that rejected it. Parser errors indicate structure issues; validation errors indicate inconsistent declarations; lexer errors concern source text.

This handbook page is part of the ULG documentation set. It is written for language
authors, library users, and maintainers. Keep public behavior separate from
implementation details so future backends can reuse the same language model.

## Reproduce the input

- Reproduce the input: The parser records this concern as structured data and never evaluates the text as executable code.
- Reproduce the input: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Reproduce the input: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Reproduce the input: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Reproduce the input: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Reproduce the input: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing reproduce the input in a valid language definition.
- Document migration behavior before changing reproduce the input.

## Check delimiters

- Check delimiters: The parser records this concern as structured data and never evaluates the text as executable code.
- Check delimiters: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Check delimiters: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Check delimiters: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Check delimiters: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Check delimiters: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing check delimiters in a valid language definition.
- Document migration behavior before changing check delimiters.

## Inspect metadata

- Inspect metadata: The parser records this concern as structured data and never evaluates the text as executable code.
- Inspect metadata: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Inspect metadata: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Inspect metadata: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Inspect metadata: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Inspect metadata: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing inspect metadata in a valid language definition.
- Document migration behavior before changing inspect metadata.

## Check list separators

- Check list separators: The parser records this concern as structured data and never evaluates the text as executable code.
- Check list separators: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Check list separators: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Check list separators: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Check list separators: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Check list separators: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing check list separators in a valid language definition.
- Document migration behavior before changing check list separators.

## Inspect MATCH arrows

- Inspect MATCH arrows: The parser records this concern as structured data and never evaluates the text as executable code.
- Inspect MATCH arrows: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Inspect MATCH arrows: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Inspect MATCH arrows: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Inspect MATCH arrows: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Inspect MATCH arrows: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing inspect match arrows in a valid language definition.
- Document migration behavior before changing inspect match arrows.

## Validate duplicate entries

- Validate duplicate entries: The parser records this concern as structured data and never evaluates the text as executable code.
- Validate duplicate entries: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Validate duplicate entries: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Validate duplicate entries: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Validate duplicate entries: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Validate duplicate entries: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing validate duplicate entries in a valid language definition.
- Document migration behavior before changing validate duplicate entries.

## Compile regex patterns

- Compile regex patterns: The parser records this concern as structured data and never evaluates the text as executable code.
- Compile regex patterns: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Compile regex patterns: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Compile regex patterns: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Compile regex patterns: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Compile regex patterns: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing compile regex patterns in a valid language definition.
- Document migration behavior before changing compile regex patterns.

## Check comment openers

- Check comment openers: The parser records this concern as structured data and never evaluates the text as executable code.
- Check comment openers: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Check comment openers: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Check comment openers: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Check comment openers: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Check comment openers: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing check comment openers in a valid language definition.
- Document migration behavior before changing check comment openers.

## Check operator order

- Check operator order: The parser records this concern as structured data and never evaluates the text as executable code.
- Check operator order: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Check operator order: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Check operator order: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Check operator order: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Check operator order: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing check operator order in a valid language definition.
- Document migration behavior before changing check operator order.

## Inspect token spans

- Inspect token spans: The parser records this concern as structured data and never evaluates the text as executable code.
- Inspect token spans: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Inspect token spans: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Inspect token spans: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Inspect token spans: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Inspect token spans: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing inspect token spans in a valid language definition.
- Document migration behavior before changing inspect token spans.

## Reduce the source file

- Reduce the source file: The parser records this concern as structured data and never evaluates the text as executable code.
- Reduce the source file: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Reduce the source file: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Reduce the source file: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Reduce the source file: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Reduce the source file: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing reduce the source file in a valid language definition.
- Document migration behavior before changing reduce the source file.

## Use CLI commands

- Use CLI commands: The parser records this concern as structured data and never evaluates the text as executable code.
- Use CLI commands: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Use CLI commands: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Use CLI commands: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Use CLI commands: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Use CLI commands: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing use cli commands in a valid language definition.
- Document migration behavior before changing use cli commands.

## Add a regression fixture

- Add a regression fixture: The parser records this concern as structured data and never evaluates the text as executable code.
- Add a regression fixture: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Add a regression fixture: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Add a regression fixture: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Add a regression fixture: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Add a regression fixture: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing add a regression fixture in a valid language definition.
- Document migration behavior before changing add a regression fixture.

## Report actionable failures

- Report actionable failures: The parser records this concern as structured data and never evaluates the text as executable code.
- Report actionable failures: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Report actionable failures: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Report actionable failures: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Report actionable failures: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Report actionable failures: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing report actionable failures in a valid language definition.
- Document migration behavior before changing report actionable failures.

## Operational checklist

- Validate every definition before invoking code generation.
- Preserve source locations in every diagnostic path.
- Keep generated output deterministic and safe to overwrite only by explicit request.
- Run the full test suite after changing a shared contract.
