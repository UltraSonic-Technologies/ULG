# Metadata Fields

Metadata identifies a language and its user-facing purpose. LANG_VER is the version of the defined language, not the ULG tool version.

This page is part of the ULG reference set. It describes behavior for language authors,
library users, and maintainers. Changes to externally visible syntax require
specification updates, migration notes, and regression coverage.

## LANG_NAME

- LANG_NAME: The parser records this concern as structured data and never evaluates the text as executable code.
- LANG_NAME: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- LANG_NAME: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- LANG_NAME: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- LANG_NAME: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- LANG_NAME: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how lang_name interacts with other sections.
- Document compatibility consequences before changing lang_name.

## LANG_DESC

- LANG_DESC: The parser records this concern as structured data and never evaluates the text as executable code.
- LANG_DESC: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- LANG_DESC: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- LANG_DESC: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- LANG_DESC: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- LANG_DESC: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how lang_desc interacts with other sections.
- Document compatibility consequences before changing lang_desc.

## LANG_VER

- LANG_VER: The parser records this concern as structured data and never evaluates the text as executable code.
- LANG_VER: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- LANG_VER: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- LANG_VER: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- LANG_VER: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- LANG_VER: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how lang_ver interacts with other sections.
- Document compatibility consequences before changing lang_ver.

## Quoted metadata

- Quoted metadata: The parser records this concern as structured data and never evaluates the text as executable code.
- Quoted metadata: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Quoted metadata: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Quoted metadata: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Quoted metadata: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Quoted metadata: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how quoted metadata interacts with other sections.
- Document compatibility consequences before changing quoted metadata.

## Version components

- Version components: The parser records this concern as structured data and never evaluates the text as executable code.
- Version components: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Version components: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Version components: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Version components: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Version components: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how version components interacts with other sections.
- Document compatibility consequences before changing version components.

## Empty metadata

- Empty metadata: The parser records this concern as structured data and never evaluates the text as executable code.
- Empty metadata: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Empty metadata: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Empty metadata: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Empty metadata: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Empty metadata: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how empty metadata interacts with other sections.
- Document compatibility consequences before changing empty metadata.

## Unicode text

- Unicode text: The parser records this concern as structured data and never evaluates the text as executable code.
- Unicode text: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unicode text: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unicode text: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unicode text: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unicode text: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how unicode text interacts with other sections.
- Document compatibility consequences before changing unicode text.

## Line breaks

- Line breaks: The parser records this concern as structured data and never evaluates the text as executable code.
- Line breaks: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Line breaks: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Line breaks: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Line breaks: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Line breaks: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how line breaks interacts with other sections.
- Document compatibility consequences before changing line breaks.

## Display names

- Display names: The parser records this concern as structured data and never evaluates the text as executable code.
- Display names: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Display names: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Display names: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Display names: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Display names: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how display names interacts with other sections.
- Document compatibility consequences before changing display names.

## Package identity

- Package identity: The parser records this concern as structured data and never evaluates the text as executable code.
- Package identity: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Package identity: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Package identity: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Package identity: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Package identity: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how package identity interacts with other sections.
- Document compatibility consequences before changing package identity.

## Tool version distinction

- Tool version distinction: The parser records this concern as structured data and never evaluates the text as executable code.
- Tool version distinction: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Tool version distinction: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Tool version distinction: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Tool version distinction: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Tool version distinction: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how tool version distinction interacts with other sections.
- Document compatibility consequences before changing tool version distinction.

## Validation errors

- Validation errors: The parser records this concern as structured data and never evaluates the text as executable code.
- Validation errors: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Validation errors: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Validation errors: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Validation errors: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Validation errors: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how validation errors interacts with other sections.
- Document compatibility consequences before changing validation errors.

## Generated constants

- Generated constants: The parser records this concern as structured data and never evaluates the text as executable code.
- Generated constants: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Generated constants: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Generated constants: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Generated constants: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Generated constants: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how generated constants interacts with other sections.
- Document compatibility consequences before changing generated constants.

## Metadata compatibility

- Metadata compatibility: The parser records this concern as structured data and never evaluates the text as executable code.
- Metadata compatibility: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Metadata compatibility: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Metadata compatibility: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Metadata compatibility: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Metadata compatibility: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how metadata compatibility interacts with other sections.
- Document compatibility consequences before changing metadata compatibility.

## Maintenance checklist

- Update the specification when behavior visible to users changes.
- Add a positive example and a negative regression case.
- Keep diagnostics actionable and preserve source locations.
- Verify generated output from the same fixture used by tests.
