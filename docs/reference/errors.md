# Error Reference

Errors identify the phase and location where a definition or source file became invalid. Stable codes let editor clients group diagnostics.

This handbook page is part of the ULG documentation set. It is written for language
authors, library users, and maintainers. Keep public behavior separate from
implementation details so future backends can reuse the same language model.

## Syntax error

- Syntax error: The parser records this concern as structured data and never evaluates the text as executable code.
- Syntax error: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Syntax error: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Syntax error: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Syntax error: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Syntax error: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing syntax error in a valid language definition.
- Document migration behavior before changing syntax error.

## Missing metadata

- Missing metadata: The parser records this concern as structured data and never evaluates the text as executable code.
- Missing metadata: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Missing metadata: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Missing metadata: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Missing metadata: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Missing metadata: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing missing metadata in a valid language definition.
- Document migration behavior before changing missing metadata.

## Invalid version

- Invalid version: The parser records this concern as structured data and never evaluates the text as executable code.
- Invalid version: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Invalid version: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Invalid version: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Invalid version: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Invalid version: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing invalid version in a valid language definition.
- Document migration behavior before changing invalid version.

## Unknown section

- Unknown section: The parser records this concern as structured data and never evaluates the text as executable code.
- Unknown section: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unknown section: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unknown section: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unknown section: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unknown section: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing unknown section in a valid language definition.
- Document migration behavior before changing unknown section.

## Duplicate section

- Duplicate section: The parser records this concern as structured data and never evaluates the text as executable code.
- Duplicate section: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Duplicate section: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Duplicate section: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Duplicate section: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Duplicate section: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing duplicate section in a valid language definition.
- Document migration behavior before changing duplicate section.

## Duplicate entry

- Duplicate entry: The parser records this concern as structured data and never evaluates the text as executable code.
- Duplicate entry: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Duplicate entry: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Duplicate entry: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Duplicate entry: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Duplicate entry: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing duplicate entry in a valid language definition.
- Document migration behavior before changing duplicate entry.

## Empty entry

- Empty entry: The parser records this concern as structured data and never evaluates the text as executable code.
- Empty entry: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Empty entry: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Empty entry: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Empty entry: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Empty entry: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing empty entry in a valid language definition.
- Document migration behavior before changing empty entry.

## Invalid comment

- Invalid comment: The parser records this concern as structured data and never evaluates the text as executable code.
- Invalid comment: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Invalid comment: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Invalid comment: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Invalid comment: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Invalid comment: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing invalid comment in a valid language definition.
- Document migration behavior before changing invalid comment.

## Invalid regex

- Invalid regex: The parser records this concern as structured data and never evaluates the text as executable code.
- Invalid regex: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Invalid regex: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Invalid regex: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Invalid regex: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Invalid regex: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing invalid regex in a valid language definition.
- Document migration behavior before changing invalid regex.

## Unexpected source character

- Unexpected source character: The parser records this concern as structured data and never evaluates the text as executable code.
- Unexpected source character: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unexpected source character: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unexpected source character: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unexpected source character: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unexpected source character: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing unexpected source character in a valid language definition.
- Document migration behavior before changing unexpected source character.

## Unterminated literal

- Unterminated literal: The parser records this concern as structured data and never evaluates the text as executable code.
- Unterminated literal: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unterminated literal: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unterminated literal: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unterminated literal: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unterminated literal: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing unterminated literal in a valid language definition.
- Document migration behavior before changing unterminated literal.

## Unterminated comment

- Unterminated comment: The parser records this concern as structured data and never evaluates the text as executable code.
- Unterminated comment: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unterminated comment: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unterminated comment: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unterminated comment: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unterminated comment: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing unterminated comment in a valid language definition.
- Document migration behavior before changing unterminated comment.

## I/O error

- I/O error: The parser records this concern as structured data and never evaluates the text as executable code.
- I/O error: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- I/O error: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- I/O error: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- I/O error: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- I/O error: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing i/o error in a valid language definition.
- Document migration behavior before changing i/o error.

## Exit status

- Exit status: The parser records this concern as structured data and never evaluates the text as executable code.
- Exit status: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Exit status: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Exit status: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Exit status: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Exit status: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing exit status in a valid language definition.
- Document migration behavior before changing exit status.

## Operational checklist

- Validate every definition before invoking code generation.
- Preserve source locations in every diagnostic path.
- Keep generated output deterministic and safe to overwrite only by explicit request.
- Run the full test suite after changing a shared contract.
