# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project status (read spec.md first)

JamLiquor is a cleanroom Rust implementation of the JAM Protocol, mid-pivot as of 2026-08-14. [`spec.md`](./spec.md) is the authoritative product contract (precedence: `spec.md` > sub-specs > `Plans.md`) — read it before any change that touches scope, identity claims, or milestones. [`Plans.md`](./Plans.md) is the task ledger and supersedes the archived `docs/archive/TODO-2026-04.md`.

Key constraints from `spec.md`:
- The current `src/` tree (`importer.rs`, `schema.rs`, `state.rs`, `coretime.rs`) is **legacy**: a JSON-fixture importer with structural checks, not a conformant JAM implementation. It parses JSON, not the binary JAM codec, and has no real cryptography (Blake2b hashing only; "signature validation" is a length check). It is kept only until `jam-core` (new workspace crate: `codec`, `crypto`, `safrole`, `trie` modules) supersedes it — see `spec.md` §6 and `Plans.md` Phase 2–4.
- `src/coretime.rs` models a "CoreTime" concept that does not correspond to any Gray Paper v0.7 subsystem; it is slated for deletion (`Plans.md` task 2.1), not extension.
- Every capability claim in docs must map to a green CI job or committed measurement (`spec.md` §8). Do not add aspirational claims when touching docs.
- `docs/REASSESSMENT/` and `DECISION.md` are immutable historical record — never edit them to fit later narratives.

## Commands

```bash
# Build
cargo build --release

# Run the legacy importer demo (imports tests/vectors/codec/tiny/block.json)
cargo run --release

# Test — full suite (unit tests in src/, integration tests in tests/, doctests)
cargo test --all-targets

# Run a single test by name (works for both unit and integration binaries)
cargo test test_block_import
cargo test --test block_validation_tests test_ticket_validation

# Lint / format (CI runs both with -D warnings / --check, so fix locally before pushing)
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings

# Benchmarks (criterion; currently panics on re-import — see Known issues)
cargo bench

# Code coverage (CI uses cargo-llvm-cov)
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Docs site (mdBook, source in docs/src/, config in docs/book.toml)
mdbook build docs
mdbook serve docs

# Security audit (CI job)
cargo audit
```

**Test vector submodules** (`tests/vectors` = `w3f/jamtestvectors`, `tests/vectors-davxy` = `davxy/jam-test-vectors`) must be initialized or most integration tests will fail to find fixtures:

```bash
git submodule update --init --recursive
```

The vector pin is frozen at **v0.7.1, commit `1dc503a`** for the MVP-1 window (`spec.md` §3) — do not run `git submodule update --remote` without checking `spec.md` first.

## Known current-state issues

- `cargo test --all-targets` currently has **one failing test**: `test_ticket_count_mismatch` in `tests/block_validation_tests.rs` — a stale test contradicting the Phase 1.0 ticket-count fix in `src/importer.rs`. Tracked for removal in `Plans.md` task 2.1; do not "fix" it by loosening the assertion, and do not treat a green run as the baseline until that task lands.
- `cargo bench` panics (`core_benchmarks` unwraps a failed re-import). Also tracked in `Plans.md` task 2.1.
- `proptest` is currently in `[dependencies]` instead of `[dev-dependencies]` in `Cargo.toml`, so it ships in release builds. Fix is `Plans.md` task 2.2 — don't casually "clean this up" outside that task since it's sequenced with other Cargo hygiene fixes.
- `cargo clippy --all-features -- -D warnings` (the exact CI command) currently **fails**: two unused-variable warnings in the stubbed `validate_extrinsic_hash` (`src/importer.rs:335`) become hard errors under `-D warnings`. Plain `cargo clippy` only warns. Also tracked for `Plans.md` task 2.2.

## Architecture

The crate is a single flat library (`jamliquor`, `src/lib.rs`) plus a thin binary (`src/main.rs`) that runs one demo import. Four modules:

- **`schema.rs`** — the JAM data model as `serde`-derived structs (`Block`, `Header`, `Extrinsic`, `TicketEnvelope`/`TicketBody`, `Preimage`, `EpochMark`, `OpaqueHash`) plus `BlockchainError` (`thiserror`) and a legacy `State`/`TicketState` pair that duplicates part of `state.rs`. These structs are JSON-shaped (via `serde_json`), not binary-codec-shaped — that mismatch is exactly what `jam-core/codec` (Phase 3) replaces.
- **`importer.rs`** — the `Importer` type, the only real pipeline in the crate: `import_block()` reads a JSON file → `validate_and_apply_block()` → `validate_block_structure()` / `validate_header()` / `validate_extrinsic()` / `validate_extrinsic_hash()` (currently stubbed out) → `state_mut().apply_block()`. Read this file first to understand the current control flow; everything else is data or support.
- **`state.rs`** — the real `State` used by `Importer` (distinct from `schema::State`): tracks `last_slot` and a `counter` derived from preimage counts. Slot-monotonicity validation lives here (`apply_block`), not in `importer.rs`.
- **`coretime.rs`** — `CoreTimeLedger`, tracking per-core resource consumption against constants like `MAX_CORETIME_PER_CORE`. Not part of the JAM spec (see Project status above); read only if working on its removal.

**Test layout**: `tests/unit/` mirrors `src/` module-by-module; `tests/integration/` exercises the importer against `tests/vectors/` fixtures (`vector_tests.rs`, `vector_diagnostics.rs`); `tests/common/mod.rs` and `tests/lib.rs` hold shared helpers (`get_vector_path`, `create_temp_test_dir`, a fixed `get_test_seed() == 42` for property tests). `tests/block_validation_tests.rs` at the top level is a standalone integration binary, not under `tests/unit/` or `tests/integration/`.

**Docs**: `docs/src/*.md` is the mdBook source (nav in `docs/src/SUMMARY.md`); `docs/archive/` holds pre-pivot documents preserved verbatim for cleanroom provenance and is *not* part of the mdBook build. `docs/REASSESSMENT/` is the dated 2026-08-13 assessment that produced the current `spec.md`.

---

Project-level instructions for Claude Code (and other LLM coding assistants) working in this repository. These merge with (not replace) any global/user-level instructions.

---

# Behavioral Guidelines

Reduce common LLM coding mistakes. These bias toward caution over speed — for trivial tasks, use judgment.

## 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:
- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them — don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

## 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

## 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:
- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it — don't delete it.

When your changes create orphans:
- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: every changed line should trace directly to the user's request.

## 4. Define Success Criteria — Loop Until Verified

Transform tasks into verifiable goals:
- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"

For multi-step tasks, state a brief plan:
```
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
```

Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.

---

**These guidelines are working if:** fewer unnecessary changes in diffs, fewer rewrites due to overcomplication, and clarifying questions come before implementation rather than after mistakes.

---

# Commit & Documentation Standards

**Do not include `Co-Authored-By:` trailers in commit messages.** This applies to all assistant-generated commits, including those produced by Claude Code or any other AI tool. Commit attribution stays with the human author. Boilerplate trailers add noise to the history without conveying meaningful authorship and should be retroactively stripped from past commits when asked.

**English-only requirement:**

- All `Plans.md` content must be in English (headers, table columns, task descriptions, status markers).
- No Japanese characters in `Plans.md` status markers (use `cc:done` instead of `cc:完了`, `cc:wip` instead of `cc:WIP`, `cc:todo` instead of `cc:TODO`/`cc:未着手`, etc).
- All harness output and documentation must be in English.
- This applies strictly to tracked files; commit to this constraint when editing `Plans.md`.
