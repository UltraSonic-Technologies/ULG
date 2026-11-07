# Document Structure

A ULG document is one complete definition enclosed by braces. The root contains initialization, metadata, a section container, and a final marker.

This page is part of the ULG reference set. It describes behavior for language authors,
library users, and maintainers. Changes to externally visible syntax require
specification updates, migration notes, and regression coverage.

## Root opening brace

- Root opening brace: The parser records this concern as structured data and never evaluates the text as executable code.
- Root opening brace: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Root opening brace: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Root opening brace: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Root opening brace: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Root opening brace: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how root opening brace interacts with other sections.
- Document compatibility consequences before changing root opening brace.

## Initialization marker

- Initialization marker: The parser records this concern as structured data and never evaluates the text as executable code.
- Initialization marker: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Initialization marker: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Initialization marker: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Initialization marker: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Initialization marker: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how initialization marker interacts with other sections.
- Document compatibility consequences before changing initialization marker.

## Metadata order

- Metadata order: The parser records this concern as structured data and never evaluates the text as executable code.
- Metadata order: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Metadata order: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Metadata order: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Metadata order: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Metadata order: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how metadata order interacts with other sections.
- Document compatibility consequences before changing metadata order.

## Container opening

- Container opening: The parser records this concern as structured data and never evaluates the text as executable code.
- Container opening: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Container opening: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Container opening: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Container opening: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Container opening: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how container opening interacts with other sections.
- Document compatibility consequences before changing container opening.

## Section sequence

- Section sequence: The parser records this concern as structured data and never evaluates the text as executable code.
- Section sequence: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Section sequence: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Section sequence: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Section sequence: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Section sequence: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how section sequence interacts with other sections.
- Document compatibility consequences before changing section sequence.

## Container closing

- Container closing: The parser records this concern as structured data and never evaluates the text as executable code.
- Container closing: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Container closing: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Container closing: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Container closing: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Container closing: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how container closing interacts with other sections.
- Document compatibility consequences before changing container closing.

## End marker

- End marker: The parser records this concern as structured data and never evaluates the text as executable code.
- End marker: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- End marker: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- End marker: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- End marker: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- End marker: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how end marker interacts with other sections.
- Document compatibility consequences before changing end marker.

## Trailing input

- Trailing input: The parser records this concern as structured data and never evaluates the text as executable code.
- Trailing input: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Trailing input: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Trailing input: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Trailing input: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Trailing input: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how trailing input interacts with other sections.
- Document compatibility consequences before changing trailing input.

## Empty documents

- Empty documents: The parser records this concern as structured data and never evaluates the text as executable code.
- Empty documents: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Empty documents: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Empty documents: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Empty documents: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Empty documents: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how empty documents interacts with other sections.
- Document compatibility consequences before changing empty documents.

## Nested delimiters

- Nested delimiters: The parser records this concern as structured data and never evaluates the text as executable code.
- Nested delimiters: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Nested delimiters: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Nested delimiters: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Nested delimiters: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Nested delimiters: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how nested delimiters interacts with other sections.
- Document compatibility consequences before changing nested delimiters.

## Whitespace between tokens

- Whitespace between tokens: The parser records this concern as structured data and never evaluates the text as executable code.
- Whitespace between tokens: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Whitespace between tokens: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Whitespace between tokens: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Whitespace between tokens: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Whitespace between tokens: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how whitespace between tokens interacts with other sections.
- Document compatibility consequences before changing whitespace between tokens.

## Duplicate sections

- Duplicate sections: The parser records this concern as structured data and never evaluates the text as executable code.
- Duplicate sections: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Duplicate sections: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Duplicate sections: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Duplicate sections: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Duplicate sections: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how duplicate sections interacts with other sections.
- Document compatibility consequences before changing duplicate sections.

## Unknown sections

- Unknown sections: The parser records this concern as structured data and never evaluates the text as executable code.
- Unknown sections: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unknown sections: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unknown sections: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unknown sections: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unknown sections: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how unknown sections interacts with other sections.
- Document compatibility consequences before changing unknown sections.

## Complete parse requirement

- Complete parse requirement: The parser records this concern as structured data and never evaluates the text as executable code.
- Complete parse requirement: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Complete parse requirement: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Complete parse requirement: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Complete parse requirement: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Complete parse requirement: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how complete parse requirement interacts with other sections.
- Document compatibility consequences before changing complete parse requirement.

## Maintenance checklist

- Update the specification when behavior visible to users changes.
- Add a positive example and a negative regression case.
- Keep diagnostics actionable and preserve source locations.
- Verify generated output from the same fixture used by tests.
