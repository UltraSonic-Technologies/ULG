# Library API Reference

The Rust API exposes the same processing stages as the CLI. Applications can parse and validate once, then reuse the immutable definition for lexical work.

This handbook page is part of the ULG documentation set. It is written for language
authors, library users, and maintainers. Keep public behavior separate from
implementation details so future backends can reuse the same language model.

## parse function

- parse function: The parser records this concern as structured data and never evaluates the text as executable code.
- parse function: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- parse function: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- parse function: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- parse function: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- parse function: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing parse function in a valid language definition.
- Document migration behavior before changing parse function.

## parse_file helper

- parse_file helper: The parser records this concern as structured data and never evaluates the text as executable code.
- parse_file helper: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- parse_file helper: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- parse_file helper: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- parse_file helper: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- parse_file helper: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing parse_file helper in a valid language definition.
- Document migration behavior before changing parse_file helper.

## LanguageDefinition

- LanguageDefinition: The parser records this concern as structured data and never evaluates the text as executable code.
- LanguageDefinition: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- LanguageDefinition: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- LanguageDefinition: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- LanguageDefinition: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- LanguageDefinition: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing languagedefinition in a valid language definition.
- Document migration behavior before changing languagedefinition.

## LanguageVersion

- LanguageVersion: The parser records this concern as structured data and never evaluates the text as executable code.
- LanguageVersion: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- LanguageVersion: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- LanguageVersion: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- LanguageVersion: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- LanguageVersion: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing languageversion in a valid language definition.
- Document migration behavior before changing languageversion.

## CommentDefinition

- CommentDefinition: The parser records this concern as structured data and never evaluates the text as executable code.
- CommentDefinition: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- CommentDefinition: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- CommentDefinition: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- CommentDefinition: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- CommentDefinition: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing commentdefinition in a valid language definition.
- Document migration behavior before changing commentdefinition.

## PunctuationDefinition

- PunctuationDefinition: The parser records this concern as structured data and never evaluates the text as executable code.
- PunctuationDefinition: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- PunctuationDefinition: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- PunctuationDefinition: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- PunctuationDefinition: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- PunctuationDefinition: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing punctuationdefinition in a valid language definition.
- Document migration behavior before changing punctuationdefinition.

## validate function

- validate function: The parser records this concern as structured data and never evaluates the text as executable code.
- validate function: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- validate function: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- validate function: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- validate function: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- validate function: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing validate function in a valid language definition.
- Document migration behavior before changing validate function.

## ValidationError

- ValidationError: The parser records this concern as structured data and never evaluates the text as executable code.
- ValidationError: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- ValidationError: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- ValidationError: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- ValidationError: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- ValidationError: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing validationerror in a valid language definition.
- Document migration behavior before changing validationerror.

## tokenize function

- tokenize function: The parser records this concern as structured data and never evaluates the text as executable code.
- tokenize function: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- tokenize function: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- tokenize function: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- tokenize function: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- tokenize function: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing tokenize function in a valid language definition.
- Document migration behavior before changing tokenize function.

## Token

- Token: The parser records this concern as structured data and never evaluates the text as executable code.
- Token: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Token: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Token: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Token: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Token: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing token in a valid language definition.
- Document migration behavior before changing token.

## TokenKind

- TokenKind: The parser records this concern as structured data and never evaluates the text as executable code.
- TokenKind: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- TokenKind: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- TokenKind: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- TokenKind: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- TokenKind: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing tokenkind in a valid language definition.
- Document migration behavior before changing tokenkind.

## LexError

- LexError: The parser records this concern as structured data and never evaluates the text as executable code.
- LexError: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- LexError: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- LexError: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- LexError: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- LexError: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing lexerror in a valid language definition.
- Document migration behavior before changing lexerror.

## generate_rust function

- generate_rust function: The parser records this concern as structured data and never evaluates the text as executable code.
- generate_rust function: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- generate_rust function: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- generate_rust function: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- generate_rust function: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- generate_rust function: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing generate_rust function in a valid language definition.
- Document migration behavior before changing generate_rust function.

## Public API compatibility

- Public API compatibility: The parser records this concern as structured data and never evaluates the text as executable code.
- Public API compatibility: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Public API compatibility: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Public API compatibility: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Public API compatibility: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Public API compatibility: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture showing public api compatibility in a valid language definition.
- Document migration behavior before changing public api compatibility.

## Operational checklist

- Validate every definition before invoking code generation.
- Preserve source locations in every diagnostic path.
- Keep generated output deterministic and safe to overwrite only by explicit request.
- Run the full test suite after changing a shared contract.
