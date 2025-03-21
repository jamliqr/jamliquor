# 🧪 Cleanroom Policy for JamLiquor

## Overview
JamLiquor is a **cleanroom Rust implementation** of the JAM Protocol. It has been built from scratch to ensure:

- JAM Prize eligibility
- Legal and ethical separation from existing implementations
- Trust in code provenance and conformance

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
   - Parity’s JAM reference clients
   - Polkadot node source code
   - Any proprietary JAM-related implementation

2. **Allowed materials**:
   - JAM Graypaper (https://github.com/gavofyork/graypaper)
   - JAM Prize specs (https://hackmd.io/@polkadot/jamprize)
   - JAM-related public blog posts (e.g. kianenigma, parity forums)
   - Official documentation and diagrams (HackMD, GitHub wikis)

3. **Codebase boundaries**:
   - All core components (`importer`, `authorer`, `vm`) are written independently
   - Any reused crates are open-source and third-party (e.g. `blake3`, `dilithium`, `tinyml`)
   - Modular extensions are explicitly separated from JAM core logic

4. **Contributors declare compliance**:
   - All contributors agree to cleanroom principles
   - PRs involving potentially derivative logic are flagged and reviewed

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
If you believe any part of JamLiquor’s code violates cleanroom policy, please open a GitHub issue or contact us directly. Transparency is essential to our mission.

---

**JamLiquor is built with principles.**  
**A JAM client for the ecosystem—not apart from it.**

