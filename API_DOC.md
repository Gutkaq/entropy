```markdown
# entropy_hpc — API Documentation

Purpose
--------
entropy_hpc is a high-performance Rust library exposing algebraic integer types and SIMD-accelerated batch operations for vectorized numeric work on small fixed-dimension algebras:
- CInt: Gaussian integers ℤ[i] (2D)
- HInt: Hurwitz quaternions (4D; supports half-integers)
- OInt: Integer octonions (8D; non-associative)

The crate provides arithmetic, Euclidean division, GCD, fraction support, human-friendly display, and AVX2-accelerated batch operations with scalar fallbacks.

Quick reference
---------------
- Crate name: entropy_hpc
- Version: 0.2.0
- Path: entropy_hpc/src
- Features: AVX2 accelerated SIMD engine (runtime-detected), scalar fallback

Public types & primary methods
-----------------------------

CInt (Gaussian integers)
- struct CInt { a: i32, b: i32 }
- Constructors:
  - CInt::new(a: i32, b: i32) -> CInt
  - CInt::zero(), CInt::one(), CInt::i()
- Core operations: Add, Sub, Mul, Neg
- Utilities:
  - is_zero(), is_unit(), conj(), norm_squared(), normalize()
  - div_rem(self, d: CInt) -> Result<(CInt, CInt), CIntError>
  - div_exact(self, d: CInt) -> Result<CInt, CIntError>
  - div_to_fraction(self, d: CInt) -> Result<CIFraction, CIntError>
  - gcd(a: CInt, b: CInt) -> CInt
  - xgcd(a: CInt, b: CInt) -> (g: CInt, x: CInt, y: CInt)
- Error: CIntError { Overflow, DivisionByZero, NotDivisible, NoInverse }
- Fraction: CIFraction { num: CInt, den: u64 }

HInt (Hurwitz quaternions)
- struct HInt { a: i32, b: i32, c: i32, d: i32 } (stored as 2×actual to support half-integers)
- Constructors:
  - HInt::new(a,b,c,d) — from integers
  - HInt::from_halves(a,b,c,d) -> Result<HInt, HIntError> — accepts parity-homogeneous components
  - zero(), one(), i(), j(), k()
- Core operations: Add, Sub, Mul, Neg
- Utilities:
  - is_zero(), is_unit(), conj(), norm_squared(), normalize()
  - div_rem(self, d: HInt) -> Result<(HInt, HInt), HIntError>
  - div_exact(self, d: HInt) -> Result<HInt, HIntError>
  - div_to_fraction(self, den: HInt) -> Result<HIFraction, HIntError>
  - gcd(a: HInt, b: HInt) -> HInt
- Extra checks:
  - from_halves enforces parity (no mixed half/integer components)
  - is_anticommutative_pair, is_associative_triple
- Error: HIntError { Overflow, DivisionByZero, NotDivisible, NoInverse, InvalidHalfInteger }
- Fraction: HIFraction { num: HInt, den: u64 }

OInt (Integer octonions)
- struct OInt { a,b,c,d,e,f,g,h: i32 } (stored as 2×actual)
- Constructors:
  - OInt::new(a,b,c,d,e,f,g,h)
  - OInt::from_halves(...) -> Result<OInt, OIntError>
  - zero(), one(), e1()..e7()
- Core operations: Add, Sub, Mul, Neg
- Utilities:
  - is_zero(), is_unit(), conj(), norm_squared(), normalize()
  - div_rem(self, d: OInt) -> Result<(OInt, OInt), OIntError>
  - div_exact(self, d: OInt) -> Result<OInt, OIntError>
  - div_to_fraction(self, den: OInt) -> Result<OIFraction, OIntError>
  - gcd(a: OInt, b: OInt) -> OInt
- Algebra-specific methods:
  - is_non_commutative_pair, is_non_associative_triple, alternative_identity, moufang_identity
- Error: OIntError { Overflow, DivisionByZero, NotDivisible, NoInverse, InvalidHalfInteger }
- Fraction: OIFraction { num: OInt, den: u64 }

Display & Debug
---------------
- Human-friendly Display implementations exist for CInt, CIFraction, HInt, HIFraction, OInt, OIFraction.
- Debug implementations delegate to Display for readable output in tests.

SIMD engine (simd::simd_engine)
------------------------------
- Runtime checks for AVX2 on x86_64; scalar fallback otherwise.
- CInt SIMD:
  - cint_add_batch(a: &[CInt;4], b: &[CInt;4]) -> [CInt;4]
  - cint_sub_batch(...)
  - cint_mul_batch(...) // scalar multiplication fallback
  - cint_add_arrays(a: &[CInt], b: &[CInt], out: &mut [CInt]) // chunked 4-element + tail
- HInt SIMD:
  - hint_add_batch(a: &[HInt;2], b: &[HInt;2]) -> [HInt;2]
  - hint_sub_batch(...)
  - hint_mul_batch(...)
  - hint_*_arrays(...) // chunked 2-element + tail
- OInt SIMD:
  - oint_add_batch(a: &[OInt;1], b: &[OInt;1]) -> [OInt;1] // full 8×i32 vectorized add
  - oint_sub_batch(...)
  - oint_mul_batch(...) // scalar fallback
  - oint_*_arrays(...) // per-element SIMD add/sub

Guidelines & notes
------------------
- Safety: no unsafe shown in public types; SIMD internal helpers use standard arch intrinsics gated by cfg and runtime checks. When adding unsafe code, document invariants.
- Panics: some arithmetic operations panic on overflow in i64->i32 conversion; consider returning Results or using checked arithmetics if you require no panics.
- Errors: division operations return typed error enums; check for DivisionByZero and NotDivisible.
- Tests: tests/demo.rs demonstrates API usage and is a useful reference for expected behavior and semantics.
- Performance: AVX2 adds major speedups for vectorized add/sub; multiplications are scalar because algebraic multiplication formulas are not generally SIMD-friendly.

Examples
--------
See `entropy_hpc/tests/demo.rs` for comprehensive usage examples and assertions.

Development
-----------
- Build: cargo build -p entropy_hpc --release
- Test: cargo test -p entropy_hpc -- --nocapture
- Document: cargo doc -p entropy_hpc --no-deps --open

Contact
-------
- Issues: https://github.com/Gutkaq/entropy/issues
- Author: Gutkaq
```
