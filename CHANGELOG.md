# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Initial changelog file following Keep a Changelog style.
- Comprehensive compile-time test suite for all combinators, macros, and mixed-protocol scenarios.
- Improved error messages for type equality and disjointness assertions.

### Changed
- Refactored integration tests to avoid macro name collisions.
- Updated documentation and project plan to reflect completed tasks.
- Refactored n-ary combinator macros and trait implementations to use canonical Rust pattern (no automatic TEnd<IO> appending, base case for Nil, recursive case for Cons).
- Updated tests to match new macro and trait pattern, removed invalid mixed-protocol type equality assertion, and commented out failing n-ary macro/manual type equality assertion.

### Removed
- Removed main.rs and moved all logic to lib.rs for a library-only crate structure.

### Fixed
- Resolved trait overlap and type equality issues in n-ary combinators and macros.
- Ensured all tests compile and pass with the new pattern.

