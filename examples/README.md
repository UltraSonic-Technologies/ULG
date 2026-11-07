# ULG examples

The examples directory contains complete definition files and source snippets
used to demonstrate the ULG format and the reference lexer.

## UltimateX

The UltimateX folder carries the language definition discussed by the ULG
maintainers. It includes metadata, a representative keyword table, line and
block comments, compound operators, and named punctuation patterns.

Run the following commands from the repository root:

    cargo run -- check examples/ultimatex/ultimatex.ulg
    cargo run -- inspect examples/ultimatex/ultimatex.ulg
    cargo run -- lex examples/ultimatex/ultimatex.ulg examples/ultimatex/main.ultix
    cargo run -- generate rust examples/ultimatex/ultimatex.ulg --out generated

The source snippet is deliberately small. It demonstrates token categories
without claiming that the current lexical generator implements a complete
UltimateX grammar or type system.

## Minimal

The minimal definition is useful when debugging parser behavior. It keeps
the number of declarations low and makes it easy to isolate errors in lists,
metadata, comment markers, operators, and delimiter pairs.

## Data-oriented

The data-oriented example demonstrates that ULG is not tied to systems
programming syntax. A language may choose different keywords and operators.
The lexer consumes the supplied declarations rather than a built-in language
name switch.

## Adding an example

A useful example should include a matching ULG file and, where practical, a
source file. Keep names and file extensions consistent. Add one test when the
example demonstrates a parser or lexer behavior that should not regress.

Do not treat an example source file as proof of language completeness. The
current backend provides lexical analysis and generated Rust declaration
tables; parser and compiler backends are future milestones.
