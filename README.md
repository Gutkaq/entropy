## entropy_hpc
we made a library for lattices nobody asked for

Gaussian integers. Quaternions. Octonions. SIMD acceleration. In Rust. Because reasons.

This is what happens when mathematicians and systems programmers have too much free time.
what is this, honestly

A Rust library that implements three increasingly unhinged normed division algebras:

    ZInt - Complex numbers but the real and imaginary parts are integers. Yes, this exists. No, you probably don't need it.

    HInt - Quaternions with half-integers sprinkled in for extra chaos. Multiplication doesn't commute. It's a feature.

    OInt - 8D hypercomplex integers using the Fano plane multiplication rules. Associativity is a luxury we can't afford. Moufang property? Sure, that's fine. But associativity? No.

All with AVX2 SIMD because we decided raw throughput was more important than your sanity.
performance metrics (that don't matter)

ZInt add 10k elements: 591ns
HInt add 10k elements: 1.4µs
OInt add 10k elements: 20ns

Your network latency is measured in milliseconds. These numbers mean nothing. They are theater. But they are fast theater.
features (because we had to list something)
ZInt (Gaussian Integers ℤ[i])

    Euclidean division with remainder

    GCD & Extended GCD (Bézout's identity)

    Exact division when applicable

    Conjugate operations

    Fractional arithmetic

    Normalization and unit tracking

    SIMD batch operations (4 at a time)

HInt (Hurwitz Quaternions)

    Non-commutative multiplication (ab ≠ ba, welcome to hell)

    Euclidean division over 4D space

    GCD computation

    Half-integers (literally 0.5 + 0.5i + 0.5j + 0.5k is valid)

    Conjugate, norm, inverse operations

    Fractional representation

    SIMD (2 quaternions at a time)

OInt (Integer Octonions Z[O])

    8D algebra with Fano plane multiplication

    NON-ASSOCIATIVE (this is not a bug, it's a feature)

    Moufang law verification (associativity is dead but we have something better)

    Euclidean division (somehow works)

    GCD over octonions (yes, really)

    Fractional arithmetic in 8D

    Full AVX2 SIMD support (all 8 dimensions at once)

SIMD Engine

    AVX2-accelerated operations with scalar fallback

    Batch processing for arrays

    Zero abstraction overhead

why though

Nobody knows. Octonions are the largest normed division algebra over the reals. After these, the mathematical structure breaks. We built a library for the most broken, unassociative, weird number system available because we could.

Use cases: Academic curiosity. Grief. A cry for help.
installation

text
[dependencies]
entropy_hpc = "0.1.0"

bash
git clone https://github.com/Gutkaq/entropy.git
cd entropy_hpc
cargo test --release -- --nocapture

Watch the tests pass. All of them. This is the only success you will experience with this library.
usage
Gaussian Integers

rust
use entropy_hpc::ZInt;

let z = ZInt::new(3, 4);
let norm = z.norm_squared();
let inv = z.inv_fraction().unwrap();
let (q, r) = z.div_rem(ZInt::new(2, 1)).unwrap();

Quaternions

rust
use entropy_hpc::HInt;

let q = HInt::new(1, 2, 3, 4);
let i = HInt::i();
let j = HInt::j();

assert_eq!(i * j, HInt::k());
assert_eq!(j * i, -HInt::k());

let gcd = HInt::gcd(q, HInt::new(2, 0, 0, 0));

Octonions

rust
use entropy_hpc::OInt;

let o = OInt::new(1, 2, 3, 4, 5, 6, 7, 8);
let e1 = OInt::e1();
let e2 = OInt::e2();

let non_commutative = e1 * e2 != e2 * e1;
let non_associative = (e1 * e2) * e1 != e1 * (e2 * e1);

let (q, r) = o.div_rem(OInt::new(2, 0, 0, 0, 0, 0, 0, 0)).unwrap();

the math (for the three people who care)

Gaussian Integers: ℤ[i] = {a + bi | a,b ∈ ℤ}
Norm: N(a+bi) = a² + b²

Hurwitz Quaternions: Half-integer or all-integer components, i²=j²=k²=ijk=-1
Non-commutative but still Euclidean.

Integer Octonions: 8D lattice with Fano plane multiplication
Non-associative. Moufang property holds. Euclidean division exists anyway.
testing

bash
cargo test --release
cargo test --release test_complete_api_showcase -- --nocapture

All tests pass. All your problems are now mathematical, not engineering.
contributing

Found a bug? It's a feature.
Want to add something? Why would you do this to yourself?
Issues and PRs welcome anyway.
license

MIT - do whatever you want with this cursed knowledge
credits

Built by people who asked "can we?" instead of "should we?"

The answer was yes.

Now go compute something. In 8D. With SIMD. God help us all.
