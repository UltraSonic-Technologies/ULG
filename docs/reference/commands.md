# CLI Command Reference

The CLI is designed for automation and interactive use. Each command validates the input definition before reading language source or generating artifacts.

This handbook page is part of the ULG documentation set. It is written for language
authors, library users, and maintainers. Keep public behavior separate from
implementation details so future backends can reuse the same language model.

## ulg check

- ulg check: The parser records this concern as structured data and never evaluates the text as executable code.
- ulg check: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- ulg check: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- ulg check: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- ulg check: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- ulg check: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing ulg check in a valid language definition.
- Document migration behavior before changing ulg check.

## ulg inspect

- ulg inspect: The parser records this concern as structured data and never evaluates the text as executable code.
- ulg inspect: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- ulg inspect: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- ulg inspect: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- ulg inspect: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- ulg inspect: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing ulg inspect in a valid language definition.
- Document migration behavior before changing ulg inspect.

## ulg lex

- ulg lex: The parser records this concern as structured data and never evaluates the text as executable code.
- ulg lex: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- ulg lex: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- ulg lex: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- ulg lex: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- ulg lex: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing ulg lex in a valid language definition.
- Document migration behavior before changing ulg lex.

## ulg generate

- ulg generate: The parser records this concern as structured data and never evaluates the text as executable code.
- ulg generate: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- ulg generate: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- ulg generate: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- ulg generate: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- ulg generate: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing ulg generate in a valid language definition.
- Document migration behavior before changing ulg generate.

## Backend argument

- Backend argument: The parser records this concern as structured data and never evaluates the text as executable code.
- Backend argument: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Backend argument: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Backend argument: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Backend argument: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Backend argument: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing backend argument in a valid language definition.
- Document migration behavior before changing backend argument.

## Definition path

- Definition path: The parser records this concern as structured data and never evaluates the text as executable code.
- Definition path: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Definition path: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Definition path: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Definition path: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Definition path: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing definition path in a valid language definition.
- Document migration behavior before changing definition path.

## Source path

- Source path: The parser records this concern as structured data and never evaluates the text as executable code.
- Source path: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Source path: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Source path: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Source path: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Source path: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing source path in a valid language definition.
- Document migration behavior before changing source path.

## Output directory

- Output directory: The parser records this concern as structured data and never evaluates the text as executable code.
- Output directory: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Output directory: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Output directory: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Output directory: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Output directory: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing output directory in a valid language definition.
- Document migration behavior before changing output directory.

## Module option

- Module option: The parser records this concern as structured data and never evaluates the text as executable code.
- Module option: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Module option: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Module option: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Module option: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Module option: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing module option in a valid language definition.
- Document migration behavior before changing module option.

## Exit codes

- Exit codes: The parser records this concern as structured data and never evaluates the text as executable code.
- Exit codes: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Exit codes: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Exit codes: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Exit codes: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Exit codes: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing exit codes in a valid language definition.
- Document migration behavior before changing exit codes.

## Standard output

- Standard output: The parser records this concern as structured data and never evaluates the text as executable code.
- Standard output: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Standard output: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Standard output: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Standard output: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Standard output: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing standard output in a valid language definition.
- Document migration behavior before changing standard output.

## Standard error

- Standard error: The parser records this concern as structured data and never evaluates the text as executable code.
- Standard error: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Standard error: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Standard error: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Standard error: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Standard error: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing standard error in a valid language definition.
- Document migration behavior before changing standard error.

## Relative paths

- Relative paths: The parser records this concern as structured data and never evaluates the text as executable code.
- Relative paths: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Relative paths: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Relative paths: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Relative paths: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Relative paths: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing relative paths in a valid language definition.
- Document migration behavior before changing relative paths.

## Shell quoting

- Shell quoting: The parser records this concern as structured data and never evaluates the text as executable code.
- Shell quoting: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Shell quoting: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Shell quoting: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Shell quoting: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Shell quoting: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing shell quoting in a valid language definition.
- Document migration behavior before changing shell quoting.

## Operational checklist

- Validate every definition before invoking code generation.
- Preserve source locations in every diagnostic path.
- Keep generated output deterministic and safe to overwrite only by explicit request.
- Run the full test suite after changing a shared contract.
