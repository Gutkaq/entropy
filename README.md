ntropy_hpc

SIMD-accelerated lattice algebra: CInt (Gaussian integers) - HInt (Quaternions) - OInt (Octonions)

Euclidean division. GCD. Half-integers. Non-commutative. Non-associative. Fast.
Quick Start

bash

    cargo test --release test_complete_api_showcase -- --nocapture

What You Get

    CInt: Complex integers. Boring. Works.

    HInt: Quaternions. Non-commutative. ij ≠ ji.

    OInt: Octonions. Non-associative. (ab)c ≠ a(bc). Sleep deprivation guaranteed.

All with AVX2 because we could. SIMD 10k adds in 30µs. Math doesn't care, but it's fast.
Features

✅ Euclidean division
✅ GCD/Extended GCD
✅ Half-integers (parity enforced)
✅ SIMD batching (4x CInt, 2x HInt, 8D OInt)
✅ Moufang identity + Alternativity
✅ Fractional arithmetic

See API_DOC.md for full docs.
License

MIT. Do whatever.

Built by people who asked "can we?" and didn't wait for the answer
