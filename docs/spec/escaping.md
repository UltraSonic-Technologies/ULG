# String Escaping

ULG string values use quoted syntax. Common escapes are decoded, while unknown escapes are preserved so regular-expression patterns retain backslashes.

This page is part of the ULG reference set. It describes behavior for language authors,
library users, and maintainers. Changes to externally visible syntax require
specification updates, migration notes, and regression coverage.

## Quote delimiter

- Quote delimiter: The parser records this concern as structured data and never evaluates the text as executable code.
- Quote delimiter: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Quote delimiter: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Quote delimiter: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Quote delimiter: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Quote delimiter: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how quote delimiter interacts with other sections.
- Document compatibility consequences before changing quote delimiter.

## Escaped quote

- Escaped quote: The parser records this concern as structured data and never evaluates the text as executable code.
- Escaped quote: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Escaped quote: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Escaped quote: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Escaped quote: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Escaped quote: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how escaped quote interacts with other sections.
- Document compatibility consequences before changing escaped quote.

## Escaped backslash

- Escaped backslash: The parser records this concern as structured data and never evaluates the text as executable code.
- Escaped backslash: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Escaped backslash: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Escaped backslash: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Escaped backslash: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Escaped backslash: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how escaped backslash interacts with other sections.
- Document compatibility consequences before changing escaped backslash.

## Newline escape

- Newline escape: The parser records this concern as structured data and never evaluates the text as executable code.
- Newline escape: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Newline escape: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Newline escape: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Newline escape: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Newline escape: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how newline escape interacts with other sections.
- Document compatibility consequences before changing newline escape.

## Carriage return escape

- Carriage return escape: The parser records this concern as structured data and never evaluates the text as executable code.
- Carriage return escape: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Carriage return escape: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Carriage return escape: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Carriage return escape: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Carriage return escape: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how carriage return escape interacts with other sections.
- Document compatibility consequences before changing carriage return escape.

## Tab escape

- Tab escape: The parser records this concern as structured data and never evaluates the text as executable code.
- Tab escape: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Tab escape: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Tab escape: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Tab escape: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Tab escape: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how tab escape interacts with other sections.
- Document compatibility consequences before changing tab escape.

## Unknown escapes

- Unknown escapes: The parser records this concern as structured data and never evaluates the text as executable code.
- Unknown escapes: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unknown escapes: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unknown escapes: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unknown escapes: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unknown escapes: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how unknown escapes interacts with other sections.
- Document compatibility consequences before changing unknown escapes.

## Regex backslashes

- Regex backslashes: The parser records this concern as structured data and never evaluates the text as executable code.
- Regex backslashes: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Regex backslashes: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Regex backslashes: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Regex backslashes: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Regex backslashes: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how regex backslashes interacts with other sections.
- Document compatibility consequences before changing regex backslashes.

## Unterminated strings

- Unterminated strings: The parser records this concern as structured data and never evaluates the text as executable code.
- Unterminated strings: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unterminated strings: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unterminated strings: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unterminated strings: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unterminated strings: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how unterminated strings interacts with other sections.
- Document compatibility consequences before changing unterminated strings.

## Newline in strings

- Newline in strings: The parser records this concern as structured data and never evaluates the text as executable code.
- Newline in strings: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Newline in strings: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Newline in strings: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Newline in strings: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Newline in strings: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how newline in strings interacts with other sections.
- Document compatibility consequences before changing newline in strings.

## Unicode characters

- Unicode characters: The parser records this concern as structured data and never evaluates the text as executable code.
- Unicode characters: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unicode characters: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unicode characters: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unicode characters: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unicode characters: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how unicode characters interacts with other sections.
- Document compatibility consequences before changing unicode characters.

## Source locations

- Source locations: The parser records this concern as structured data and never evaluates the text as executable code.
- Source locations: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Source locations: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Source locations: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Source locations: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Source locations: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how source locations interacts with other sections.
- Document compatibility consequences before changing source locations.

## Generated Rust literals

- Generated Rust literals: The parser records this concern as structured data and never evaluates the text as executable code.
- Generated Rust literals: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Generated Rust literals: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Generated Rust literals: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Generated Rust literals: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Generated Rust literals: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how generated rust literals interacts with other sections.
- Document compatibility consequences before changing generated rust literals.

## Round-trip expectations

- Round-trip expectations: The parser records this concern as structured data and never evaluates the text as executable code.
- Round-trip expectations: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Round-trip expectations: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Round-trip expectations: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Round-trip expectations: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Round-trip expectations: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how round-trip expectations interacts with other sections.
- Document compatibility consequences before changing round-trip expectations.

## Maintenance checklist

- Update the specification when behavior visible to users changes.
- Add a positive example and a negative regression case.
- Keep diagnostics actionable and preserve source locations.
- Verify generated output from the same fixture used by tests.
