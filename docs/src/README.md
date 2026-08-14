# JamLiquor

[← Back to Index](./index.md)

> This page mirrors the root [`README.md`](../../README.md), which is canonical — follow that link for the current quick-start, project status, and links. This page exists only for mdBook navigation.

JamLiquor is a cleanroom **JAM protocol research core**: it aims to prove deterministic agreement with the official binary test vectors (MVP-1) and then measure JAM's real cost on constrained ARM64/RISC-V hardware (MVP-2/3). It is not a validator, full client, or production node — see [`spec.md`](../../spec.md) for the binding product contract.

## 🚀 Quick Start

```bash
git clone https://github.com/jamliqr/jamliquor.git
cd jamliquor
rustup default stable
cargo build --release
cargo test
```

Run the legacy fixture importer (pre-pivot code, kept until `jam-core` supersedes it):

```bash
cargo run --release
```

## 🏗️ Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for the pre-pivot design philosophy, and [`spec.md`](../../spec.md) §6 for the current `jam-core` module plan (`codec`, `crypto`, `safrole`, `trie`).

## 🧪 Development

```bash
cargo test --all-targets
cargo fmt -- --check
cargo clippy --all-targets --all-features
```

See [Cleanroom Policy](CLEANROOM.md) for the review and provenance process.

## 🤝 Contributing

See [`CONTRIBUTING.md`](../../CONTRIBUTING.md).

## 📄 License

JamLiquor is licensed under GPL-3.0. See [`LICENSE`](../../LICENSE).

## 🔗 Additional Resources

- [Project Manifesto](MANIFESTO.md)
- [Development Guidelines](CLEANROOM.md)
- [Milestones & Roadmap](MILESTONE.md)
- [Research Notes (AI, PQC — retired from scope)](research-notes.md)

---

**Navigation**
Previous: [Documentation Home](./index.md) | Next: [Project Manifesto](./MANIFESTO.md)
