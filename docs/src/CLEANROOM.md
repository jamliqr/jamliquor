# 🧪 Cleanroom Policy for JamLiquor

[← Back to Index](./index.md)

## Table of Contents
- [Overview](#overview)
- [What Cleanroom Means in JamLiquor](#-what-cleanroom-means-in-jamliquor)
- [Cleanroom Principles](#-cleanroom-principles)
- [Development Setup](#-development-setup)
- [Code Review Process](#-code-review-process)
- [Release Protocols](#-release-protocols)
- [Documentation Trail](#-documentation-trail)
- [License & Legal](#-license--legal)
- [Questions or Concerns](#-questions-or-concerns)

## Overview
JamLiquor is a **cleanroom Rust implementation** of the JAM Protocol. It has been built from scratch to ensure:

- Legal and ethical separation from existing implementations
- Trust in code provenance and conformance

> **Note:** JAM Prize entrant status is explicitly out of current scope — see [`spec.md`](../../spec.md) §1/§2. Cleanroom provenance is maintained on its own merits (auditability, legal separation), not as prize-track preparation. The historical "Allowed materials" and "JAM Prize assessments" references below predate that decision and are kept as an accurate account of how the codebase was actually built.

This document outlines how we ensure our cleanroom standards.

---

## ✅ What Cleanroom Means in JamLiquor
A cleanroom implementation ensures:
- **No direct copying** of JAM source code (e.g. from ParityTech or Web3 Foundation repositories)
- **Independent reimplementation** based on publicly available specifications
- **Transparent commit history**, publicly timestamped
- **Documented references** and architecture reasoning

---

## 🔐 Cleanroom Principles
1. **No source reuse** from:
   - Parity's JAM reference clients
   - Polkadot node source code
   - Any proprietary JAM-related implementation

2. **Allowed materials**:
   - JAM Graypaper (https://github.com/gavofyork/graypaper)
   - JAM Prize specs (https://hackmd.io/@polkadot/jamprize)
   - JAM-related public blog posts (e.g. kianenigma, parity forums)
   - Official documentation and diagrams (HackMD, GitHub wikis)

3. **Codebase boundaries**:
   - Core components (`jam-core`: codec, crypto, safrole, trie — see [`spec.md`](../../spec.md) §6) are written independently
   - Any reused crates are open-source, third-party, and audited (e.g. `blake2b_simd`, `ed25519-dalek`) — no PQC or AI crates; those are retired from scope, see [Research Notes](./research-notes.md)

4. **Contributors declare compliance**:
   - All contributors agree to cleanroom principles
   - PRs involving potentially derivative logic are flagged and reviewed

---

## 🛠️ Development Setup
1. **Environment Setup**
   - Install Rust toolchain (1.75.0+)
   - Configure development tools:
     ```bash
     rustup component add clippy rustfmt
     cargo install cargo-audit cargo-tarpaulin
     ```
   - Set up git hooks for pre-commit checks

2. **Code Quality Tools**
   - Rustfmt for formatting
   - Clippy for linting
   - cargo-audit for security
   - cargo-tarpaulin for coverage

3. **Development Workflow**
   - Branch naming: `feature/`, `fix/`, `docs/`, `refactor/`
   - Commit message format: `type(scope): description`
   - Local testing before PR submission

---

## 👥 Code Review Process
1. **Pre-Review Checklist**
   - All tests passing
   - Documentation updated
   - Cleanroom compliance verified
   - No linter warnings
   - Test coverage maintained

2. **Review Stages**
   - Initial automated checks
   - Primary reviewer assessment
   - Secondary reviewer for core changes
   - Final cleanroom compliance check

3. **Review Focus Areas**
   - Code correctness
   - Performance implications
   - Security considerations
   - Documentation quality
   - Test coverage
   - Cleanroom compliance

4. **Approval Requirements**
   - Two approving reviews
   - All CI checks passing
   - No unresolved comments
   - Cleanroom compliance confirmed

---

## 🔄 Release Protocols
1. **Pre-Release Phase**
   - Version bump PR
   - Changelog update
   - Documentation review
   - Full test suite execution
   - Security audit completion

2. **Release Process**
   - Create release branch
   - Run release test suite
   - Generate release artifacts
   - Sign release binaries
   - Update documentation
   - Tag release commit

3. **Post-Release**
   - Deploy to staging
   - Verify deployment
   - Monitor for issues
   - Update production
   - Announce release

4. **Security Protocols**
   - Regular security audits
   - Vulnerability reporting
   - Emergency patch process
   - CVE tracking

---

## 📝 Documentation Trail
To support auditing and open collaboration:
- All modules include top-level comments referencing JAM specs
- Significant design decisions are logged in `docs/decisions/`
- Any changes in architectural interpretation are documented

---

## ⚖ License & Legal
JamLiquor is licensed under **GPL-3.0**, with:
- Explicit declaration of cleanroom intent
- No reuse of GPL-incompatible sources

We welcome external audits, JAM Prize assessments, and community reviews to validate this policy.

---

## 📬 Questions or Concerns?
If you believe any part of JamLiquor's code violates cleanroom policy, please open a GitHub issue or contact us directly. Transparency is essential to our mission.

---

**JamLiquor is built with principles.**
**A JAM client for the ecosystem—not apart from it.**

---

**Navigation**
Previous: [Project Manifesto](./MANIFESTO.md) | Next: [Milestones & Roadmap](./MILESTONE.md)
