# Security policy

## Supported releases

Security fixes are applied to the current development branch. Until a stable
release series exists, users should assume that compatibility and security
guarantees are still being established.

## Reporting a vulnerability

Do not publish exploit details in a public issue. Contact the repository
maintainers through GitHub's private vulnerability reporting feature when it
is available for this repository. Include the affected commit, operating
system, command, and a minimal reproduction that does not expose secrets.

## Input handling

ULG files are untrusted declarative data. The parser must not execute source
expressions, invoke a shell, load dynamic libraries, or evaluate arbitrary
code embedded in a definition. Code generation must only write to an
explicitly selected output path.

## Resource limits

Applications embedding ULG should set reasonable limits on definition size,
number of declarations, string length, and source files passed to the lexer.
Large regular expressions should be reviewed before use with untrusted input.
The current implementation uses the regex crate, which avoids several
backtracking hazards, but does not remove the need for input-size limits.

## Filesystem behavior

The CLI reads only the paths supplied by the user. Generated files are
written to the requested output directory. Callers should avoid running the
tool with elevated privileges when processing untrusted definitions.

## Dependency maintenance

Keep dependencies updated and review security advisories before releases.
Lock production dependency versions when building release artifacts.
