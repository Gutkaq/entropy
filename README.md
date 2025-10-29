Entropy

Making math go fast (sometimes)
Projects
entropy_hpc

Gaussian integers with SIMD. Got a 2x speedup on multiplication. Compiler auto-vectorized everything else and made us look silly.
Quick Start

cd entropy_hpc
cargo test # Run tests
RUSTFLAGS='-C target-cpu=native' cargo test --release -- --ignored --nocapture # Benchmark
Benchmark Results (16M elements)
Operation	Speedup	Verdict
Multiply	2.00x	🎉 We won
Add/Sub	~1.0x	Compiler already did it
Norm²	0.96x	Oops

TL;DR: SIMD works for complex ops. For simple stuff, the compiler is already smarter than us.
Requirements

    Rust

    x86_64 CPU with AVX2 (or don't, fallbacks exist)

PROJECT README.md:
entropy_hpc

High-performance Gaussian integers (ℤ[i]) with AVX2 SIMD
What Is This

Complex numbers but integers only (a + bi). We made multiplication 2x faster with SIMD. The compiler auto-vectorized everything else.
Installation

git clone https://github.com/Gutkaq/entropy.git
cd entropy/entropy_hpc
cargo test
Benchmarks

cargo test --release -- --ignored --nocapture

Results on 16M random elements:

┌──────────────┬───────────┬───────────┬─────────┐
│ Operation │ Scalar(ms)│ SIMD(ms) │ Speedup │
├──────────────┼───────────┼───────────┼─────────┤
│ MUL │ 37.89 │ 18.97 │ 2.00x │ ← actual win
│ ADD │ 23.58 │ 22.51 │ 1.05x │ ← compiler did this
│ SUB │ 18.92 │ 19.03 │ 0.99x │ ← we tried
│ NORM² │ 19.70 │ 20.45 │ 0.96x │ ← L
└──────────────┴───────────┴───────────┴─────────┘
Usage

use entropy_hpc_lib::ZInt;

let z1 = ZInt::new(3, 4);
let z2 = ZInt::new(1, 2);

// Basic ops
let sum = z1 + z2;
let product = z1 * z2; // 2x faster with SIMD

// Number theory
let norm = z1.norm_squared();
let (quotient, remainder) = z1.div_rem(z2)?;
let gcd = ZInt::gcd(z1, z2);
Features

    ✅ Complete Gaussian integer arithmetic

    ✅ Euclidean division & GCD

    ✅ AVX2 batch operations (4-way parallel)

    ✅ 2x speedup on complex multiplication

    ✅ 40 unit tests

    ✅ Automatic scalar fallback

What We Learned

Manual SIMD beats compiler on complex operations. For simple loops, LLVM already optimizes better than we can.

Moral: Measure before optimizing. The compiler might already be doing it.
License

MIT

