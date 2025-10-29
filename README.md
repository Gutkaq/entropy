Entropy 🎲

Making complex numbers go brrrr (sometimes)
What's This?

entropy_hpc - Gaussian integers with AVX2 SIMD

We made multiplication 2x faster. Compiler auto-vectorized everything else and made us look silly.
🚀 Build & Run

cd entropy_hpc
cargo test
RUSTFLAGS='-C target-cpu=native' cargo test --release -- --ignored --nocapture
📊 Benchmark Results

Tested on 16,777,216 random Gaussian integers:

Complex Multiply: 37.9ms → 19.0ms = 2.00x speedup 🚀 (WE'RE HIM)

Addition: 23.6ms → 22.5ms = 1.05x (compiler already did this)

Associates: 66.2ms → 59.0ms = 1.12x (decent)

Norm²: 19.7ms → 20.4ms = 0.96x (we don't talk about this)
✨ Features

    ✅ Complete Gaussian integer arithmetic (ℤ[i])

    ✅ Euclidean division & GCD

    ✅ AVX2 SIMD batch operations (4-way parallel)

    ✅ 2x speedup on complex multiplication

    ✅ 40 unit tests (all passing)

    ✅ Automatic fallback for non-AVX2 CPUs

🎓 What We Learned

Manual SIMD beats compiler on complex operations. For simple loops, LLVM already optimizes better than we can.

Lesson: Profile first. The compiler might already be doing it.
🛠️ Requirements

    Rust 2021+

    x86_64 CPU with AVX2 (recommended)

📄 License

MIT

"We spent 3 weeks optimizing and the compiler was already doing it. Except multiplication. Multiplication is ours."
