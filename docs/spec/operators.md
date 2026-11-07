# Operator Declarations

Operators use maximal-munch matching. Longer spellings take precedence over shorter prefixes so compound operators remain distinct.

This page is part of the ULG reference set. It describes behavior for language authors,
library users, and maintainers. Changes to externally visible syntax require
specification updates, migration notes, and regression coverage.

## Operator spelling

- Operator spelling: The parser records this concern as structured data and never evaluates the text as executable code.
- Operator spelling: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Operator spelling: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Operator spelling: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Operator spelling: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Operator spelling: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how operator spelling interacts with other sections.
- Document compatibility consequences before changing operator spelling.

## Longest-first matching

- Longest-first matching: The parser records this concern as structured data and never evaluates the text as executable code.
- Longest-first matching: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Longest-first matching: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Longest-first matching: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Longest-first matching: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Longest-first matching: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how longest-first matching interacts with other sections.
- Document compatibility consequences before changing longest-first matching.

## Prefix conflicts

- Prefix conflicts: The parser records this concern as structured data and never evaluates the text as executable code.
- Prefix conflicts: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Prefix conflicts: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Prefix conflicts: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Prefix conflicts: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Prefix conflicts: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how prefix conflicts interacts with other sections.
- Document compatibility consequences before changing prefix conflicts.

## Assignment operators

- Assignment operators: The parser records this concern as structured data and never evaluates the text as executable code.
- Assignment operators: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Assignment operators: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Assignment operators: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Assignment operators: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Assignment operators: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how assignment operators interacts with other sections.
- Document compatibility consequences before changing assignment operators.

## Comparison operators

- Comparison operators: The parser records this concern as structured data and never evaluates the text as executable code.
- Comparison operators: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Comparison operators: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Comparison operators: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Comparison operators: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Comparison operators: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how comparison operators interacts with other sections.
- Document compatibility consequences before changing comparison operators.

## Arithmetic operators

- Arithmetic operators: The parser records this concern as structured data and never evaluates the text as executable code.
- Arithmetic operators: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Arithmetic operators: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Arithmetic operators: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Arithmetic operators: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Arithmetic operators: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how arithmetic operators interacts with other sections.
- Document compatibility consequences before changing arithmetic operators.

## Logical operators

- Logical operators: The parser records this concern as structured data and never evaluates the text as executable code.
- Logical operators: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Logical operators: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Logical operators: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Logical operators: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Logical operators: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how logical operators interacts with other sections.
- Document compatibility consequences before changing logical operators.

## Arrow operators

- Arrow operators: The parser records this concern as structured data and never evaluates the text as executable code.
- Arrow operators: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Arrow operators: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Arrow operators: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Arrow operators: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Arrow operators: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how arrow operators interacts with other sections.
- Document compatibility consequences before changing arrow operators.

## Single-character operators

- Single-character operators: The parser records this concern as structured data and never evaluates the text as executable code.
- Single-character operators: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Single-character operators: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Single-character operators: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Single-character operators: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Single-character operators: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how single-character operators interacts with other sections.
- Document compatibility consequences before changing single-character operators.

## Duplicate operators

- Duplicate operators: The parser records this concern as structured data and never evaluates the text as executable code.
- Duplicate operators: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Duplicate operators: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Duplicate operators: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Duplicate operators: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Duplicate operators: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how duplicate operators interacts with other sections.
- Document compatibility consequences before changing duplicate operators.

## Empty operators

- Empty operators: The parser records this concern as structured data and never evaluates the text as executable code.
- Empty operators: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Empty operators: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Empty operators: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Empty operators: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Empty operators: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how empty operators interacts with other sections.
- Document compatibility consequences before changing empty operators.

## Unexpected symbols

- Unexpected symbols: The parser records this concern as structured data and never evaluates the text as executable code.
- Unexpected symbols: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unexpected symbols: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unexpected symbols: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unexpected symbols: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unexpected symbols: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how unexpected symbols interacts with other sections.
- Document compatibility consequences before changing unexpected symbols.

## Token spans

- Token spans: The parser records this concern as structured data and never evaluates the text as executable code.
- Token spans: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Token spans: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Token spans: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Token spans: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Token spans: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how token spans interacts with other sections.
- Document compatibility consequences before changing token spans.

## Generated operator tables

- Generated operator tables: The parser records this concern as structured data and never evaluates the text as executable code.
- Generated operator tables: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Generated operator tables: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Generated operator tables: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Generated operator tables: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Generated operator tables: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how generated operator tables interacts with other sections.
- Document compatibility consequences before changing generated operator tables.

## Maintenance checklist

- Update the specification when behavior visible to users changes.
- Add a positive example and a negative regression case.
- Keep diagnostics actionable and preserve source locations.
- Verify generated output from the same fixture used by tests.
