# Versioning Rules

Language versions are dotted unsigned integer components. ULG tool versioning is separate from LANG_VER and follows its own compatibility policy.

This page is part of the ULG reference set. It describes behavior for language authors,
library users, and maintainers. Changes to externally visible syntax require
specification updates, migration notes, and regression coverage.

## Version atom

- Version atom: The parser records this concern as structured data and never evaluates the text as executable code.
- Version atom: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Version atom: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Version atom: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Version atom: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Version atom: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how version atom interacts with other sections.
- Document compatibility consequences before changing version atom.

## Major component

- Major component: The parser records this concern as structured data and never evaluates the text as executable code.
- Major component: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Major component: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Major component: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Major component: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Major component: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how major component interacts with other sections.
- Document compatibility consequences before changing major component.

## Minor component

- Minor component: The parser records this concern as structured data and never evaluates the text as executable code.
- Minor component: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Minor component: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Minor component: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Minor component: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Minor component: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how minor component interacts with other sections.
- Document compatibility consequences before changing minor component.

## Patch component

- Patch component: The parser records this concern as structured data and never evaluates the text as executable code.
- Patch component: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Patch component: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Patch component: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Patch component: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Patch component: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how patch component interacts with other sections.
- Document compatibility consequences before changing patch component.

## Additional components

- Additional components: The parser records this concern as structured data and never evaluates the text as executable code.
- Additional components: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Additional components: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Additional components: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Additional components: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Additional components: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how additional components interacts with other sections.
- Document compatibility consequences before changing additional components.

## Unsigned values

- Unsigned values: The parser records this concern as structured data and never evaluates the text as executable code.
- Unsigned values: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Unsigned values: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Unsigned values: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Unsigned values: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Unsigned values: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how unsigned values interacts with other sections.
- Document compatibility consequences before changing unsigned values.

## Empty components

- Empty components: The parser records this concern as structured data and never evaluates the text as executable code.
- Empty components: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Empty components: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Empty components: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Empty components: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Empty components: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how empty components interacts with other sections.
- Document compatibility consequences before changing empty components.

## Leading zeros

- Leading zeros: The parser records this concern as structured data and never evaluates the text as executable code.
- Leading zeros: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Leading zeros: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Leading zeros: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Leading zeros: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Leading zeros: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how leading zeros interacts with other sections.
- Document compatibility consequences before changing leading zeros.

## Numeric overflow

- Numeric overflow: The parser records this concern as structured data and never evaluates the text as executable code.
- Numeric overflow: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Numeric overflow: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Numeric overflow: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Numeric overflow: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Numeric overflow: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how numeric overflow interacts with other sections.
- Document compatibility consequences before changing numeric overflow.

## String formatting

- String formatting: The parser records this concern as structured data and never evaluates the text as executable code.
- String formatting: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- String formatting: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- String formatting: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- String formatting: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- String formatting: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how string formatting interacts with other sections.
- Document compatibility consequences before changing string formatting.

## Version comparison

- Version comparison: The parser records this concern as structured data and never evaluates the text as executable code.
- Version comparison: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Version comparison: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Version comparison: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Version comparison: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Version comparison: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how version comparison interacts with other sections.
- Document compatibility consequences before changing version comparison.

## Language release process

- Language release process: The parser records this concern as structured data and never evaluates the text as executable code.
- Language release process: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Language release process: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Language release process: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Language release process: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Language release process: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how language release process interacts with other sections.
- Document compatibility consequences before changing language release process.

## Format version distinction

- Format version distinction: The parser records this concern as structured data and never evaluates the text as executable code.
- Format version distinction: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Format version distinction: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Format version distinction: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Format version distinction: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Format version distinction: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how format version distinction interacts with other sections.
- Document compatibility consequences before changing format version distinction.

## Migration policy

- Migration policy: The parser records this concern as structured data and never evaluates the text as executable code.
- Migration policy: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Migration policy: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Migration policy: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Migration policy: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Migration policy: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how migration policy interacts with other sections.
- Document compatibility consequences before changing migration policy.

## Maintenance checklist

- Update the specification when behavior visible to users changes.
- Add a positive example and a negative regression case.
- Keep diagnostics actionable and preserve source locations.
- Verify generated output from the same fixture used by tests.
