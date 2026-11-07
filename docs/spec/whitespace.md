# Whitespace and Formatting

Whitespace separates ULG tokens but does not change section meaning. Authors can format lists and MATCH blocks across multiple lines.

This page is part of the ULG reference set. It describes behavior for language authors,
library users, and maintainers. Changes to externally visible syntax require
specification updates, migration notes, and regression coverage.

## Spaces

- Spaces: The parser records this concern as structured data and never evaluates the text as executable code.
- Spaces: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Spaces: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Spaces: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Spaces: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Spaces: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how spaces interacts with other sections.
- Document compatibility consequences before changing spaces.

## Tabs

- Tabs: The parser records this concern as structured data and never evaluates the text as executable code.
- Tabs: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Tabs: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Tabs: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Tabs: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Tabs: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how tabs interacts with other sections.
- Document compatibility consequences before changing tabs.

## Line feeds

- Line feeds: The parser records this concern as structured data and never evaluates the text as executable code.
- Line feeds: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Line feeds: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Line feeds: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Line feeds: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Line feeds: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how line feeds interacts with other sections.
- Document compatibility consequences before changing line feeds.

## Carriage returns

- Carriage returns: The parser records this concern as structured data and never evaluates the text as executable code.
- Carriage returns: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Carriage returns: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Carriage returns: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Carriage returns: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Carriage returns: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how carriage returns interacts with other sections.
- Document compatibility consequences before changing carriage returns.

## Indentation

- Indentation: The parser records this concern as structured data and never evaluates the text as executable code.
- Indentation: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Indentation: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Indentation: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Indentation: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Indentation: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how indentation interacts with other sections.
- Document compatibility consequences before changing indentation.

## Blank lines

- Blank lines: The parser records this concern as structured data and never evaluates the text as executable code.
- Blank lines: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Blank lines: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Blank lines: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Blank lines: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Blank lines: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how blank lines interacts with other sections.
- Document compatibility consequences before changing blank lines.

## Whitespace before delimiters

- Whitespace before delimiters: The parser records this concern as structured data and never evaluates the text as executable code.
- Whitespace before delimiters: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Whitespace before delimiters: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Whitespace before delimiters: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Whitespace before delimiters: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Whitespace before delimiters: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how whitespace before delimiters interacts with other sections.
- Document compatibility consequences before changing whitespace before delimiters.

## Whitespace inside lists

- Whitespace inside lists: The parser records this concern as structured data and never evaluates the text as executable code.
- Whitespace inside lists: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Whitespace inside lists: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Whitespace inside lists: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Whitespace inside lists: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Whitespace inside lists: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how whitespace inside lists interacts with other sections.
- Document compatibility consequences before changing whitespace inside lists.

## Whitespace around arrows

- Whitespace around arrows: The parser records this concern as structured data and never evaluates the text as executable code.
- Whitespace around arrows: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Whitespace around arrows: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Whitespace around arrows: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Whitespace around arrows: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Whitespace around arrows: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how whitespace around arrows interacts with other sections.
- Document compatibility consequences before changing whitespace around arrows.

## UTF-8 source

- UTF-8 source: The parser records this concern as structured data and never evaluates the text as executable code.
- UTF-8 source: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- UTF-8 source: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- UTF-8 source: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- UTF-8 source: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- UTF-8 source: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how utf-8 source interacts with other sections.
- Document compatibility consequences before changing utf-8 source.

## Column counting

- Column counting: The parser records this concern as structured data and never evaluates the text as executable code.
- Column counting: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Column counting: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Column counting: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Column counting: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Column counting: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how column counting interacts with other sections.
- Document compatibility consequences before changing column counting.

## Byte offsets

- Byte offsets: The parser records this concern as structured data and never evaluates the text as executable code.
- Byte offsets: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Byte offsets: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Byte offsets: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Byte offsets: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Byte offsets: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how byte offsets interacts with other sections.
- Document compatibility consequences before changing byte offsets.

## Formatting style

- Formatting style: The parser records this concern as structured data and never evaluates the text as executable code.
- Formatting style: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Formatting style: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Formatting style: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Formatting style: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Formatting style: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how formatting style interacts with other sections.
- Document compatibility consequences before changing formatting style.

## Canonical formatter direction

- Canonical formatter direction: The parser records this concern as structured data and never evaluates the text as executable code.
- Canonical formatter direction: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Canonical formatter direction: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Canonical formatter direction: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Canonical formatter direction: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Canonical formatter direction: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how canonical formatter direction interacts with other sections.
- Document compatibility consequences before changing canonical formatter direction.

## Maintenance checklist

- Update the specification when behavior visible to users changes.
- Add a positive example and a negative regression case.
- Keep diagnostics actionable and preserve source locations.
- Verify generated output from the same fixture used by tests.
