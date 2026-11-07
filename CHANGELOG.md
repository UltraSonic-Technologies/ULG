# Changelog

All notable changes to ULG are recorded in this file.

The project has not published a stable release. Dates and versions below
describe repository milestones rather than a promise of API stability.

## Unreleased

### Added

- Initial Rust library and command-line application.
- Concrete parser for the ULG document delimiters.
- Typed language definition model and dotted language versions.
- Semantic validation for metadata, duplicate entries, comments, operators,
  and punctuation expressions.
- Source-aware lexer for keywords, identifiers, numeric literals, strings,
  character literals, comments, operators, and punctuation.
- Rust source generation for language metadata and lexical lookup tables.
- Check, inspect, lex, and generate CLI commands.
- UltimateX, Minimal, and DataScript example definitions.
- Integration tests and CI workflow.

### Design notes

The first generator release is intentionally lexical. It does not claim to
generate complete context-free parsers, type checkers, compilers, runtimes,
or native binaries. Those capabilities require separate specifications and
backends and will be tracked as future milestones.
