# Editor Integration

Editors can load ULG definitions to power token categories, syntax colors, and language metadata. Completion and semantic diagnostics require later parser and language-service layers.

This design note is part of the ULG engineering handbook. It describes responsibilities
for contributors and consumers of the library. Public behavior should be specified
independently from implementation details so multiple backends can share one model.

## Loading definitions

- Loading definitions: The parser records this concern as structured data and never evaluates the text as executable code.
- Loading definitions: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Loading definitions: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Loading definitions: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Loading definitions: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Loading definitions: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating loading definitions in a normal language definition.
- Review error behavior and compatibility notes whenever loading definitions changes.

## Caching parsed models

- Caching parsed models: The parser records this concern as structured data and never evaluates the text as executable code.
- Caching parsed models: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Caching parsed models: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Caching parsed models: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Caching parsed models: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Caching parsed models: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating caching parsed models in a normal language definition.
- Review error behavior and compatibility notes whenever caching parsed models changes.

## File extension mapping

- File extension mapping: The parser records this concern as structured data and never evaluates the text as executable code.
- File extension mapping: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- File extension mapping: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- File extension mapping: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- File extension mapping: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- File extension mapping: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating file extension mapping in a normal language definition.
- Review error behavior and compatibility notes whenever file extension mapping changes.

## Token category mapping

- Token category mapping: The parser records this concern as structured data and never evaluates the text as executable code.
- Token category mapping: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Token category mapping: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Token category mapping: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Token category mapping: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Token category mapping: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating token category mapping in a normal language definition.
- Review error behavior and compatibility notes whenever token category mapping changes.

## Source span conversion

- Source span conversion: The parser records this concern as structured data and never evaluates the text as executable code.
- Source span conversion: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Source span conversion: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Source span conversion: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Source span conversion: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Source span conversion: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating source span conversion in a normal language definition.
- Review error behavior and compatibility notes whenever source span conversion changes.

## Syntax color themes

- Syntax color themes: The parser records this concern as structured data and never evaluates the text as executable code.
- Syntax color themes: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Syntax color themes: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Syntax color themes: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Syntax color themes: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Syntax color themes: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating syntax color themes in a normal language definition.
- Review error behavior and compatibility notes whenever syntax color themes changes.

## Comment toggles

- Comment toggles: The parser records this concern as structured data and never evaluates the text as executable code.
- Comment toggles: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Comment toggles: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Comment toggles: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Comment toggles: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Comment toggles: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating comment toggles in a normal language definition.
- Review error behavior and compatibility notes whenever comment toggles changes.

## Bracket matching

- Bracket matching: The parser records this concern as structured data and never evaluates the text as executable code.
- Bracket matching: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Bracket matching: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Bracket matching: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Bracket matching: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Bracket matching: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating bracket matching in a normal language definition.
- Review error behavior and compatibility notes whenever bracket matching changes.

## Operator completion

- Operator completion: The parser records this concern as structured data and never evaluates the text as executable code.
- Operator completion: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Operator completion: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Operator completion: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Operator completion: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Operator completion: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating operator completion in a normal language definition.
- Review error behavior and compatibility notes whenever operator completion changes.

## Keyword completion

- Keyword completion: The parser records this concern as structured data and never evaluates the text as executable code.
- Keyword completion: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Keyword completion: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Keyword completion: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Keyword completion: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Keyword completion: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating keyword completion in a normal language definition.
- Review error behavior and compatibility notes whenever keyword completion changes.

## Diagnostics presentation

- Diagnostics presentation: The parser records this concern as structured data and never evaluates the text as executable code.
- Diagnostics presentation: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Diagnostics presentation: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Diagnostics presentation: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Diagnostics presentation: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Diagnostics presentation: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating diagnostics presentation in a normal language definition.
- Review error behavior and compatibility notes whenever diagnostics presentation changes.

## Incremental tokenization

- Incremental tokenization: The parser records this concern as structured data and never evaluates the text as executable code.
- Incremental tokenization: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Incremental tokenization: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Incremental tokenization: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Incremental tokenization: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Incremental tokenization: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating incremental tokenization in a normal language definition.
- Review error behavior and compatibility notes whenever incremental tokenization changes.

## Workspace configuration

- Workspace configuration: The parser records this concern as structured data and never evaluates the text as executable code.
- Workspace configuration: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Workspace configuration: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Workspace configuration: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Workspace configuration: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Workspace configuration: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating workspace configuration in a normal language definition.
- Review error behavior and compatibility notes whenever workspace configuration changes.

## LSP roadmap

- LSP roadmap: The parser records this concern as structured data and never evaluates the text as executable code.
- LSP roadmap: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- LSP roadmap: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- LSP roadmap: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- LSP roadmap: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- LSP roadmap: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating lsp roadmap in a normal language definition.
- Review error behavior and compatibility notes whenever lsp roadmap changes.

## Review checklist

- Does the change have a documented public contract?
- Are edge cases covered by automated tests?
- Does the command-line interface expose useful errors?
- Is generated output reproducible?

## Related implementation

The implementation is organized under src so each stage can be tested independently. See the module-level documentation and integration tests before changing a shared contract.
