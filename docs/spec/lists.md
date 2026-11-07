# LIST Sections

LIST blocks provide ordered string values for lexical declarations. The parser preserves source order while validators and scanners derive lookup tables.

This page is part of the ULG reference set. It describes behavior for language authors,
library users, and maintainers. Changes to externally visible syntax require
specification updates, migration notes, and regression coverage.

## LIST opening token

- LIST opening token: The parser records this concern as structured data and never evaluates the text as executable code.
- LIST opening token: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- LIST opening token: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- LIST opening token: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- LIST opening token: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- LIST opening token: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how list opening token interacts with other sections.
- Document compatibility consequences before changing list opening token.

## LIST closing token

- LIST closing token: The parser records this concern as structured data and never evaluates the text as executable code.
- LIST closing token: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- LIST closing token: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- LIST closing token: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- LIST closing token: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- LIST closing token: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how list closing token interacts with other sections.
- Document compatibility consequences before changing list closing token.

## String entries

- String entries: The parser records this concern as structured data and never evaluates the text as executable code.
- String entries: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- String entries: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- String entries: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- String entries: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- String entries: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how string entries interacts with other sections.
- Document compatibility consequences before changing string entries.

## Comma separators

- Comma separators: The parser records this concern as structured data and never evaluates the text as executable code.
- Comma separators: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Comma separators: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Comma separators: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Comma separators: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Comma separators: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how comma separators interacts with other sections.
- Document compatibility consequences before changing comma separators.

## Trailing comma

- Trailing comma: The parser records this concern as structured data and never evaluates the text as executable code.
- Trailing comma: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Trailing comma: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Trailing comma: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Trailing comma: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Trailing comma: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how trailing comma interacts with other sections.
- Document compatibility consequences before changing trailing comma.

## Empty list

- Empty list: The parser records this concern as structured data and never evaluates the text as executable code.
- Empty list: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Empty list: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Empty list: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Empty list: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Empty list: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how empty list interacts with other sections.
- Document compatibility consequences before changing empty list.

## Whitespace handling

- Whitespace handling: The parser records this concern as structured data and never evaluates the text as executable code.
- Whitespace handling: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Whitespace handling: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Whitespace handling: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Whitespace handling: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Whitespace handling: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how whitespace handling interacts with other sections.
- Document compatibility consequences before changing whitespace handling.

## Duplicate entries

- Duplicate entries: The parser records this concern as structured data and never evaluates the text as executable code.
- Duplicate entries: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Duplicate entries: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Duplicate entries: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Duplicate entries: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Duplicate entries: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how duplicate entries interacts with other sections.
- Document compatibility consequences before changing duplicate entries.

## Empty strings

- Empty strings: The parser records this concern as structured data and never evaluates the text as executable code.
- Empty strings: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Empty strings: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Empty strings: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Empty strings: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Empty strings: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how empty strings interacts with other sections.
- Document compatibility consequences before changing empty strings.

## Keyword values

- Keyword values: The parser records this concern as structured data and never evaluates the text as executable code.
- Keyword values: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Keyword values: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Keyword values: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Keyword values: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Keyword values: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how keyword values interacts with other sections.
- Document compatibility consequences before changing keyword values.

## Comment values

- Comment values: The parser records this concern as structured data and never evaluates the text as executable code.
- Comment values: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Comment values: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Comment values: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Comment values: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Comment values: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how comment values interacts with other sections.
- Document compatibility consequences before changing comment values.

## Operator values

- Operator values: The parser records this concern as structured data and never evaluates the text as executable code.
- Operator values: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Operator values: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Operator values: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Operator values: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Operator values: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how operator values interacts with other sections.
- Document compatibility consequences before changing operator values.

## Source order

- Source order: The parser records this concern as structured data and never evaluates the text as executable code.
- Source order: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Source order: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Source order: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Source order: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Source order: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how source order interacts with other sections.
- Document compatibility consequences before changing source order.

## Generated arrays

- Generated arrays: The parser records this concern as structured data and never evaluates the text as executable code.
- Generated arrays: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Generated arrays: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Generated arrays: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Generated arrays: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Generated arrays: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Maintain a compact example showing how generated arrays interacts with other sections.
- Document compatibility consequences before changing generated arrays.

## Maintenance checklist

- Update the specification when behavior visible to users changes.
- Add a positive example and a negative regression case.
- Keep diagnostics actionable and preserve source locations.
- Verify generated output from the same fixture used by tests.
