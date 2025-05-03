# Contributing to JamLiquor

Thank you for considering contributing to JamLiquor! This document outlines our cleanroom development process, code of conduct, and how to get started.

## Cleanroom Policy
- No direct copying from JAM reference clients or proprietary code.
- All implementations must be based on public specs or documentation.
- Commit history must be transparent and public.
- Major design decisions are documented in `docs/decisions/`.

## How to Contribute
1. Fork the repository and create a feature branch (`feature/`, `fix/`, `docs/`, `refactor/`).
2. Follow our [Cleanroom Policy](docs/src/CLEANROOM.md).
3. Ensure all code is well-documented with rustdoc comments (`///`).
4. Run all tests and ensure CI passes.
5. Submit a pull request with a clear description.

## Code Style
- Use `cargo fmt` for formatting.
- Use `cargo clippy` for linting.
- Document public APIs and modules.
- Write tests for new features.

## Commit Guidelines
- Commit messages: `type(scope): description` (e.g., `feat(core): add block validation`).
- Reference related issues or specs when appropriate.

## Code Review
- All PRs require at least two approving reviews.
- All CI checks must pass before merging.

## Security
- Report vulnerabilities via GitHub issues or direct contact.
- Follow security protocols outlined in [CLEANROOM.md](docs/src/CLEANROOM.md#security-protocols).

## License
By contributing, you agree that your contributions will be licensed under the GPL-3.0 license.

---

For questions, see [CLEANROOM.md](docs/src/CLEANROOM.md) or open an issue.
