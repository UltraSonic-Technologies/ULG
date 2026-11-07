# Security Reference

ULG input is data, not code. Parser, validator, lexer, and generator must avoid executing input and should operate with bounded resources.

This handbook page is part of the ULG documentation set. It is written for language
authors, library users, and maintainers. Keep public behavior separate from
implementation details so future backends can reuse the same language model.

## Untrusted definitions

- Untrusted definitions: The parser records this concern as structured data and never evaluates the text as executable code.
- Untrusted definitions: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Untrusted definitions: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Untrusted definitions: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Untrusted definitions: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Untrusted definitions: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing untrusted definitions in a valid language definition.
- Document migration behavior before changing untrusted definitions.

## No evaluation

- No evaluation: The parser records this concern as structured data and never evaluates the text as executable code.
- No evaluation: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- No evaluation: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- No evaluation: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- No evaluation: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- No evaluation: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing no evaluation in a valid language definition.
- Document migration behavior before changing no evaluation.

## No shell execution

- No shell execution: The parser records this concern as structured data and never evaluates the text as executable code.
- No shell execution: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- No shell execution: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- No shell execution: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- No shell execution: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- No shell execution: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing no shell execution in a valid language definition.
- Document migration behavior before changing no shell execution.

## Regex resource use

- Regex resource use: The parser records this concern as structured data and never evaluates the text as executable code.
- Regex resource use: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Regex resource use: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Regex resource use: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Regex resource use: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Regex resource use: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing regex resource use in a valid language definition.
- Document migration behavior before changing regex resource use.

## Large list limits

- Large list limits: The parser records this concern as structured data and never evaluates the text as executable code.
- Large list limits: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Large list limits: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Large list limits: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Large list limits: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Large list limits: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing large list limits in a valid language definition.
- Document migration behavior before changing large list limits.

## Large source limits

- Large source limits: The parser records this concern as structured data and never evaluates the text as executable code.
- Large source limits: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Large source limits: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Large source limits: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Large source limits: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Large source limits: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing large source limits in a valid language definition.
- Document migration behavior before changing large source limits.

## Filesystem boundaries

- Filesystem boundaries: The parser records this concern as structured data and never evaluates the text as executable code.
- Filesystem boundaries: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Filesystem boundaries: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Filesystem boundaries: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Filesystem boundaries: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Filesystem boundaries: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing filesystem boundaries in a valid language definition.
- Document migration behavior before changing filesystem boundaries.

## Output directory safety

- Output directory safety: The parser records this concern as structured data and never evaluates the text as executable code.
- Output directory safety: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Output directory safety: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Output directory safety: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Output directory safety: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Output directory safety: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing output directory safety in a valid language definition.
- Document migration behavior before changing output directory safety.

## Dependency review

- Dependency review: The parser records this concern as structured data and never evaluates the text as executable code.
- Dependency review: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Dependency review: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Dependency review: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Dependency review: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Dependency review: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing dependency review in a valid language definition.
- Document migration behavior before changing dependency review.

## Error data exposure

- Error data exposure: The parser records this concern as structured data and never evaluates the text as executable code.
- Error data exposure: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Error data exposure: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Error data exposure: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Error data exposure: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Error data exposure: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing error data exposure in a valid language definition.
- Document migration behavior before changing error data exposure.

## CI permissions

- CI permissions: The parser records this concern as structured data and never evaluates the text as executable code.
- CI permissions: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- CI permissions: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- CI permissions: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- CI permissions: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- CI permissions: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing ci permissions in a valid language definition.
- Document migration behavior before changing ci permissions.

## Private disclosure

- Private disclosure: The parser records this concern as structured data and never evaluates the text as executable code.
- Private disclosure: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Private disclosure: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Private disclosure: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Private disclosure: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Private disclosure: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing private disclosure in a valid language definition.
- Document migration behavior before changing private disclosure.

## Threat assumptions

- Threat assumptions: The parser records this concern as structured data and never evaluates the text as executable code.
- Threat assumptions: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Threat assumptions: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Threat assumptions: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Threat assumptions: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Threat assumptions: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing threat assumptions in a valid language definition.
- Document migration behavior before changing threat assumptions.

## Hardening roadmap

- Hardening roadmap: The parser records this concern as structured data and never evaluates the text as executable code.
- Hardening roadmap: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Hardening roadmap: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Hardening roadmap: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Hardening roadmap: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Hardening roadmap: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing hardening roadmap in a valid language definition.
- Document migration behavior before changing hardening roadmap.

## Operational checklist

- Validate every definition before invoking code generation.
- Preserve source locations in every diagnostic path.
- Keep generated output deterministic and safe to overwrite only by explicit request.
- Run the full test suite after changing a shared contract.
