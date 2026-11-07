# Contributing to ULG

Thank you for considering a contribution to Ultra Language Grammar.

## Before opening a change

Read the format specification and the design documents under docs. Search
existing tests before adding a new behavior so that the contribution follows
the current compatibility rules.

A good change is focused. It identifies the user-visible behavior, explains
why the behavior belongs in ULG, and includes regression coverage.

## Development setup

Install the stable Rust toolchain with rustfmt and clippy. Clone the
repository and run cargo test before making changes. Use cargo fmt to format
Rust sources and cargo clippy to catch suspicious patterns.

## Code conventions

- Keep parser errors location-aware.
- Avoid panics for malformed ULG input.
- Do not execute content from ULG definitions.
- Prefer explicit types over stringly typed internal state.
- Preserve authored spelling when formatting is not part of the change.
- Keep generated output deterministic.
- Add tests for Unicode, empty lists, escaped strings, and malformed input.
- Document public APIs and non-obvious compatibility decisions.

## Pull requests

A pull request should describe the problem, the approach, compatibility
implications, and how the change was tested. Include examples for format
changes. Do not combine unrelated refactoring and behavior changes unless
there is a clear reason.

The CI checks formatting, compilation, tests, and documentation. Resolve
failures before requesting review. If a check cannot run locally, explain the
environment limitation and provide the expected verification steps.

## Format changes

The ULG syntax is user-authored configuration. A change to delimiters,
escaping, metadata, or section semantics can affect every language package.
Propose format changes with a specification update and acceptance tests.

Do not silently reinterpret existing declarations. Prefer an explicit new
field or a versioned syntax rule, and document migration behavior.

## Reporting bugs

Include the smallest ULG definition that demonstrates the issue, the exact
command used, the output, and the operating system. Remove private project
data before publishing an example.

## Security

Do not include executable scripts or shell fragments in language definitions.
The parser must treat ULG values as data. Report security issues privately
to repository maintainers rather than disclosing an exploit in a public issue.
