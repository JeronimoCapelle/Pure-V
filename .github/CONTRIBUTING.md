# Contributing

This repository uses standard stable Rust with zero custom build wrappers required.

## Running Checks Locally

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --doc
```

If you have cargo-nextest and cargo-deny installed, you can also run:
Bash

```sh
cargo nextest run --workspace
cargo deny check
```
