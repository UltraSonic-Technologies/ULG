# Testing ULG Implementations

A robust test suite includes valid golden documents, malformed inputs, lexer boundary cases, and generated-source compilation checks.

This handbook page is part of the ULG documentation set. It is written for language
authors, library users, and maintainers. Keep public behavior separate from
implementation details so future backends can reuse the same language model.

## Golden definitions

- Golden definitions: The parser records this concern as structured data and never evaluates the text as executable code.
- Golden definitions: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Golden definitions: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Golden definitions: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Golden definitions: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Golden definitions: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing golden definitions in a valid language definition.
- Document migration behavior before changing golden definitions.

## Minimal fixtures

- Minimal fixtures: The parser records this concern as structured data and never evaluates the text as executable code.
- Minimal fixtures: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Minimal fixtures: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Minimal fixtures: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Minimal fixtures: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Minimal fixtures: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing minimal fixtures in a valid language definition.
- Document migration behavior before changing minimal fixtures.

## UltimateX fixture

- UltimateX fixture: The parser records this concern as structured data and never evaluates the text as executable code.
- UltimateX fixture: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- UltimateX fixture: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- UltimateX fixture: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- UltimateX fixture: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- UltimateX fixture: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing ultimatex fixture in a valid language definition.
- Document migration behavior before changing ultimatex fixture.

## Malformed metadata

- Malformed metadata: The parser records this concern as structured data and never evaluates the text as executable code.
- Malformed metadata: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Malformed metadata: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Malformed metadata: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Malformed metadata: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Malformed metadata: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing malformed metadata in a valid language definition.
- Document migration behavior before changing malformed metadata.

## Missing markers

- Missing markers: The parser records this concern as structured data and never evaluates the text as executable code.
- Missing markers: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Missing markers: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Missing markers: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Missing markers: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Missing markers: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing missing markers in a valid language definition.
- Document migration behavior before changing missing markers.

## Duplicate sections

- Duplicate sections: The parser records this concern as structured data and never evaluates the text as executable code.
- Duplicate sections: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Duplicate sections: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Duplicate sections: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Duplicate sections: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Duplicate sections: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing duplicate sections in a valid language definition.
- Document migration behavior before changing duplicate sections.

## Escaped strings

- Escaped strings: The parser records this concern as structured data and never evaluates the text as executable code.
- Escaped strings: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Escaped strings: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Escaped strings: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Escaped strings: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Escaped strings: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing escaped strings in a valid language definition.
- Document migration behavior before changing escaped strings.

## Unicode identifiers

- Unicode identifiers: The parser records this concern as structured data and never evaluates the text as executable code.
- Unicode identifiers: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unicode identifiers: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unicode identifiers: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unicode identifiers: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unicode identifiers: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing unicode identifiers in a valid language definition.
- Document migration behavior before changing unicode identifiers.

## Comment boundaries

- Comment boundaries: The parser records this concern as structured data and never evaluates the text as executable code.
- Comment boundaries: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Comment boundaries: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Comment boundaries: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Comment boundaries: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Comment boundaries: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing comment boundaries in a valid language definition.
- Document migration behavior before changing comment boundaries.

## Operator prefixes

- Operator prefixes: The parser records this concern as structured data and never evaluates the text as executable code.
- Operator prefixes: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Operator prefixes: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Operator prefixes: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Operator prefixes: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Operator prefixes: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing operator prefixes in a valid language definition.
- Document migration behavior before changing operator prefixes.

## Punctuation regexes

- Punctuation regexes: The parser records this concern as structured data and never evaluates the text as executable code.
- Punctuation regexes: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Punctuation regexes: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Punctuation regexes: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Punctuation regexes: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Punctuation regexes: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing punctuation regexes in a valid language definition.
- Document migration behavior before changing punctuation regexes.

## Generated module tests

- Generated module tests: The parser records this concern as structured data and never evaluates the text as executable code.
- Generated module tests: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Generated module tests: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Generated module tests: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Generated module tests: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Generated module tests: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing generated module tests in a valid language definition.
- Document migration behavior before changing generated module tests.

## CLI process tests

- CLI process tests: The parser records this concern as structured data and never evaluates the text as executable code.
- CLI process tests: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- CLI process tests: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- CLI process tests: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- CLI process tests: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- CLI process tests: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing cli process tests in a valid language definition.
- Document migration behavior before changing cli process tests.

## Regression policy

- Regression policy: The parser records this concern as structured data and never evaluates the text as executable code.
- Regression policy: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Regression policy: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Regression policy: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Regression policy: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Regression policy: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing regression policy in a valid language definition.
- Document migration behavior before changing regression policy.

## Operational checklist

- Validate every definition before invoking code generation.
- Preserve source locations in every diagnostic path.
- Keep generated output deterministic and safe to overwrite only by explicit request.
- Run the full test suite after changing a shared contract.
