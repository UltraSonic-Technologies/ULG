# ULG Language Specification

The specification defines the external contract for ULG files: concrete delimiters, declaration order, literal handling, and semantic meaning.

This page is part of the ULG reference set. It describes behavior for language authors,
library users, and maintainers. Changes to externally visible syntax require
specification updates, migration notes, and regression coverage.

## Document envelope

- Document envelope: The parser records this concern as structured data and never evaluates the text as executable code.
- Document envelope: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Document envelope: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Document envelope: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Document envelope: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Document envelope: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how document envelope interacts with other sections.
- Document compatibility consequences before changing document envelope.

## Initialization marker

- Initialization marker: The parser records this concern as structured data and never evaluates the text as executable code.
- Initialization marker: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Initialization marker: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Initialization marker: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Initialization marker: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Initialization marker: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how initialization marker interacts with other sections.
- Document compatibility consequences before changing initialization marker.

## Metadata tuples

- Metadata tuples: The parser records this concern as structured data and never evaluates the text as executable code.
- Metadata tuples: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Metadata tuples: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Metadata tuples: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Metadata tuples: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Metadata tuples: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how metadata tuples interacts with other sections.
- Document compatibility consequences before changing metadata tuples.

## Version atoms

- Version atoms: The parser records this concern as structured data and never evaluates the text as executable code.
- Version atoms: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Version atoms: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Version atoms: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Version atoms: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Version atoms: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how version atoms interacts with other sections.
- Document compatibility consequences before changing version atoms.

## Section container

- Section container: The parser records this concern as structured data and never evaluates the text as executable code.
- Section container: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Section container: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Section container: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Section container: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Section container: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how section container interacts with other sections.
- Document compatibility consequences before changing section container.

## Keyword declarations

- Keyword declarations: The parser records this concern as structured data and never evaluates the text as executable code.
- Keyword declarations: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Keyword declarations: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Keyword declarations: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Keyword declarations: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Keyword declarations: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how keyword declarations interacts with other sections.
- Document compatibility consequences before changing keyword declarations.

## Comment declarations

- Comment declarations: The parser records this concern as structured data and never evaluates the text as executable code.
- Comment declarations: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Comment declarations: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Comment declarations: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Comment declarations: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Comment declarations: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how comment declarations interacts with other sections.
- Document compatibility consequences before changing comment declarations.

## Operator declarations

- Operator declarations: The parser records this concern as structured data and never evaluates the text as executable code.
- Operator declarations: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Operator declarations: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Operator declarations: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Operator declarations: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Operator declarations: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how operator declarations interacts with other sections.
- Document compatibility consequences before changing operator declarations.

## Punctuation declarations

- Punctuation declarations: The parser records this concern as structured data and never evaluates the text as executable code.
- Punctuation declarations: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Punctuation declarations: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Punctuation declarations: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Punctuation declarations: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Punctuation declarations: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how punctuation declarations interacts with other sections.
- Document compatibility consequences before changing punctuation declarations.

## List delimiters

- List delimiters: The parser records this concern as structured data and never evaluates the text as executable code.
- List delimiters: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- List delimiters: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- List delimiters: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- List delimiters: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- List delimiters: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how list delimiters interacts with other sections.
- Document compatibility consequences before changing list delimiters.

## Match delimiters

- Match delimiters: The parser records this concern as structured data and never evaluates the text as executable code.
- Match delimiters: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Match delimiters: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Match delimiters: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Match delimiters: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Match delimiters: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how match delimiters interacts with other sections.
- Document compatibility consequences before changing match delimiters.

## String literals

- String literals: The parser records this concern as structured data and never evaluates the text as executable code.
- String literals: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- String literals: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- String literals: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- String literals: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- String literals: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how string literals interacts with other sections.
- Document compatibility consequences before changing string literals.

## Whitespace policy

- Whitespace policy: The parser records this concern as structured data and never evaluates the text as executable code.
- Whitespace policy: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Whitespace policy: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Whitespace policy: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Whitespace policy: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Whitespace policy: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how whitespace policy interacts with other sections.
- Document compatibility consequences before changing whitespace policy.

## End marker

- End marker: The parser records this concern as structured data and never evaluates the text as executable code.
- End marker: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- End marker: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- End marker: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- End marker: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- End marker: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how end marker interacts with other sections.
- Document compatibility consequences before changing end marker.

## Maintenance checklist

- Update the specification when behavior visible to users changes.
- Add a positive example and a negative regression case.
- Keep diagnostics actionable and preserve source locations.
- Verify generated output from the same fixture used by tests.
