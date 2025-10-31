entropy_hpc: API Documentation
Overview

entropy_hpc is a high-performance Rust library for SIMD-accelerated Euclidean lattice algebra. It implements three increasingly sophisticated normed division algebras:

    CInt (Gaussian Integers): ℤ[i] - 2D complex integers

    HInt (Hurwitz Quaternions): Half-integer or full-integer quaternions - 4D hypercomplex

    OInt (Integer Octonions): 8D non-associative algebra with Fano plane multiplication

All types support Euclidean division, GCD computation, fractional arithmetic, and AVX2 SIMD acceleration.
Module: cint (Complex Integers ℤ[i])
Types
CInt

Represents a Gaussian integer: a + bi where a, b ∈ ℤ.

Fields:

    a: i32 - Real part

    b: i32 - Imaginary part

CIFraction

Represents a fraction of Gaussian integers: numerator / denominator.

Fields:

    num: CInt - Numerator

    den: u64 - Denominator (always positive)

CIntError

Error type for CInt operations.

Variants:

    Overflow - Arithmetic overflow in multiplication

    DivisionByZero - Attempted division by zero

    NotDivisible - Exact division failed (non-zero remainder)

    NoInverse - Element has no inverse

Constructors

rust
pub fn new(a: i32, b: i32) -> Self

Create a Gaussian integer from real and imaginary parts.

rust
pub fn zero() -> Self

Return 0 + 0i (additive identity).

rust
pub fn one() -> Self

Return 1 + 0i (multiplicative identity).

rust
pub fn i() -> Self

Return 0 + 1i (imaginary unit).
Properties

rust
pub fn is_zero(self) -> bool

Check if equals zero.

rust
pub fn is_unit(self) -> bool

Check if unit (norm = 1). Units in ℤ[i]: ±1, ±i.

rust
pub fn norm_squared(self) -> u64

Compute N(a+bi) = a² + b² (Euclidean norm squared).

rust
pub fn conj(self) -> Self

Conjugate: a + bi → a - bi.

rust
pub fn normalize(self) -> Self

Normalize to canonical form (prioritize positive real part).

rust
pub fn associates(self) -> [Self; 4]

Return all 4 unit associates: {z, -z, iz, -iz}.
Arithmetic Operations

rust
impl Add for CInt { ... }
impl Sub for CInt { ... }
impl Mul for CInt { ... }
impl Neg for CInt { ... }

All standard operators with wrapping/panicking on overflow in mul.
Division & GCD

rust
pub fn div_rem(self, d: Self) -> Result<(Self, Self), CIntError>

Euclidean division: returns (q, r) where self = q*d + r and N(r) < N(d).

rust
pub fn div_exact(self, d: Self) -> Result<Self, CIntError>

Exact division: returns q if self = q*d exactly, else error.

rust
pub fn gcd(a: Self, b: Self) -> Self

Compute GCD using Euclidean algorithm (returns normalized result).

rust
pub fn xgcd(a: Self, b: Self) -> (Self, Self, Self)

Extended GCD: returns (g, x, y) where g = ax + by (Bézout identity).
Fractions

rust
pub fn div_to_fraction(self, d: Self) -> Result<CIFraction, CIntError>

Convert division to fraction form: self/d = (self·conj(d)) / N(d).

rust
pub fn inv_fraction(self) -> Result<CIFraction, CIntError>

Multiplicative inverse as fraction: 1/self = conj(self) / N(self).

rust
pub fn reduce_fraction(frac: CIFraction) -> CIFraction

Reduce fraction by GCD of numerator components and denominator.

rust
pub fn inv_unit(self) -> Result<Self, CIntError>

Inverse of a unit: for units, returns conjugate (since N=1).
Module: hint (Hurwitz Quaternions)
Types
HInt

Represents a Hurwitz quaternion: a + bi + cj + dk (half-integers or full integers).

Fields:

    a: i32 - Stored as 2×actual value

    b: i32 - Imaginary i component (2×actual)

    c: i32 - Imaginary j component (2×actual)

    d: i32 - Imaginary k component (2×actual)

Storage as 2× allows uniform handling of half-integers (odd values) and integers (even values).
HIFraction

Fraction of Hurwitz quaternions.

Fields:

    num: HInt - Numerator

    den: u64 - Denominator

HIntError

Error type.

Variants:

    Overflow, DivisionByZero, NotDivisible, NoInverse (same as CInt)

    InvalidHalfInteger - Mixed parity in from_halves (half and integer components together)

Constructors

rust
pub fn new(a: i32, b: i32, c: i32, d: i32) -> Self

Create from integers. Internally stores as 2× each component.

rust
pub fn from_halves(a: i32, b: i32, c: i32, d: i32) -> Result<Self, HIntError>

Create from half-integers. All components must have same parity (all odd for halves, all even for integers). No mixing allowed.

rust
pub fn zero() -> Self

Return 0 + 0i + 0j + 0k.

rust
pub fn one() -> Self

Return 1 + 0i + 0j + 0k.

rust
pub fn i() -> Self

Return 0 + 1i + 0j + 0k.

rust
pub fn j() -> Self

Return 0 + 0i + 1j + 0k.

rust
pub fn k() -> Self

Return 0 + 0i + 0j + 1k.
Properties

rust
pub fn is_zero(self) -> bool

Check if all components are zero.

rust
pub fn is_unit(self) -> bool

Check if norm = 1 (units in Hurwitz integers).

rust
pub fn norm_squared(self) -> u64

Compute N(q) = (a² + b² + c² + d²) / 4 (accounts for 2× storage).

rust
pub fn conj(self) -> Self

Quaternion conjugate: a + bi + cj + dk → a - bi - cj - dk.

rust
pub fn to_float_components(self) -> (f64, f64, f64, f64)

Convert to float: divides by 2 for display purposes.

rust
pub fn normalize(self) -> Self

Normalize by ensuring positive real part (or multiply by unit if needed).

rust
pub fn associates(self) -> [HInt; 8]

Return all 8 unit associates (multiply by each unit: ±1, ±i, ±j, ±k).
Quaternion Algebra

rust
pub fn is_anticommutative_pair(a: Self, b: Self) -> bool

Check if ab == -(ba). True for orthogonal basis elements (e.g., ij = -ji).

rust
pub fn is_associative_triple(a: Self, b: Self, c: Self) -> bool

Check if (ab)c == a(bc). Always true for quaternions.
Arithmetic

rust
impl Add, Sub, Mul, Neg for HInt { ... }

All operators implemented. Mul is non-commutative.
Division & GCD

rust
pub fn div_rem(self, d: HInt) -> Result<(HInt, HInt), HIntError>

Euclidean division over quaternions.

rust
pub fn div_exact(self, d: HInt) -> Result<HInt, HIntError>

Exact division.

rust
pub fn gcd(mut a: Self, mut b: Self) -> Self

GCD computation (Euclidean algorithm). No extended GCD (non-commutative).
Fractions

rust
pub fn div_to_fraction(self, den: HInt) -> Result<HIFraction, HIntError>

Convert to fraction form.

rust
pub fn reduce_fraction(frac: HIFraction) -> HIFraction

Reduce by GCD of all components.

rust
pub fn inv_fraction(self) -> Result<HIFraction, HIntError>

Multiplicative inverse as fraction.

rust
pub fn inv_unit(self) -> Result<HInt, HIntError>

Inverse of a unit: conjugate (since norm=1).
Module: oint (Integer Octonions)
Types
OInt

Represents an integer octonion: a + be₁ + ce₂ + de₃ + ee₄ + fe₅ + ge₆ + he₇.

Fields:

    a: i32 - Scalar part (2× storage for half-integer support)

    b, c, d, e, f, g, h: i32 - Seven imaginary basis coefficients

OIFraction

Fraction of octonions.
OIntError

Same variants as CInt + InvalidHalfInteger.
Constructors

rust
pub fn new(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> Self

Create from integers (stored as 2× each).

rust
pub fn from_halves(a: i32, ..., h: i32) -> Result<Self, OIntError>

Create from half-integers. Parity check enforced (no mixing).

rust
pub fn zero() -> Self
pub fn one() -> Self
pub fn e1() -> Self
pub fn e2() -> Self
pub fn e3() -> Self
pub fn e4() -> Self
pub fn e5() -> Self
pub fn e6() -> Self
pub fn e7() -> Self

Basis constructors.
Properties

rust
pub fn is_zero(self) -> bool
pub fn is_unit(self) -> bool
pub fn norm_squared(self) -> u64
pub fn conj(self) -> Self
pub fn to_float_components(self) -> (f64, f64, f64, f64, f64, f64, f64, f64)
pub fn normalize(self) -> Self
pub fn associates(self) -> [OInt; 8]

Same semantics as HInt.
Octonion-Specific Algebra

rust
pub fn is_non_commutative_pair(a: Self, b: Self) -> bool

Check if ab ≠ ba. True for most pairs (Fano plane).

rust
pub fn is_non_associative_triple(a: Self, b: Self, c: Self) -> bool

Check if (ab)c ≠ a(bc). True for some triples (defining property of octonions).

rust
pub fn moufang_identity(a: Self, b: Self, c: Self) -> bool

Check Moufang law: (ab)(ca) = a(b*c)*a. Always true for octonions.

rust
pub fn alternative_identity(a: Self, b: Self) -> bool

Check alternativity: (aa)b = a(ab) AND (ab)b = a(bb). True for all octonions.
Arithmetic & Division

rust
impl Add, Sub, Mul, Neg for OInt { ... }
pub fn div_rem(self, d: Self) -> Result<(Self, Self), OIntError>
pub fn div_exact(self, d: Self) -> Result<Self, OIntError>
pub fn gcd(mut a: Self, mut b: Self) -> Self

Same as HInt (Euclidean properties hold despite non-associativity).
Fractions

rust
pub fn div_to_fraction(self, den: Self) -> Result<OIFraction, OIntError>
pub fn reduce_fraction(frac: OIFraction) -> OIFraction
pub fn inv_fraction(self) -> Result<OIFraction, OIntError>
pub fn inv_unit(self) -> Result<Self, OIntError>

Same as HInt.
Module: simd → simd_engine

SIMD-accelerated batch operations with AVX2 when available, scalar fallback otherwise.
CInt SIMD (4 elements at a time)

rust
pub fn cint_add_batch(a: &[CInt; 4], b: &[CInt; 4]) -> [CInt; 4]
pub fn cint_sub_batch(a: &[CInt; 4], b: &[CInt; 4]) -> [CInt; 4]
pub fn cint_mul_batch(a: &[CInt; 4], b: &[CInt; 4]) -> [CInt; 4]

Batch operations for 4 Gaussian integers. Add/sub vectorized, mul scalar.

rust
pub fn cint_add_arrays(a: &[CInt], b: &[CInt], out: &mut [CInt])
pub fn cint_sub_arrays(a: &[CInt], b: &[CInt], out: &mut [CInt])
pub fn cint_mul_arrays(a: &[CInt], b: &[CInt], out: &mut [CInt])

Array versions: process in 4-element chunks + tail scalar.
HInt SIMD (2 elements at a time)

rust
pub fn hint_add_batch(a: &[HInt; 2], b: &[HInt; 2]) -> [HInt; 2]
pub fn hint_sub_batch(a: &[HInt; 2], b: &[HInt; 2]) -> [HInt; 2]
pub fn hint_mul_batch(a: &[HInt; 2], b: &[HInt; 2]) -> [HInt; 2]

Batch for 2 quaternions (8 i32s = 256 bits). Add/sub vectorized, mul scalar.

rust
pub fn hint_add_arrays(a: &[HInt], b: &[HInt], out: &mut [HInt])
pub fn hint_sub_arrays(a: &[HInt], b: &[HInt], out: &mut [HInt])
pub fn hint_mul_arrays(a: &[HInt], b: &[HInt], out: &mut [HInt])

Array versions: 2-element chunks + tail.
OInt SIMD (1 element, 8D vectorized)

rust
pub fn oint_add_batch(a: &[OInt; 1], b: &[OInt; 1]) -> [OInt; 1]
pub fn oint_sub_batch(a: &[OInt; 1], b: &[OInt; 1]) -> [OInt; 1]
pub fn oint_mul_batch(a: &[OInt; 1], b: &[OInt; 1]) -> [OInt; 1]

Full 8D vectorized add/sub in single 256-bit AVX2 instruction. Mul scalar.

rust
pub fn oint_add_arrays(a: &[OInt], b: &[OInt], out: &mut [OInt])
pub fn oint_sub_arrays(a: &[OInt], b: &[OInt], out: &mut [OInt])
pub fn oint_mul_arrays(a: &[OInt], b: &[OInt], out: &mut [OInt])

Array versions: SIMD per element for add/sub, scalar mul.
Module: display

Formatting and display implementations.
CInt Display

rust
impl Display for CInt
impl Display for CIFraction

Format: a + bi, (num) / den
HInt Display

rust
impl Display for HInt
impl Display for HIFraction

Format: a + bi + cj + dk (with proper signs, handles halves as 1/2).
Fraction: (a + bi + cj + dk) / den
OInt Display

rust
impl Display for OInt
impl Display for OIFraction

Format: a + be₁ + ce₂ + de₃ + ee₄ + fe₅ + ge₆ + he₇ (8D full).
Fraction: (8D components) / den
Debug Implementations

rust
impl Debug for CInt, CIFraction, HInt, HIFraction, OInt, OIFraction

Delegate to Display for readable debug output.
Example Usage
CInt

rust
use entropy_hpc::CInt;

let a = CInt::new(3, 4);
let b = CInt::new(1, 2);

// Arithmetic
println!("{} + {} = {}", a, b, a + b);  // 4 + 6i
println!("{} * {} = {}", a, b, a * b);  // -5 + 10i

// Euclidean division
let (q, r) = a.div_rem(b).unwrap();
println!("div: q={}, r={}", q, r);

// GCD
let gcd = CInt::gcd(CInt::new(12, 0), CInt::new(18, 0));
println!("gcd = {}", gcd);  // 6

HInt

rust
use entropy_hpc::HInt;

let q = HInt::new(1, 2, 3, 4);
let halves = HInt::from_halves(1, 1, 1, 1).unwrap();  // 1/2 + 1/2i + 1/2j + 1/2k

// Quaternion algebra
let i = HInt::i();
let j = HInt::j();
println!("i*j = {}", i * j);  // k
println!("i*i = {}", i * i);  // -1

// Norm multiplicativity
let prod = q * q;
println!("N(a*b) = {}, N(a)*N(b) = {}", prod.norm_squared(), q.norm_squared() * q.norm_squared());

OInt

rust
use entropy_hpc::OInt;

let e1 = OInt::e1();
let e2 = OInt::e2();

// Non-commutative
println!("e1*e2 = {}", e1 * e2);  // e4
println!("e2*e1 = {}", e2 * e1);  // -e4

// Non-associative
let e4 = OInt::e4();
let lhs = (e1 * e2) * e4;
let rhs = e1 * (e2 * e4);
println!("(e1*e2)*e4 = {} != {} = e1*(e2*e4)", lhs, rhs);

// Moufang holds
assert!(OInt::moufang_identity(e1, e2, e4));

SIMD

rust
use entropy_hpc::{CInt, simd_engine};

let ca = [CInt::new(1,2), CInt::new(3,4), CInt::new(5,6), CInt::new(7,8)];
let cb = [CInt::new(2,1), CInt::new(4,3), CInt::new(6,5), CInt::new(8,7)];

let result = simd_engine::cint_add_batch(&ca, &cb);
println!("{:?}", result);  // [3+3i, 7+7i, 11+11i, 15+15i]

// Array operations (10k elements)
let vec_a: Vec<CInt> = (0..10000).map(|i| CInt::new((i % 100) as i32, (i / 100) as i32)).collect();
let vec_b: Vec<CInt> = (0..10000).map(|i| CInt::new((i / 50) as i32, (i % 50) as i32)).collect();
let mut out = vec![CInt::zero(); 10000];

simd_engine::cint_add_arrays(&vec_a, &vec_b, &mut out);  // ~30µs for 10k

Performance Notes

    CInt SIMD: ~30µs for 10k adds (4-element chunks with AVX2)

    HInt SIMD: ~80µs for 10k adds (2-element chunks)

    OInt SIMD: ~180µs for 10k adds (8D full vectorized)

    Multiplication: Scalar fallback for all (complex formulas not SIMD-friendly)

    AVX2 Auto-detection: Enabled at runtime; scalar fallback on older CPUs

Mathematical Properties Guaranteed
Property	CInt	HInt	OInt
Commutative	✓	✗	✗
Associative	✓	✓	✗
Euclidean	✓	✓	✓
GCD exists	✓	✓	✓
Division algorithm	✓	✓	✓
Half-integers	✗	✓	✓
Moufang law	-	-	✓
Alternative	-	-	✓
Error Handling

All operations return Result<T, Error> except where noted:

    new() / from_halves() - May error on parity mismatch (halves)

    Arithmetic - Panics on overflow (i64 intermediate)

    Division/GCD - Errors on division by zero, non-divisibility

