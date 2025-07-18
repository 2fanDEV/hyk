# AGENTS.md

## Build, Lint, and Test Commands

### Build
To build the project, use:
```bash
cargo build --verbose
```

### Run Tests
To run all tests, use:
```bash
cargo test --verbose
```
To run a single test, specify the test name:
```bash
cargo test --verbose <test_name>
```

### Lint
Use `cargo clippy` for linting:
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Code Style Guidelines

### Imports
- Group imports logically (standard library, external crates, internal modules).
- Use `use` statements for specific items rather than importing entire modules.

### Formatting
- Follow `rustfmt` conventions. Run:
```bash
cargo fmt
```

### Types
- Prefer explicit types over `impl Trait` for clarity.
- Use `Option` and `Result` for nullable and error-prone values.

### Naming Conventions
- Use `snake_case` for variables and functions.
- Use `PascalCase` for structs, enums, and traits.
- Prefix private module items with `_` if unused.

### Error Handling
- Use `anyhow` for error propagation.
- Prefer `?` operator for error handling over `unwrap`.

### Additional Notes
- Ensure compatibility with dependencies listed in `Cargo.toml`.
- Follow guidelines in `.github/workflows/rust.yml` for CI/CD.
- Before you change or fix something, always ask for consent if the operator agrees with it
---

This file is designed to assist coding agents in maintaining consistency and quality across the repository.
