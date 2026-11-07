# Embedding the ULG Library

Applications can embed parser, validator, and lexer stages without invoking the CLI. Convert the public token model into application-specific structures at the boundary.

This handbook page is part of the ULG documentation set. It is written for language
authors, library users, and maintainers. Keep public behavior separate from
implementation details so future backends can reuse the same language model.

## Library dependency

- Library dependency: The parser records this concern as structured data and never evaluates the text as executable code.
- Library dependency: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Library dependency: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Library dependency: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Library dependency: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Library dependency: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing library dependency in a valid language definition.
- Document migration behavior before changing library dependency.

## Parsing source text

- Parsing source text: The parser records this concern as structured data and never evaluates the text as executable code.
- Parsing source text: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Parsing source text: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Parsing source text: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Parsing source text: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Parsing source text: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing parsing source text in a valid language definition.
- Document migration behavior before changing parsing source text.

## Validation calls

- Validation calls: The parser records this concern as structured data and never evaluates the text as executable code.
- Validation calls: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Validation calls: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Validation calls: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Validation calls: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Validation calls: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing validation calls in a valid language definition.
- Document migration behavior before changing validation calls.

## Error propagation

- Error propagation: The parser records this concern as structured data and never evaluates the text as executable code.
- Error propagation: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Error propagation: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Error propagation: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Error propagation: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Error propagation: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing error propagation in a valid language definition.
- Document migration behavior before changing error propagation.

## Language caching

- Language caching: The parser records this concern as structured data and never evaluates the text as executable code.
- Language caching: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Language caching: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Language caching: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Language caching: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Language caching: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing language caching in a valid language definition.
- Document migration behavior before changing language caching.

## Lexer construction

- Lexer construction: The parser records this concern as structured data and never evaluates the text as executable code.
- Lexer construction: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Lexer construction: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Lexer construction: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Lexer construction: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Lexer construction: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing lexer construction in a valid language definition.
- Document migration behavior before changing lexer construction.

## Token conversion

- Token conversion: The parser records this concern as structured data and never evaluates the text as executable code.
- Token conversion: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Token conversion: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Token conversion: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Token conversion: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Token conversion: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing token conversion in a valid language definition.
- Document migration behavior before changing token conversion.

## Source ownership

- Source ownership: The parser records this concern as structured data and never evaluates the text as executable code.
- Source ownership: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Source ownership: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Source ownership: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Source ownership: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Source ownership: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing source ownership in a valid language definition.
- Document migration behavior before changing source ownership.

## Threading considerations

- Threading considerations: The parser records this concern as structured data and never evaluates the text as executable code.
- Threading considerations: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Threading considerations: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Threading considerations: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Threading considerations: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Threading considerations: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing threading considerations in a valid language definition.
- Document migration behavior before changing threading considerations.

## Memory lifetime

- Memory lifetime: The parser records this concern as structured data and never evaluates the text as executable code.
- Memory lifetime: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Memory lifetime: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Memory lifetime: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Memory lifetime: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Memory lifetime: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing memory lifetime in a valid language definition.
- Document migration behavior before changing memory lifetime.

## Input limits

- Input limits: The parser records this concern as structured data and never evaluates the text as executable code.
- Input limits: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Input limits: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Input limits: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Input limits: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Input limits: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing input limits in a valid language definition.
- Document migration behavior before changing input limits.

## Generated artifacts

- Generated artifacts: The parser records this concern as structured data and never evaluates the text as executable code.
- Generated artifacts: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Generated artifacts: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Generated artifacts: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Generated artifacts: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Generated artifacts: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing generated artifacts in a valid language definition.
- Document migration behavior before changing generated artifacts.

## Public API stability

- Public API stability: The parser records this concern as structured data and never evaluates the text as executable code.
- Public API stability: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Public API stability: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Public API stability: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Public API stability: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Public API stability: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing public api stability in a valid language definition.
- Document migration behavior before changing public api stability.

## Application integration

- Application integration: The parser records this concern as structured data and never evaluates the text as executable code.
- Application integration: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Application integration: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Application integration: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Application integration: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Application integration: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing application integration in a valid language definition.
- Document migration behavior before changing application integration.

## Operational checklist

- Validate every definition before invoking code generation.
- Preserve source locations in every diagnostic path.
- Keep generated output deterministic and safe to overwrite only by explicit request.
- Run the full test suite after changing a shared contract.
