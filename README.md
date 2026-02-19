# Tic Tac Toe (Rust)

A simple Tic Tac Toe project in Rust with two run modes:
- `window` (default) — GUI built with `macroquad`
- `cli` — terminal mode

## Run

```bash
cargo run
```

CLI mode:

```bash
cargo run -- cli
```

AI selection (`minimax` is the default):

```bash
cargo run -- cli ml
cargo run -- window ml
```

## Code Quality and CI

This repository includes a GitHub Actions workflow: [.github/workflows/ci.yml](.github/workflows/ci.yml).

The pipeline runs:
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --locked`

## Pre-commit (Local)

The repository includes a shared hook: [.githooks/pre-commit](.githooks/pre-commit).

Enable it locally once per clone:

```bash
git config core.hooksPath .githooks
```

From that point, `git commit` is blocked if these checks fail:
- `rustfmt`
- `clippy` (with `-D warnings`)
