# GitHub Copilot Usage & Contribution Instructions

## Overview

This project uses [GitHub Copilot](https://github.com/features/copilot) to assist with code, documentation, and protocol design. Copilot is a tool to help, not replace, human review and design. Please follow these guidelines to ensure high-quality, maintainable, and safe contributions.

---

## General Guidelines

- **Copilot suggestions are a starting point.**  
  Always review, edit, and test generated code or documentation before committing.
- **Prioritize clarity and correctness.**  
  If Copilot’s suggestion is unclear or incorrect, revise it or write your own.
- **Document your intent.**  
  Add comments or docstrings to clarify non-obvious code, especially for protocol logic and type-level programming.
- **Prefer explicitness over cleverness.**  
  Readability and maintainability are more important than brevity or “tricks.”

---

## Markdown & Documentation

- All documentation must pass [markdownlint](https://github.com/DavidAnson/markdownlint) checks.
- Use line wrapping at 80-100 characters for readability.
- Ensure code examples in docs are up to date and compile (doctest where possible).
- Use clear section headings and bullet points for structure.

---

## Code Style

- Follow Rust’s standard formatting (`rustfmt`).
- Use idiomatic Rust patterns and avoid unnecessary complexity.
- All code must pass `cargo clippy` and `cargo test` before merging.

---

## Protocol & Type-Level Design

- When using Copilot for type-level or macro-heavy code, double-check trait bounds, recursion, and type safety.
- Add compile-time assertions (e.g., `assert_type_eq!`, `assert_disjoint!`) for protocol invariants.
- Document any non-trivial type-level logic.

---

## Commit & PR Workflow

- Summarize Copilot’s involvement in your PR description if it generated significant code or documentation.
- All PRs must be reviewed by a human before merging.
- Use draft PRs for work-in-progress and to get early feedback.

---

## Security & Safety

- Never accept Copilot suggestions that include secrets, credentials, or unsafe code.
- Review all dependencies and generated code for potential vulnerabilities.

---

## Required Code Test & Verification Steps

Before submitting code or documentation, you must run and pass all of the following commands locally:

- `cargo check` — Ensure the code compiles without errors.
- `cargo build` — Build the project to catch any build-time issues.
- `cargo test` — Run all tests to verify correctness.
- `cargo fmt --all -- --check` — Check that all code is properly formatted.
- `cargo clippy` — Run the linter to catch common mistakes and improve code quality.

All code must pass these checks before a pull request is submitted or merged. These steps are also enforced in CI.

---

## Feedback & Improvements

- If Copilot makes repeated mistakes or low-quality suggestions, document them in issues or PRs for future reference.
- Suggest improvements to these instructions as the project evolves.

---

*Last updated: 2025-05-12*
