# Copilot Instructions

## Language Rules
- Always write documentation in English.
- Always write code comments in English.
- Keep identifiers (function names, variable names, type names) in English unless there is a strict project requirement to do otherwise.

## Rust Guidelines
- Prefer idiomatic Rust and standard library patterns over custom or verbose implementations.
- Prefer `Result`/`Option` for recoverable flows and avoid `panic!` in normal runtime logic.
- Keep ownership and borrowing explicit; avoid unnecessary cloning unless it improves correctness or readability.
- Prefer iterator adapters (`map`, `filter`, `find`, `collect`) when they improve clarity.
- Implement `Default` for types that expose `new()` with obvious default values.

## Quality Gates
- Ensure generated code passes `cargo fmt --all -- --check`.
- Ensure generated code passes `cargo clippy --all-targets --all-features -- -D warnings`.
- Ensure generated code passes `cargo test --locked` when tests are present.

## Testing Rules
- Add or update tests for behavior changes, especially argument parsing, public API contracts, and game logic.
- Keep tests deterministic and focused; avoid flaky timing-dependent tests.
- Prefer unit tests close to modules and integration tests in the `tests/` directory for public API behavior.

## Change Scope
- Make minimal, focused edits that solve the root problem.
- Avoid unrelated refactors unless required to make the change compile, lint cleanly, or test correctly.

## Scope
These rules apply to all generated or edited files in this repository.
