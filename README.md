# ULG — Ultra Language Grammar

ULG is a declarative language-definition format and compiler toolkit.
It is designed to let language authors describe lexical conventions and
syntax metadata in readable `.ulg` files, then validate those definitions
and generate reusable language tooling.

> ULG is a language-tooling project. It does not claim that a grammar
> definition alone implements a complete compiler or runtime.

## Project goals

- Define programming-language metadata in a stable, readable format.
- Describe keywords, operators, comments, punctuation, literals, and lexical rules.
- Validate definitions before code generation.
- Provide deterministic tokenization with source locations.
- Generate Rust lexer artifacts from language definitions.
- Keep the definition format independent of any one programming language.
- Offer a command-line interface suitable for local and CI workflows.
- Provide a documented extension path for future parser backends.

## Status

The repository is being established as the reference implementation of ULG.
The initial implementation focuses on the specification parser, semantic
validation, lexical model, Rust code generation, and CLI. Parser generation,
type systems, compiler backends, and language runtimes are separate milestones.

## Workspace layout

- `crates/ulg-syntax`: parsing the ULG definition format.
- `crates/ulg-model`: typed language-definition data structures.
- `crates/ulg-diagnostics`: diagnostics and source spans.
- `crates/ulg-validate`: semantic validation of definitions.
- `crates/ulg-lexer`: runtime lexer and token stream.
- `crates/ulg-codegen`: generated Rust artifacts.
- `crates/ulg-cli`: the `ulg` command-line application.
- `examples/`: complete language definitions and source examples.
- `docs/`: specification, guides, design notes, and API explanations.
- `tests/`: integration and regression tests.

## ULG definition example

A language definition has a document header, metadata, and named sections:

```ulg
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
            BRACES -> "\\{\\}"
        })
        (!END)
    })
    [!END]
}
```

The exact grammar and supported fields are documented in
[`docs/specification.md`](docs/specification.md).

## Command-line usage

After installing the CLI:

```sh
ulg check examples/ultimatex/ultimatex.ulg
ulg inspect examples/ultimatex/ultimatex.ulg
ulg lex examples/ultimatex/ultimatex.ulg examples/ultimatex/main.ultix
ulg generate rust examples/ultimatex/ultimatex.ulg --out generated
```

During development, run commands through Cargo:

```sh
cargo run -p ulg-cli -- check examples/ultimatex/ultimatex.ulg
cargo run -p ulg-cli -- inspect examples/ultimatex/ultimatex.ulg
cargo run -p ulg-cli -- generate rust examples/ultimatex/ultimatex.ulg --out generated
```

The CLI reports errors with file, line, and column information. Generated
artifacts are deterministic so they can be checked into source control or
recreated in a build pipeline.

## Rust library usage

The workspace crates are intended to be usable independently. Applications
can parse a definition, validate it, inspect the typed model, tokenize source,
or invoke a code-generation backend without shelling out to the CLI.

See [`docs/library-api.md`](docs/library-api.md) for the public API design
and examples.

## Development

Requirements:

- Rust stable toolchain.
- Cargo.
- A platform supported by Rust.

Useful commands:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --workspace --release
```

See [`CONTRIBUTING.md`](CONTRIBUTING.md) before opening a pull request.
The project uses continuous integration to run formatting, linting, tests,
and documentation checks.

## Compatibility and stability

ULG syntax is versioned independently from generated language versions.
A definition's `LANG_VER` describes the language being defined; it does not
select the ULG file-format version. The format version is managed by the ULG
toolchain and will be documented when compatibility guarantees are introduced.

The first release is an evolving development release. Public APIs and generated
file layouts may change before a stable 1.0 release. Breaking changes should
be recorded in the changelog and migration notes.

## Security

ULG files are treated as untrusted input. Parsers and validators must enforce
reasonable input limits, avoid executing definition content, and never invoke
shell commands based on values from a definition. Code generation writes only
to the requested output directory and must not silently overwrite unrelated
files.

Please report security issues privately to the repository maintainers rather
than publishing exploit details in an issue.

## Licensing

ULG is distributed under the MIT License. See [`LICENSE`](LICENSE).

## Contributing

Contributions are welcome. Useful areas include grammar compatibility,
diagnostics, deterministic code generation, lexer edge cases, documentation,
platform testing, and additional examples. Please include regression tests for
bug fixes and update the relevant documentation when behavior changes.

## Roadmap

1. Stabilize parsing and semantic validation of ULG definitions.
2. Complete the generic lexer and source-position model.
3. Generate a self-contained Rust lexer module.
4. Add a declarative parser grammar and parser-generation backend.
5. Add language-package manifests and installation support.
6. Add LSP integration for editor tooling.
7. Expand cross-platform CI and reproducible release artifacts.

## Acknowledgements

ULG is implemented in Rust and uses the Rust ecosystem for parsing, validation,
testing, and command-line distribution. Individual dependencies retain their
own licenses; see the dependency metadata for details.
