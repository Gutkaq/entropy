# entropy_hpc: API Documentation

## Overview

entropy_hpc is a high-performance Rust library for SIMD-accelerated Euclidean lattice algebra. It implements three increasingly sophisticated normed division algebras:

- CInt (Gaussian Integers): ℤ[i] — 2D complex integers
- HInt (Hurwitz Quaternions): Half-integer or full-integer quaternions — 4D hypercomplex
- OInt (Integer Octonions): 8D non-associative algebra with Fano plane multiplication

All types support Euclidean division, GCD computation, fractional arithmetic, and AVX2 SIMD acceleration.

---

## Module: cint (Complex Integers ℤ[i])

### Types

#### CInt
Represents a Gaussian integer: a + bi where a, b ∈ ℤ.

Fields:
- `a: i32` — Real part
- `b: i32` — Imaginary part

#### CIFraction
Represents a fraction of Gaussian integers: numerator / denominator.

Fields:
- `num: CInt` — Numerator
- `den: u64` — Denominator (always positive)

#### CIntError
Error type for CInt operations.

Variants:
- `Overflow` — Arithmetic overflow in multiplication
- `DivisionByZero` — Attempted division by zero
- `NotDivisible` — Exact division failed (non-zero remainder)
- `NoInverse` — Element has no inverse

### Constructors

```rust
pub fn new(a: i32, b: i32) -> Self
pub fn zero() -> Self
pub fn one() -> Self
pub fn i() -> Self
```

Creates common constants and values.

### Properties

```rust
pub fn is_zero(self) -> bool
pub fn is_unit(self) -> bool
pub fn norm_squared(self) -> u64
pub fn conj(self) -> Self
pub fn normalize(self) -> Self
pub fn associates(self) -> [Self; 4]
```

- `is_unit`: checks units in ℤ[i]: ±1, ±i.
- `norm_squared`: N(a+bi) = a² + b².
- `associates`: returns `{z, -z, iz, -iz}`.

### Arithmetic Operations

```rust
impl Add for CInt { ... }
impl Sub for CInt { ... }
impl Mul for CInt { ... }
impl Neg for CInt { ... }
```

All standard operators with wrapping/panicking on overflow in multiplication.

### Division & GCD

```rust
pub fn div_rem(self, d: Self) -> Result<(Self, Self), CIntError>
pub fn div_exact(self, d: Self) -> Result<Self, CIntError>
pub fn gcd(a: Self, b: Self) -> Self
pub fn xgcd(a: Self, b: Self) -> (Self, Self, Self)
```

- `div_rem`: Euclidean division returning `(q, r)` with `self = q*d + r` and `N(r) < N(d)`.
- `div_exact`: exact division or error.
- `xgcd`: extended GCD returning `(g, x, y)` where `g = ax + by`.

### Fractions

```rust
pub fn div_to_fraction(self, d: Self) -> Result<CIFraction, CIntError>
pub fn inv_fraction(self) -> Result<CIFraction, CIntError>
pub fn reduce_fraction(frac: CIFraction) -> CIFraction
pub fn inv_unit(self) -> Result<Self, CIntError>
```

---

## Module: hint (Hurwitz Quaternions)

### Types

#### HInt
Represents a Hurwitz quaternion: a + bi + cj + dk (half-integers or full integers).

Fields (stored as 2× actual value to represent halves uniformly):
- `a: i32`
- `b: i32`
- `c: i32`
- `d: i32`

#### HIFraction
Fraction of Hurwitz quaternions.

Fields:
- `num: HInt`
- `den: u64`

#### HIntError
Error type. Variants include `Overflow`, `DivisionByZero`, `NotDivisible`, `NoInverse`, and `InvalidHalfInteger` (mixed parity in `from_halves`).

### Constructors

```rust
pub fn new(a: i32, b: i32, c: i32, d: i32) -> Self
pub fn from_halves(a: i32, b: i32, c: i32, d: i32) -> Result<Self, HIntError>
pub fn zero() -> Self
pub fn one() -> Self
pub fn i() -> Self
pub fn j() -> Self
pub fn k() -> Self
```

`from_halves` requires all components to have the same parity (all odd for halves or all even for integers).

### Properties & Algebra

```rust
pub fn is_zero(self) -> bool
pub fn is_unit(self) -> bool
pub fn norm_squared(self) -> u64
pub fn conj(self) -> Self
pub fn to_float_components(self) -> (f64, f64, f64, f64)
pub fn normalize(self) -> Self
pub fn associates(self) -> [HInt; 8]
```

Quaternion-specific helpers:

```rust
pub fn is_anticommutative_pair(a: Self, b: Self) -> bool
pub fn is_associative_triple(a: Self, b: Self, c: Self) -> bool
```

Arithmetic and division mirror CInt but multiplication is non-commutative.

---

## Module: oint (Integer Octonions)

### Types

#### OInt
Represents an integer octonion: a + be1 + ce2 + de3 + ee4 + fe5 + ge6 + he7.

Fields (scalar stored as 2× for half-integers support):
- `a: i32`
- `b, c, d, e, f, g, h: i32`

#### OIFraction
Fraction of octonions.

#### OIntError
Same variants as CInt + `InvalidHalfInteger`.

### Constructors & Basis

```rust
pub fn new(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> Self
pub fn from_halves(a: i32, ..., h: i32) -> Result<Self, OIntError>
pub fn zero() -> Self
pub fn one() -> Self
pub fn e1() -> Self
pub fn e2() -> Self
pub fn e3() -> Self
pub fn e4() -> Self
pub fn e5() -> Self
pub fn e6() -> Self
pub fn e7() -> Self
```

### Properties

```rust
pub fn is_zero(self) -> bool
pub fn is_unit(self) -> bool
pub fn norm_squared(self) -> u64
pub fn conj(self) -> Self
pub fn to_float_components(self) -> (f64, f64, f64, f64, f64, f64, f64, f64)
pub fn normalize(self) -> Self
pub fn associates(self) -> [OInt; 8]
```

Octonion-specific algebra helpers include checks for non-commutativity, non-associativity, Moufang and alternative identities.

---

## Module: simd → simd_engine

SIMD-accelerated batch operations with AVX2 when available, scalar fallback otherwise.

### CInt SIMD (4 elements at a time)

```rust
pub fn cint_add_batch(a: &[CInt; 4], b: &[CInt; 4]) -> [CInt; 4]
pub fn cint_sub_batch(a: &[CInt; 4], b: &[CInt; 4]) -> [CInt; 4]
pub fn cint_mul_batch(a: &[CInt; 4], b: &[CInt; 4]) -> [CInt; 4]
```

Array versions process in 4-element chunks with a tail scalar.

### HInt SIMD (2 elements at a time)

```rust
pub fn hint_add_batch(a: &[HInt; 2], b: &[HInt; 2]) -> [HInt; 2]
pub fn hint_sub_batch(a: &[HInt; 2], b: &[HInt; 2]) -> [HInt; 2]
pub fn hint_mul_batch(a: &[HInt; 2], b: &[HInt; 2]) -> [HInt; 2]
```

### OInt SIMD (1 element, 8D vectorized)

```rust
pub fn oint_add_batch(a: &[OInt; 1], b: &[OInt; 1]) -> [OInt; 1]
pub fn oint_sub_batch(a: &[OInt; 1], b: &[OInt; 1]) -> [OInt; 1]
pub fn oint_mul_batch(a: &[OInt; 1], b: &[OInt; 1]) -> [OInt; 1]
```

---

## Module: display

Formatting and display implementations for types and fractions (`Display`, `Debug`).

---

## Example Usage

(Short examples showcasing CInt, HInt, OInt and SIMD usage are included in the library — see the repository examples.)

---

## Performance Notes

- CInt SIMD: ~30µs for 10k adds (4-element chunks with AVX2)
- HInt SIMD: ~80µs for 10k adds (2-element chunks)
- OInt SIMD: ~180µs for 10k adds (8D full vectorized)
- Multiplication: scalar fallback for all (complex formulas not SIMD-friendly)
- AVX2 Auto-detection: enabled at runtime; scalar fallback on older CPUs

---

## Mathematical Properties Guaranteed

| Property           | CInt | HInt | OInt |
|--------------------|:----:|:----:|:----:|
| Commutative        | ✓    | ✗    | ✗    |
| Associative        | ✓    | ✓    | ✗    |
| Euclidean          | ✓    | ✓    | ✓    |
| GCD exists         | ✓    | ✓    | ✓    |
| Division algorithm | ✓    | ✓    | ✓    |
| Half-integers      | ✗    | ✓    | ✓    |
| Moufang law        | -    | -    | ✓    |
| Alternative        | -    | -    | ✓    |

---

## Error Handling

All operations return `Result<T, Error>` except where noted:

- `new()` / `from_halves()` — May error on parity mismatch (halves)
- Arithmetic — Panics on overflow (i64 intermediate)
- Division/GCD — Errors on division by zero, non-divisibility
