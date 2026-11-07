# Punctuation MATCH Rules

Punctuation is declared as named patterns in MATCH blocks. Conventional names describe paired delimiters; custom names use regular-expression patterns.

This page is part of the ULG reference set. It describes behavior for language authors,
library users, and maintainers. Changes to externally visible syntax require
specification updates, migration notes, and regression coverage.

## MATCH block

- MATCH block: The parser records this concern as structured data and never evaluates the text as executable code.
- MATCH block: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- MATCH block: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- MATCH block: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- MATCH block: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- MATCH block: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how match block interacts with other sections.
- Document compatibility consequences before changing match block.

## Rule identifiers

- Rule identifiers: The parser records this concern as structured data and never evaluates the text as executable code.
- Rule identifiers: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Rule identifiers: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Rule identifiers: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Rule identifiers: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Rule identifiers: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how rule identifiers interacts with other sections.
- Document compatibility consequences before changing rule identifiers.

## Arrow separator

- Arrow separator: The parser records this concern as structured data and never evaluates the text as executable code.
- Arrow separator: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Arrow separator: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Arrow separator: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Arrow separator: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Arrow separator: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how arrow separator interacts with other sections.
- Document compatibility consequences before changing arrow separator.

## Quoted patterns

- Quoted patterns: The parser records this concern as structured data and never evaluates the text as executable code.
- Quoted patterns: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Quoted patterns: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Quoted patterns: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Quoted patterns: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Quoted patterns: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how quoted patterns interacts with other sections.
- Document compatibility consequences before changing quoted patterns.

## PARENS convention

- PARENS convention: The parser records this concern as structured data and never evaluates the text as executable code.
- PARENS convention: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- PARENS convention: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- PARENS convention: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- PARENS convention: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- PARENS convention: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how parens convention interacts with other sections.
- Document compatibility consequences before changing parens convention.

## BRACKETS convention

- BRACKETS convention: The parser records this concern as structured data and never evaluates the text as executable code.
- BRACKETS convention: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- BRACKETS convention: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- BRACKETS convention: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- BRACKETS convention: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- BRACKETS convention: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how brackets convention interacts with other sections.
- Document compatibility consequences before changing brackets convention.

## BRACES convention

- BRACES convention: The parser records this concern as structured data and never evaluates the text as executable code.
- BRACES convention: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- BRACES convention: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- BRACES convention: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- BRACES convention: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- BRACES convention: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how braces convention interacts with other sections.
- Document compatibility consequences before changing braces convention.

## Custom patterns

- Custom patterns: The parser records this concern as structured data and never evaluates the text as executable code.
- Custom patterns: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Custom patterns: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Custom patterns: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Custom patterns: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Custom patterns: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how custom patterns interacts with other sections.
- Document compatibility consequences before changing custom patterns.

## Regex compilation

- Regex compilation: The parser records this concern as structured data and never evaluates the text as executable code.
- Regex compilation: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Regex compilation: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Regex compilation: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Regex compilation: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Regex compilation: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how regex compilation interacts with other sections.
- Document compatibility consequences before changing regex compilation.

## Empty regex matches

- Empty regex matches: The parser records this concern as structured data and never evaluates the text as executable code.
- Empty regex matches: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Empty regex matches: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Empty regex matches: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Empty regex matches: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Empty regex matches: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how empty regex matches interacts with other sections.
- Document compatibility consequences before changing empty regex matches.

## Longest punctuation match

- Longest punctuation match: The parser records this concern as structured data and never evaluates the text as executable code.
- Longest punctuation match: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Longest punctuation match: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Longest punctuation match: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Longest punctuation match: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Longest punctuation match: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how longest punctuation match interacts with other sections.
- Document compatibility consequences before changing longest punctuation match.

## Rule names in tokens

- Rule names in tokens: The parser records this concern as structured data and never evaluates the text as executable code.
- Rule names in tokens: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Rule names in tokens: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Rule names in tokens: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Rule names in tokens: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Rule names in tokens: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how rule names in tokens interacts with other sections.
- Document compatibility consequences before changing rule names in tokens.

## Duplicate names

- Duplicate names: The parser records this concern as structured data and never evaluates the text as executable code.
- Duplicate names: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Duplicate names: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Duplicate names: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Duplicate names: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Duplicate names: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how duplicate names interacts with other sections.
- Document compatibility consequences before changing duplicate names.

## Generated matcher tables

- Generated matcher tables: The parser records this concern as structured data and never evaluates the text as executable code.
- Generated matcher tables: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Generated matcher tables: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Generated matcher tables: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Generated matcher tables: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Generated matcher tables: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how generated matcher tables interacts with other sections.
- Document compatibility consequences before changing generated matcher tables.

## Maintenance checklist

- Update the specification when behavior visible to users changes.
- Add a positive example and a negative regression case.
- Keep diagnostics actionable and preserve source locations.
- Verify generated output from the same fixture used by tests.
