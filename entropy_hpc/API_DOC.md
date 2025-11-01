# entropy_hpc — API Documentation

entropy_hpc is a high-performance Rust crate for SIMD-accelerated Euclidean lattice algebra and algebraic-integer arithmetic.  
It implements three related algebra families with consistent APIs, lattice helpers, and SIMD batch operations:

- CInt — Gaussian integers (ℤ[i]) — 2D complex integers  
- HInt — Hurwitz quaternions (Hurwitz integers) — 4D (supports half-integers)  
- OInt — Integer octonions — 8D (non-associative, Fano-plane multiplication)

All types provide arithmetic, Euclidean division, GCD, fraction representations, human-friendly Display/Debug, and AVX2 SIMD-accelerated batch add/sub helpers with scalar fallbacks.

---

Table of contents
- Quick overview
- Design & storage conventions
- Public types
  - CInt (Gaussian integers)
  - HInt (Hurwitz quaternions)
  - OInt (Integer octonions)
- Lattice helpers (Z² / D₄ / E₈)
- SIMD API (simd::)
- Display & Debug
- Error types
- Examples (usage snippets)
- Performance & notes
- Development & testing

---

Quick overview
--------------
- Crate: entropy_hpc
- Version: 0.3.0 (see entropy_hpc/Cargo.toml)
- Purpose: correct, tested algebraic-number types + high-performance SIMD batch helpers for lattice geometry (A2/Z², D4, E8)
- License: MIT (declared in entropy_hpc/Cargo.toml)

Design & storage conventions
----------------------------
- HInt and OInt store components multiplied by two (stored_value = 2 * mathematical_value). This lets the same representation handle integers and half-integers: integer components are even, half-integers are odd.
- Arithmetic operator traits are implemented (Add, Sub, Mul, Neg) for ergonomic arithmetic in Rust.
- Division APIs return Result<T, Error> (div_rem, div_exact). Fraction types represent numerators + denominators for exact rational-style results.
- SIMD code uses AVX2 intrinsics on x86_64 when available (runtime-detected) and falls back to scalar implementations otherwise.

Public types
------------

CInt — Gaussian integers (ℤ[i])
- Declaration:
  - repr(C) pub struct CInt { pub a: i32, pub b: i32 }
- Constructors:
  - CInt::new(a: i32, b: i32) -> CInt
  - CInt::zero(), CInt::one(), CInt::i()
- Core ops:
  - impl Add, Sub, Mul, Neg
  - Note: Mul uses i64 intermediates and panics on overflow when result does not fit in i32.
- Utilities:
  - is_zero() -> bool
  - is_unit() -> bool
  - conj() -> CInt
  - norm_squared() -> u64
  - normalize() -> CInt
  - associates() -> [CInt; 4]
- Division & Fractions:
  - div_rem(self, d: CInt) -> Result<(CInt, CInt), CIntError> — returns (q, r) with self = q*d + r and N(r) < N(d)
  - div_exact(self, d: CInt) -> Result<CInt, CIntError>
  - div_to_fraction(self, d: CInt) -> Result<CIFraction, CIntError>
  - inv_fraction(self) -> Result<CIFraction, CIntError>
  - reduce_fraction(frac: CIFraction) -> CIFraction
  - gcd(a: CInt, b: CInt) -> CInt
  - xgcd(a: CInt, b: CInt) -> (g: CInt, x: CInt, y: CInt)
- Lattice helpers (Z²):
  - to_lattice_vector(self) -> (i32, i32)
  - from_lattice_vector(v: (i32, i32)) -> CInt
  - lattice_distance_squared(self, other: CInt) -> i32
  - lattice_norm_squared(self) -> i32
  - closest_lattice_point_int(target: (i32, i32)) -> CInt
  - fundamental_domain() -> ((i32, i32), (i32, i32))
  - lattice_volume() -> i32
  - is_in_lattice(v: (i32, i32)) -> bool

HInt — Hurwitz quaternions (Hurwitz integers)
- Declaration:
  - repr(C) pub struct HInt { pub a: i32, pub b: i32, pub c: i32, pub d: i32 } (stored as 2×value)
- Constructors:
  - HInt::new(a, b, c, d) — accepts integer arguments (stored internally *2)
  - HInt::from_halves(a, b, c, d) -> Result<HInt, HIntError> — accepts parity-homogeneous half/integer inputs
  - zero(), one(), i(), j(), k()
- Core ops:
  - impl Add, Sub, Mul, Neg
  - Mul implements quaternion multiplication adapted for 2× storage
- Utilities:
  - is_zero(), is_unit(), conj(), norm_squared() -> u64
  - to_float_components() -> (f64,f64,f64,f64) — divide stored components by 2 for display
  - normalize(), associates() -> [HInt; 8]
  - is_anticommutative_pair(a, b) -> bool
  - is_associative_triple(a, b, c) -> bool
- Division & Fractions:
  - div_rem(self, d: HInt) -> Result<(HInt,HInt), HIntError>
  - div_exact(self, d: HInt) -> Result<HInt, HIntError>
  - div_to_fraction(self, den: HInt) -> Result<HIFraction, HIntError>
  - reduce_fraction(frac: HIFraction) -> HIFraction
  - inv_fraction(self) -> Result<HIFraction, HIntError>
  - inv_unit(self) -> Result<HInt, HIntError>
  - gcd(a: HInt, b: HInt) -> HInt
- Lattice helpers (D₄):
  - to_lattice_vector(), from_lattice_vector(), lattice_distance_squared(), lattice_norm_squared(), closest_lattice_point_int(), fundamental_domain(), lattice_volume(), is_in_lattice()

OInt — Integer octonions
- Declaration:
  - repr(C) pub struct OInt { pub a,b,c,d,e,f,g,h: i32 } (stored as 2×value)
- Constructors:
  - OInt::new(a..h)
  - OInt::from_halves(...) -> Result<OInt, OIntError>
  - zero(), one(), e1()..e7()
- Core ops:
  - impl Add, Sub, Mul, Neg
  - Mul implements Fano-plane based octonion multiplication and uses 2× storage conventions
- Utilities:
  - is_zero(), is_unit(), conj(), norm_squared(), to_float_components()
  - normalize(), associates()
  - is_non_commutative_pair(a,b), is_non_associative_triple(a,b,c)
  - alternative_identity(a,b) -> bool
  - moufang_identity(a,b,c) -> bool
- Division & Fractions:
  - div_rem(self, d: OInt) -> Result<(OInt, OInt), OIntError>
  - div_exact, div_to_fraction, reduce_fraction, inv_fraction, inv_unit, gcd
- Lattice helpers (E₈): analogous 8D helpers as for CInt/HInt, adapted for parity rule used by E₈

Lattice helpers (Z² / D₄ / E₈)
--------------------------------
- Each algebra type exposes convenience lattice helpers:
  - to_lattice_vector / from_lattice_vector
  - lattice_distance_squared / lattice_norm_squared
  - closest_lattice_point_int
  - fundamental_domain() -> basis vectors for the fundamental parallelotope
  - lattice_volume() -> i32
  - is_in_lattice(...) -> bool (parity / membership constraints)
- These are used by the SIMD lattice helpers to implement batch routines.

SIMD API (crate simd)
---------------------
- Module: entropy_hpc::simd
- Purpose: batch transformations and AVX2-accelerated add/sub for small fixed-size vectors
- Key simd_engine functions:
  - CInt (4-element lanes):
    - cint_add_batch(a: &[CInt; 4], b: &[CInt; 4]) -> [CInt; 4]
    - cint_sub_batch(...)
    - cint_mul_batch(...)  // scalar multiplication fallback
    - cint_add_arrays(a:&[CInt], b:&[CInt], out:&mut [CInt]) — chunked by 4 with tail
    - cint_sub_arrays, cint_mul_arrays
  - HInt (2-element lanes):
    - hint_add_batch(a: &[HInt; 2], b: &[HInt; 2]) -> [HInt; 2]
    - hint_sub_batch, hint_mul_batch
    - hint_*_arrays — chunked by 2 with tail
  - OInt (1-element 8×i32 lane):
    - oint_add_batch(a: &[OInt; 1], b: &[OInt; 1]) -> [OInt; 1]
    - oint_sub_batch, oint_mul_batch
    - oint_*_arrays
- Runtime behavior:
  - AVX2 intrinsics are used when the CPU supports them (is_x86_feature_detected!("avx2")), otherwise the scalar fallback is used to preserve correctness and portability.

SIMD lattice batch helpers
--------------------------
- Module: simd::lattice_simd (also simd_lattice)
- LatticeSimd offers thin batch wrappers:
  - Z² (A2): z2_to_lattice_batch, z2_from_lattice_batch, z2_distance_squared_batch, z2_norm_squared_batch, z2_closest_point_batch, z2_fundamental_domain_batch, z2_volume_batch, z2_in_lattice_batch
  - D₄: d4_... equivalents for HInt
  - E₈: e8_... equivalents for OInt
- These are implemented as safe, element-wise mappers returning Vecs (and on some platforms they can be chunked for SIMD-friendly iteration).

Display & Debug
---------------
- Display implementations:
  - CInt: "a + bi"
  - CIFraction: "(<num>) / <den>"
  - HInt: "a + bi + cj + dk" — prints half-integers nicely (e.g., "1/2")
  - HIFraction: "(...) / den"
  - OInt: "a + be₁ + ce₂ + ... + he₇"
  - OIFraction: "(...) / den"
- Debug implementations delegate to Display for readable output in tests and logs.

Error types
-----------
- CIntError: { Overflow, DivisionByZero, NotDivisible, NoInverse }
- HIntError: { Overflow, DivisionByZero, NotDivisible, NoInverse, InvalidHalfInteger }
- OIntError: { Overflow, DivisionByZero, NotDivisible, NoInverse, InvalidHalfInteger }

Examples (usage snippets)
-------------------------
CInt basic
```rust
use entropy_hpc::CInt;

let a = CInt::new(3, 4);
let b = CInt::new(1, 2);
println!("{} + {} = {}", a, b, a + b);
let (q, r) = a.div_rem(b).unwrap();
println!("div: q={}, r={}", q, r);
```

HInt (half-integers)
```rust
use entropy_hpc::HInt;

let q = HInt::new(1, 2, 3, 4);
let h = HInt::from_halves(1, 1, 1, 1).unwrap(); // 1/2 + 1/2 i + 1/2 j + 1/2 k
println!("q * q = {}", q * q);
```

OInt (octonions)
```rust
use entropy_hpc::OInt;

let o1 = OInt::new(1,1,1,1,0,0,0,0);
let o2 = OInt::e1();
println!("o1 + o2 = {}", o1 + o2);
assert!(OInt::alternative_identity(o1, o2));
```

SIMD / lattice batches
```rust
use entropy_hpc::{CInt, simd::LatticeSimd};

let pts = vec![CInt::new(0,0), CInt::new(1,1)];
let norms = LatticeSimd::z2_norm_squared_batch(&pts);
let vecs = LatticeSimd::z2_to_lattice_batch(&pts);
```

Performance & notes
-------------------
- SIMD adds large speedups for add/sub on AVX2-capable x86_64 hosts; multiplications remain scalar because algebraic multiplication is not generally SIMD-friendly.
- Multiplication uses i64 intermediates and will panic on overflow when casting back to i32. If you require non-panicking arithmetic, consider switching to checked arithmetic and Result-based APIs.
- div_rem uses floating-point rounding to compute quotient components; tests exercise typical cases. Be mindful of rounding ties when designing dependent algorithms.

Development & testing
---------------------
- Build: cargo build -p entropy_hpc --release
- Test (demo prints many checks): cargo test -p entropy_hpc --test demo -- --nocapture
- Docs: cargo doc -p entropy_hpc --no-deps --open
- Recommended dev tools: cargo fmt, cargo clippy, cargo-audit, cargo-geiger (unsafe analysis), cargo-llvm-cov or cargo-tarpaulin (coverage)

Where to look
-------------
- Source: entropy_hpc/src/{types, simd, lattice}
  - types: cint.rs, hint.rs, oint.rs, display.rs
  - simd: simd_engine.rs, lattice_simd.rs, simd_lattice.rs
  - lattice: z2.rs, d4.rs, e8.rs
- Tests & demo: entropy_hpc/tests/demo.rs — comprehensive live demonstration and assertions

Contributing
------------
- Run formatting and lints before PRs:
  - cargo fmt --all
  - cargo clippy --all-targets --all-features -- -D warnings
- Add tests for new behavior and document any unsafe invariants inline with the unsafe block.
- If you add unsafe code, comment invariants (why it is safe) and add tests that exercise edge cases.

---

This document reflects the crate's public API and behavior as implemented in the repository (files under entropy_hpc/src). If you want, I can produce a generated reference (extracting exact function/type signatures automatically) or open a PR with this updated API_DOC.md applied to the repo

