# entropy_hpc

Welcome to entropy_hpc — where algebra meets SIMD, and chaos gets a speed boost.

entropy_hpc is a high-performance Rust library for Euclidean lattice algebra: Gaussian integers, Hurwitz quaternions, and integer octonions, all tuned to hum when AVX2 is available. Use it when you want exact arithmetic in funky dimensions, but also want it to run absurdly fast on modern CPUs.

## Quick Highlights (TL;DR)
- SIMD-accelerated batch operations (AVX2 when available, scalar fallback otherwise)
- Euclidean division and GCD for CInt, HInt and OInt
- Fraction support and canonical normalization (half-integers handled cleanly)
- Readable Display / Debug implementations and handy examples

## Install

Add to your Cargo.toml:

```toml
[dependencies]
entropy_hpc = { git = "https://github.com/Gutkaq/entropy" }
```

Or if a published crate exists:

```bash
cargo add entropy_hpc
```

## Quick Example

Caveat: these tiny snippets are for demonstration. See the library examples and API_DOC.md for fuller usage.

CInt (Gaussian integers)
```rust
use entropy_hpc::CInt;

let a = CInt::new(3, 4); // 3 + 4i
let b = CInt::new(1, 2); // 1 + 2i

println!("{} + {} = {}", a, b, a + b); // 4 + 6i
let (q, r) = a.div_rem(b).unwrap();
println!("div: q={}, r={}", q, r);
```

HInt (Hurwitz quaternions)
```rust
use entropy_hpc::HInt;

let i = HInt::i();
let j = HInt::j();
println!("i*j = {}", i * j); // k
```

SIMD example (CInt add batch)
```rust
use entropy_hpc::{CInt, simd_engine};

let ca = [CInt::new(1,2), CInt::new(3,4), CInt::new(5,6), CInt::new(7,8)];
let cb = [CInt::new(2,1), CInt::new(4,3), CInt::new(6,5), CInt::new(8,7)];
let result = simd_engine::cint_add_batch(&ca, &cb);
println!("{:?}", result); // [3+3i, 7+7i, 11+11i, 15+15i]
```

## Documentation

Full API and details are in API_DOC.md:
- ./API_DOC.md

## Features & Performance Notes

- CInt SIMD: ~30µs for 10k adds (4-element chunks with AVX2)
- HInt SIMD: ~80µs for 10k adds (2-element chunks)
- OInt SIMD: ~180µs for 10k adds (8D vectorized)
- Multiplication uses scalar formulas (complex to vectorize); addition and subtraction are vectorized where possible.

## Contributing

Contributions, bug reports, and performance ideas are welcome. If you want to help:
1. Open an issue describing the idea or bug.
2. Fork, branch, and open a PR with focused changes and tests.
3. Keep commits small and topic-focused.

If you'd like me to add a CONTRIBUTING.md or CI badges, say the word and I’ll draft them.

## License

MIT — see the LICENSE file.

## Final note (the witty bit)

If numbers had a gym membership, entropy_hpc would be the trainer yelling "more reps, less rounding errors!" — fast, exact, and oddly satisfying.
