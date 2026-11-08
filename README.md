# ULG — Ultra Language Grammar

ULG is a declarative language-definition format and language-tooling
generator. Language authors describe lexical conventions in readable
.ulg files, validate those definitions, tokenize source files, and
generate reusable language tooling.

ULG is designed to be language-independent. Its reference implementation
is written in Rust, while generated Rust lexer modules can be embedded in
other Rust projects without linking the ULG library.

> ULG generates lexical tooling. A lexical definition is not, by itself,
> a complete parser, type checker, compiler, virtual machine, or runtime.

## Project goals

- Define programming-language metadata in a stable, readable format.
- Describe keywords, operators, comments, and punctuation.
- Validate definitions before code generation.
- Tokenize source with deterministic maximal-munch matching.
- Preserve byte spans and one-based line and column locations.
- Generate a self-contained Rust lexer from a .ulg definition.
- Keep generated language tables independent of the ULG runtime.
- Provide a CLI suitable for local development and CI.
- Keep the format independent of any one target programming language.
- Document extension points for future parser and compiler backends.

## Current capabilities

| Capability | Status |
| --- | --- |
| ULG document parser | Implemented |
| Metadata and version model | Implemented |
| Duplicate and consistency validation | Implemented |
| Keyword recognition | Implemented |
| Unicode-aware identifiers | Implemented |
| Integer and floating-point literals | Implemented |
| Quoted strings and character literals | Implemented |
| Line and block comments | Implemented |
| Longest-match operators | Implemented |
| Named punctuation and custom regex rules | Implemented |
| Source spans and lexical diagnostics | Implemented |
| Generic runtime lexer | Implemented |
| Standalone Rust lexer generation | Implemented |
| Complete context-free parser generation | Planned |
| Type checking and semantic compiler | Planned |
| Native executable generation | Planned |
| Language package registry | Planned |

## Workspace layout

- src/model/: typed language-definition data structures.
- src/parser/: concrete reader for ULG documents and source locations.
- src/diagnostics/: diagnostics, stable codes, and source spans.
- src/validate/: semantic validation of declarations.
- src/lexing/: generic runtime lexer and token stream.
- src/codegen/: deterministic Rust source generation and lexer template.
- src/cli/: check, inspect, lex, and generate commands.
- examples/: complete language definitions and source examples.
- docs/: specification, guides, design notes, and API explanations.
- tests/: integration and regression tests.
- .github/workflows/: CI and cross-platform release builds.

## ULG definition example

A language definition has a document header, metadata, and named sections:

~~~ulg
{
    [!INIT]
    (LANG_NAME "Example")
    (LANG_DESC "An example language")
    (LANG_VER 1.0.0)
    ({
        [KEYWORDS]:(!INIT)
        (LIST[
            "fn", "let", "return", "if", "else"
        ])
        (!END)

        [COMMENTS]:(!INIT)
        (LIST[
            "//", "/* */"
        ])
        (!END)

        [OPERATORS]:(!INIT)
        (LIST[
            "+", "-", "*", "/", "=", "==", "->"
        ])
        (!END)

        [PUNCTUATION]:(!INIT)
        (MATCH{
            PARENS -> "\\(\\)",
            BRACKETS -> "\\[\\]",
            BRACES -> "\\{\\}",
            SEMICOLON -> ";"
        })
        (!END)
    })
    [!END]
}
~~~

The parser preserves regular-expression escapes in quoted values. The exact
format rules are documented in docs/spec/specification.md.

## Install the ULG CLI

Install the stable Rust toolchain, then install ULG from a local checkout:

~~~sh
cargo install --path .
~~~

The executable is named ulg. Cargo places installed binaries in its
user-level Cargo bin directory. Ensure that directory is included in PATH.

Check the installation:

~~~sh
ulg --version
ulg --help
~~~

## Command-line usage

Validate a language definition:

~~~sh
ulg check examples/ultimatex/ultimatex.ulg
~~~

Inspect all metadata and lexical declarations:

~~~sh
ulg inspect examples/ultimatex/ultimatex.ulg
~~~

Tokenize an UltimateX source file:

~~~sh
ulg lex examples/ultimatex/ultimatex.ulg examples/ultimatex/main.ultix
~~~

Generate a standalone Rust lexer module:

~~~sh
ulg generate rust examples/ultimatex/ultimatex.ulg --out generated
~~~

The command creates generated/language_definition.rs. Choose a different
output directory or filename when regenerating. Existing files are not
silently overwritten.

You can customize the generated module label:

~~~sh
ulg generate rust examples/ultimatex/ultimatex.ulg --out generated --module ultimatex
~~~

During development, run the same commands through Cargo:

~~~sh
cargo run -- check examples/ultimatex/ultimatex.ulg
cargo run -- inspect examples/ultimatex/ultimatex.ulg
cargo run -- lex examples/ultimatex/ultimatex.ulg examples/ultimatex/main.ultix
cargo run -- generate rust examples/ultimatex/ultimatex.ulg --out target/generated
~~~

## Using a generated lexer

The generated Rust file is standalone with respect to ULG. It contains the
language metadata, keyword and operator tables, comment declarations,
punctuation rules, token types, source spans, diagnostics, and tokenizer.

Copy the generated module into a Rust application and add the regex crate:

~~~toml
[dependencies]
regex = "1.11"
~~~

Include the module and tokenize source:

~~~rust
mod ultimatex_lexer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = r#"
        fn main() {
            var message = "Hello from UltimateX";
            return message;
        }
    "#;

    let tokens = ultimatex_lexer::tokenize(source)?;

    for token in tokens {
        println!(
            "{} {} at {}:{}",
            token.kind,
            token.lexeme,
            token.span.line,
            token.span.column
        );
    }

    Ok(())
}
~~~

Generated lexers expose TokenKind, Token, Span, LexError, tokenize,
is_keyword, longest_operator, and format_tokens. Custom punctuation
patterns are compiled once and cached for subsequent tokenization calls.

## Rust library usage

Applications can use ULG as a library without shelling out to the CLI:

~~~rust
use std::fs;
use ulg::{parse, tokenize, validate};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string("language.ulg")?;
    let definition = parse(&text)?;
    validate(&definition)?;

    let tokens = tokenize(&definition, "fn main() {}")?;
    println!("{} tokens", tokens.len());
    Ok(())
}
~~~

The public API is documented in docs/reference/api.md.

## Development

Requirements:

- Rust stable toolchain.
- Cargo.
- A platform supported by Rust.

Useful commands:

~~~sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --workspace --release
~~~

See CONTRIBUTING.md before opening a pull request. The CI workflow runs
formatting, linting, tests, and example commands.

## Compatibility and stability

ULG syntax is versioned independently from generated language versions.
A definition's LANG_VER describes the language being defined; it does not
select the ULG file-format version. The format version is managed by the
toolchain and will be documented when compatibility guarantees are introduced.

The project is an evolving development release. Public APIs and generated
file layouts may change before a stable 1.0 release. Breaking changes should
be recorded in the changelog and migration notes.

## Security

ULG files are untrusted declarative input. The parser and validator do not
execute definition content. The CLI never invokes a shell using values from a
definition. Generated output is written only to the requested path, and the
current generator refuses to overwrite an existing file.

Custom punctuation expressions are compiled with the Rust regex crate.
Applications embedding ULG should still enforce reasonable input-size and
declaration-count limits when processing untrusted files.

Please report security issues privately to repository maintainers rather
than publishing exploit details in an issue.

## Licensing

ULG is distributed under the MIT License. See LICENSE.

## Contributing

Contributions are welcome. Useful areas include parser compatibility,
diagnostics, deterministic code generation, lexer edge cases, documentation,
platform testing, and additional examples. Include regression tests for
bug fixes and update the relevant documentation when behavior changes.

## Roadmap

1. Stabilize parsing and semantic validation of ULG definitions.
2. Maintain the generic lexer and source-position model.
3. Generate self-contained Rust lexer modules.
4. Specify declarative grammar rules and parser generation.
5. Add language-package manifests and installation support.
6. Add LSP integration for editor tooling.
7. Expand cross-platform CI and reproducible release artifacts.
8. Explore additional output backends after the language model is stable.

## Acknowledgements

ULG is implemented in Rust and uses the Rust ecosystem for parsing,
validation, testing, and command-line distribution. Individual dependencies
retain their own licenses; see dependency metadata for details.
