# Creating a Language

Language authors define identity and lexical conventions first, then expand a definition as syntax decisions become stable.

This design note is part of the ULG engineering handbook. It describes responsibilities
for contributors and consumers of the library. Public behavior should be specified
independently from implementation details so multiple backends can share one model.

## Choose a language name

- Choose a language name: The parser records this concern as structured data and never evaluates the text as executable code.
- Choose a language name: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Choose a language name: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Choose a language name: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Choose a language name: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Choose a language name: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating choose a language name in a normal language definition.
- Review error behavior and compatibility notes whenever choose a language name changes.

## Write a description

- Write a description: The parser records this concern as structured data and never evaluates the text as executable code.
- Write a description: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Write a description: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Write a description: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Write a description: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Write a description: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating write a description in a normal language definition.
- Review error behavior and compatibility notes whenever write a description changes.

## Select a version

- Select a version: The parser records this concern as structured data and never evaluates the text as executable code.
- Select a version: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Select a version: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Select a version: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Select a version: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Select a version: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating select a version in a normal language definition.
- Review error behavior and compatibility notes whenever select a version changes.

## List reserved words

- List reserved words: The parser records this concern as structured data and never evaluates the text as executable code.
- List reserved words: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- List reserved words: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- List reserved words: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- List reserved words: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- List reserved words: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating list reserved words in a normal language definition.
- Review error behavior and compatibility notes whenever list reserved words changes.

## Define comment markers

- Define comment markers: The parser records this concern as structured data and never evaluates the text as executable code.
- Define comment markers: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Define comment markers: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Define comment markers: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Define comment markers: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Define comment markers: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating define comment markers in a normal language definition.
- Review error behavior and compatibility notes whenever define comment markers changes.

## Choose operator spellings

- Choose operator spellings: The parser records this concern as structured data and never evaluates the text as executable code.
- Choose operator spellings: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Choose operator spellings: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Choose operator spellings: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Choose operator spellings: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Choose operator spellings: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating choose operator spellings in a normal language definition.
- Review error behavior and compatibility notes whenever choose operator spellings changes.

## Name punctuation groups

- Name punctuation groups: The parser records this concern as structured data and never evaluates the text as executable code.
- Name punctuation groups: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Name punctuation groups: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Name punctuation groups: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Name punctuation groups: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Name punctuation groups: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating name punctuation groups in a normal language definition.
- Review error behavior and compatibility notes whenever name punctuation groups changes.

## Add literal rules later

- Add literal rules later: The parser records this concern as structured data and never evaluates the text as executable code.
- Add literal rules later: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Add literal rules later: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Add literal rules later: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Add literal rules later: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Add literal rules later: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating add literal rules later in a normal language definition.
- Review error behavior and compatibility notes whenever add literal rules later changes.

## Run structural parsing

- Run structural parsing: The parser records this concern as structured data and never evaluates the text as executable code.
- Run structural parsing: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Run structural parsing: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Run structural parsing: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Run structural parsing: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Run structural parsing: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating run structural parsing in a normal language definition.
- Review error behavior and compatibility notes whenever run structural parsing changes.

## Run semantic validation

- Run semantic validation: The parser records this concern as structured data and never evaluates the text as executable code.
- Run semantic validation: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Run semantic validation: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Run semantic validation: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Run semantic validation: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Run semantic validation: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating run semantic validation in a normal language definition.
- Review error behavior and compatibility notes whenever run semantic validation changes.

## Lex representative code

- Lex representative code: The parser records this concern as structured data and never evaluates the text as executable code.
- Lex representative code: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Lex representative code: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Lex representative code: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Lex representative code: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Lex representative code: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating lex representative code in a normal language definition.
- Review error behavior and compatibility notes whenever lex representative code changes.

## Check ambiguous prefixes

- Check ambiguous prefixes: The parser records this concern as structured data and never evaluates the text as executable code.
- Check ambiguous prefixes: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Check ambiguous prefixes: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Check ambiguous prefixes: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Check ambiguous prefixes: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Check ambiguous prefixes: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating check ambiguous prefixes in a normal language definition.
- Review error behavior and compatibility notes whenever check ambiguous prefixes changes.

## Commit example fixtures

- Commit example fixtures: The parser records this concern as structured data and never evaluates the text as executable code.
- Commit example fixtures: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Commit example fixtures: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Commit example fixtures: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Commit example fixtures: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Commit example fixtures: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating commit example fixtures in a normal language definition.
- Review error behavior and compatibility notes whenever commit example fixtures changes.

## Plan grammar evolution

- Plan grammar evolution: The parser records this concern as structured data and never evaluates the text as executable code.
- Plan grammar evolution: Semantic validation checks consistency before the definition is passed to lexical analysis or generation.
- Plan grammar evolution: The lexer preserves original spelling and source byte ranges while consuming the validated declaration.
- Plan grammar evolution: The CLI exposes this behavior through explicit commands so build scripts can reproduce the same result.
- Plan grammar evolution: Generated Rust output must be deterministic for identical definitions and toolchain versions.
- Plan grammar evolution: Regression tests should cover valid input, malformed input, boundary conditions, and Unicode text.
- Keep a focused fixture demonstrating plan grammar evolution in a normal language definition.
- Review error behavior and compatibility notes whenever plan grammar evolution changes.

## Review checklist

- Does the change have a documented public contract?
- Are edge cases covered by automated tests?
- Does the command-line interface expose useful errors?
- Is generated output reproducible?

## Related implementation

The implementation is organized under src so each stage can be tested independently. See the module-level documentation and integration tests before changing a shared contract.
