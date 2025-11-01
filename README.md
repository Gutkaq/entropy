```markdown
# entropy

entropy is a Rust workspace providing high-performance lattice algebra primitives and utilities:
- entropy_hpc: SIMD-accelerated Euclidean lattice algebra (Gaussian integers, Hurwitz quaternions, integer octonions)
- (other crates / binaries may exist in the workspace)

This repository focuses on correct, tested implementations of algebraic types with performance-oriented SIMD helpers and a small CLI/test harness.

Highlights
- Types: CInt (Gaussian integers), HInt (Hurwitz quaternions), OInt (integer octonions)
- SIMD engine with AVX2 acceleration and safe scalar fallbacks
- Test suite with a demonstration test exercising the public API
- Clean, documented APIs suitable for embedding in other Rust projects

Status
- Language: Rust
- Latest commit: 2025-10-31
- License: MIT (set in entropy_hpc/Cargo.toml; add a top-level LICENSE if desired)

Quickstart

1. Clone
   git clone https://github.com/Gutkaq/entropy.git
   cd entropy

2. Build (workspace)
   cargo build --release

3. Run the demo tests
   cargo test --all --tests

4. Run example (if a binary exists)
   cargo run --package entropy_hpc --bin demo --release

Development tools
- rustup (stable toolchain)
- cargo fmt
- cargo clippy
- cargo-audit (security)
- cargo-geiger (unsafe analysis)
- cargo-llvm-cov / cargo-tarpaulin (coverage)
- Optional: nightly toolchain for advanced profiling or codegen tricks

Recommended CI (GitHub Actions)
- Run: cargo fmt -- --check
- Run: cargo clippy --all-targets --all-features -- -D warnings
- Run: cargo test --all
- Run: cargo audit

Repository layout (relevant)
- entropy_hpc/              — SIMD-accelerated algebra crate
  - src/
  - tests/
  - Cargo.toml
  - API_DOC.md (module-level docs)
- README.md                 — this file
- (add LICENSE, CODE_OF_CONDUCT, CONTRIBUTING as needed)

Contributing
- Run cargo fmt and cargo clippy before creating PRs.
- Add unit tests for new behavior.
- When adding unsafe blocks, include a short comment documenting the safety invariants.

Contact
- Repo: https://github.com/Gutkaq/entropy
- Author: Gutkaq
```
