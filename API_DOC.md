# API Documentation — entropy

This document describes the public API surface of the entropy crate and examples of how to use it. Replace types/function names below with the actual items from your codebase.

Table of contents
- Overview
- Crate features
- Public types
- Functions / Methods
- Errors
- Examples
- Notes on safety / thread-safety / performance

## Overview
entropy exposes a small, focused API to compute information-theoretic measures and utilities for randomness analysis. The crate is split into:
- library API (for embedding in other projects)
- CLI binary (if present) for ad-hoc usage

## Crate features
List available Cargo features and what they enable (example):
- default: safe, portable implementation
- simd: enable SIMD accelerated code paths
- serde: enable serialization for public types

## Public types

EntropyCalculator
- Purpose: primary high-level API to compute entropy and related statistics.
- Example:
```rust
use entropy::EntropyCalculator;

let calc = EntropyCalculator::new();
let e = calc.entropy(&[0u8, 1u8, 2u8]);
```

EntropyResult
- A struct containing results such as:
  - bits_per_byte: f64
  - distribution: Vec<(u8, f64)>
  - sample_size: usize

If your crate exposes different types, list them here and document fields.

## Functions / Methods

EntropyCalculator::new() -> EntropyCalculator
- Create a new calculator using default settings.

EntropyCalculator::with_config(cfg: Config) -> EntropyCalculator
- Create with custom settings (e.g., window size, normalization).

EntropyCalculator::entropy(&self, data: &[u8]) -> f64
- Compute Shannon entropy (bits/byte). Returns an f64.

EntropyCalculator::entropy_by_window(&self, data: &[u8], window: usize) -> Vec<EntropyResult>
- Compute sliding-window entropy. Useful for detecting local structure.

Utility functions
- fn shannon_distribution(data: &[u8]) -> HashMap<u8, usize>
- fn normalized_entropy_from_counts(counts: &HashMap<u8, usize>, sample_size: usize) -> f64

Document signatures exactly as in code and any panics/constraints.

## Errors
If functions return Result, list error types and when they occur:
- Error::EmptyInput — returned when input length is zero
- Error::InvalidWindow — returned when window size is 0 or > input length

Show how to handle:
```rust
match calc.entropy_checked(data) {
    Ok(e) => println!("{}", e),
    Err(e) => eprintln!("error: {}", e),
}
```

## Examples

1) Basic entropy
```rust
use entropy::EntropyCalculator;

let calc = EntropyCalculator::default();
let score = calc.entropy(b"hello world");
println!("entropy: {:.3}", score);
```

2) Sliding window
```rust
let results = calc.entropy_by_window(b"some long data...", 1024);
for r in results {
    println!("offset: {}, entropy: {}", r.offset, r.bits_per_byte);
}
```

3) CLI usage
If a CLI exists, show common flags:
```
entropy-cli --input file.bin --window 4096 --format json
```

## Notes on safety
- Document any unsafe blocks and their invariants.
- Describe thread-safety: Are types Send + Sync? Which functions can be called concurrently?

## Performance considerations
- Complexity: O(n + alphabet) for single-pass entropy.
- Memory: describe allocations and whether results reuse buffers.

## Testing recommendations
- Provide typical test vectors and expected entropy values for deterministic verification.
- Suggest using fuzzing for input-parsing functions.

## Changelog & versioning
- Indicate where to find release notes or CHANGELOG.md

## Contact / support
- Open issue: https://github.com/Gutkaq/entropy/issues
- Author: Gutkaq
