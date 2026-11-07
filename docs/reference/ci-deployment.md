# CI and Deployment

ULG can validate language definitions in continuous integration and regenerate checked-in Rust tables. Builds should pin the toolchain and dependency resolution.

This handbook page is part of the ULG documentation set. It is written for language
authors, library users, and maintainers. Keep public behavior separate from
implementation details so future backends can reuse the same language model.

## Toolchain selection

- Toolchain selection: The parser records this concern as structured data and never evaluates the text as executable code.
- Toolchain selection: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Toolchain selection: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Toolchain selection: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Toolchain selection: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Toolchain selection: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing toolchain selection in a valid language definition.
- Document migration behavior before changing toolchain selection.

## Cargo lockfile

- Cargo lockfile: The parser records this concern as structured data and never evaluates the text as executable code.
- Cargo lockfile: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Cargo lockfile: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Cargo lockfile: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Cargo lockfile: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Cargo lockfile: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing cargo lockfile in a valid language definition.
- Document migration behavior before changing cargo lockfile.

## Formatting checks

- Formatting checks: The parser records this concern as structured data and never evaluates the text as executable code.
- Formatting checks: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Formatting checks: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Formatting checks: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Formatting checks: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Formatting checks: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing formatting checks in a valid language definition.
- Document migration behavior before changing formatting checks.

## Clippy checks

- Clippy checks: The parser records this concern as structured data and never evaluates the text as executable code.
- Clippy checks: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Clippy checks: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Clippy checks: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Clippy checks: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Clippy checks: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing clippy checks in a valid language definition.
- Document migration behavior before changing clippy checks.

## Unit tests

- Unit tests: The parser records this concern as structured data and never evaluates the text as executable code.
- Unit tests: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unit tests: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unit tests: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unit tests: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unit tests: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing unit tests in a valid language definition.
- Document migration behavior before changing unit tests.

## Integration tests

- Integration tests: The parser records this concern as structured data and never evaluates the text as executable code.
- Integration tests: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Integration tests: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Integration tests: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Integration tests: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Integration tests: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing integration tests in a valid language definition.
- Document migration behavior before changing integration tests.

## Definition validation

- Definition validation: The parser records this concern as structured data and never evaluates the text as executable code.
- Definition validation: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Definition validation: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Definition validation: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Definition validation: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Definition validation: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing definition validation in a valid language definition.
- Document migration behavior before changing definition validation.

## Generated output checks

- Generated output checks: The parser records this concern as structured data and never evaluates the text as executable code.
- Generated output checks: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Generated output checks: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Generated output checks: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Generated output checks: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Generated output checks: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing generated output checks in a valid language definition.
- Document migration behavior before changing generated output checks.

## Cross-platform builds

- Cross-platform builds: The parser records this concern as structured data and never evaluates the text as executable code.
- Cross-platform builds: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Cross-platform builds: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Cross-platform builds: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Cross-platform builds: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Cross-platform builds: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing cross-platform builds in a valid language definition.
- Document migration behavior before changing cross-platform builds.

## Artifact naming

- Artifact naming: The parser records this concern as structured data and never evaluates the text as executable code.
- Artifact naming: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Artifact naming: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Artifact naming: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Artifact naming: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Artifact naming: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing artifact naming in a valid language definition.
- Document migration behavior before changing artifact naming.

## Release tags

- Release tags: The parser records this concern as structured data and never evaluates the text as executable code.
- Release tags: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Release tags: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Release tags: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Release tags: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Release tags: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing release tags in a valid language definition.
- Document migration behavior before changing release tags.

## Checksum publication

- Checksum publication: The parser records this concern as structured data and never evaluates the text as executable code.
- Checksum publication: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Checksum publication: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Checksum publication: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Checksum publication: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Checksum publication: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing checksum publication in a valid language definition.
- Document migration behavior before changing checksum publication.

## Reproducible builds

- Reproducible builds: The parser records this concern as structured data and never evaluates the text as executable code.
- Reproducible builds: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Reproducible builds: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Reproducible builds: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Reproducible builds: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Reproducible builds: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing reproducible builds in a valid language definition.
- Document migration behavior before changing reproducible builds.

## Failure reporting

- Failure reporting: The parser records this concern as structured data and never evaluates the text as executable code.
- Failure reporting: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Failure reporting: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Failure reporting: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Failure reporting: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Failure reporting: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing failure reporting in a valid language definition.
- Document migration behavior before changing failure reporting.

## Operational checklist

- Validate every definition before invoking code generation.
- Preserve source locations in every diagnostic path.
- Keep generated output deterministic and safe to overwrite only by explicit request.
- Run the full test suite after changing a shared contract.
