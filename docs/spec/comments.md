# Comment Declarations

Comment declarations define line or block delimiters. One marker denotes a line comment; two whitespace-separated markers denote a block comment.

This page is part of the ULG reference set. It describes behavior for language authors,
library users, and maintainers. Changes to externally visible syntax require
specification updates, migration notes, and regression coverage.

## Line comment form

- Line comment form: The parser records this concern as structured data and never evaluates the text as executable code.
- Line comment form: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Line comment form: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Line comment form: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Line comment form: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Line comment form: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how line comment form interacts with other sections.
- Document compatibility consequences before changing line comment form.

## Block comment form

- Block comment form: The parser records this concern as structured data and never evaluates the text as executable code.
- Block comment form: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Block comment form: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Block comment form: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Block comment form: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Block comment form: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how block comment form interacts with other sections.
- Document compatibility consequences before changing block comment form.

## Opening marker

- Opening marker: The parser records this concern as structured data and never evaluates the text as executable code.
- Opening marker: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Opening marker: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Opening marker: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Opening marker: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Opening marker: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how opening marker interacts with other sections.
- Document compatibility consequences before changing opening marker.

## Closing marker

- Closing marker: The parser records this concern as structured data and never evaluates the text as executable code.
- Closing marker: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Closing marker: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Closing marker: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Closing marker: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Closing marker: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how closing marker interacts with other sections.
- Document compatibility consequences before changing closing marker.

## Identical delimiters

- Identical delimiters: The parser records this concern as structured data and never evaluates the text as executable code.
- Identical delimiters: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Identical delimiters: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Identical delimiters: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Identical delimiters: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Identical delimiters: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how identical delimiters interacts with other sections.
- Document compatibility consequences before changing identical delimiters.

## Marker prefixes

- Marker prefixes: The parser records this concern as structured data and never evaluates the text as executable code.
- Marker prefixes: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Marker prefixes: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Marker prefixes: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Marker prefixes: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Marker prefixes: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how marker prefixes interacts with other sections.
- Document compatibility consequences before changing marker prefixes.

## Boundary checks

- Boundary checks: The parser records this concern as structured data and never evaluates the text as executable code.
- Boundary checks: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Boundary checks: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Boundary checks: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Boundary checks: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Boundary checks: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how boundary checks interacts with other sections.
- Document compatibility consequences before changing boundary checks.

## Multiline scanning

- Multiline scanning: The parser records this concern as structured data and never evaluates the text as executable code.
- Multiline scanning: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Multiline scanning: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Multiline scanning: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Multiline scanning: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Multiline scanning: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how multiline scanning interacts with other sections.
- Document compatibility consequences before changing multiline scanning.

## Unterminated blocks

- Unterminated blocks: The parser records this concern as structured data and never evaluates the text as executable code.
- Unterminated blocks: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unterminated blocks: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unterminated blocks: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unterminated blocks: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unterminated blocks: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how unterminated blocks interacts with other sections.
- Document compatibility consequences before changing unterminated blocks.

## Comment token spans

- Comment token spans: The parser records this concern as structured data and never evaluates the text as executable code.
- Comment token spans: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Comment token spans: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Comment token spans: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Comment token spans: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Comment token spans: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how comment token spans interacts with other sections.
- Document compatibility consequences before changing comment token spans.

## Comments inside strings

- Comments inside strings: The parser records this concern as structured data and never evaluates the text as executable code.
- Comments inside strings: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Comments inside strings: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Comments inside strings: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Comments inside strings: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Comments inside strings: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how comments inside strings interacts with other sections.
- Document compatibility consequences before changing comments inside strings.

## String escape precedence

- String escape precedence: The parser records this concern as structured data and never evaluates the text as executable code.
- String escape precedence: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- String escape precedence: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- String escape precedence: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- String escape precedence: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- String escape precedence: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how string escape precedence interacts with other sections.
- Document compatibility consequences before changing string escape precedence.

## Ambiguous markers

- Ambiguous markers: The parser records this concern as structured data and never evaluates the text as executable code.
- Ambiguous markers: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Ambiguous markers: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Ambiguous markers: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Ambiguous markers: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Ambiguous markers: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how ambiguous markers interacts with other sections.
- Document compatibility consequences before changing ambiguous markers.

## Diagnostic policy

- Diagnostic policy: The parser records this concern as structured data and never evaluates the text as executable code.
- Diagnostic policy: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Diagnostic policy: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Diagnostic policy: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Diagnostic policy: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Diagnostic policy: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how diagnostic policy interacts with other sections.
- Document compatibility consequences before changing diagnostic policy.

## Maintenance checklist

- Update the specification when behavior visible to users changes.
- Add a positive example and a negative regression case.
- Keep diagnostics actionable and preserve source locations.
- Verify generated output from the same fixture used by tests.
