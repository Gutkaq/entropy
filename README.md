🌀 entropy_hpc - Gaussian Integers Go BRRRR

    Because real numbers are for cowards and scalar code is a war crime

Blazingly fast™ Gaussian integers (ℤ[i]) with SIMD acceleration. We made imaginary math 20% faster. Your move, physicists.
What Even Is This?

Complex numbers but both parts are integers. So instead of boring 5, you get C O O L numbers like 5 + 3i. Euclidean division? GCD? Extended GCD? We got it all baby.
Quick Start

rust
use entropy_hpc::ZInt;

let a = ZInt::new(3, 4);  // 3 + 4i
let b = ZInt::new(5, 12); // 5 + 12i
let product = a * b;      // -33 + 56i (wait what)

// Division with remainder but make it ✨complex✨
let (q, r) = a.div_rem(b).unwrap();

// GCD because mathematicians have too much free time
let gcd = ZInt::gcd(a, b);

// SIMD MODE: Process 4 at once
let results = simd_engine::mul_batch(&[a,b,a,b], &[b,a,b,a]);

Features That Slap

    ✅ Euclidean Division - Yes, you can divide imaginary numbers

    ✅ GCD & Extended GCD - Find GCDs and Bézout coefficients because we're extra

    ✅ AVX2 SIMD - 1.2x faster than scalar (20% speed boost!)

    ✅ Actually Correct - 27 tests, 1000+ random cases, 0 failures

    ✅ Runtime CPU Detection - Falls back to scalar if your CPU is from 2010

Performance

text
Scalar:  410 µs 😴
SIMD:    341 µs 🚀  
Speedup: 1.20x 💪

Math Stuff (For Nerds)

Gaussian integers are a Euclidean domain with norm N(a+bi) = a²+b². They have units {1, -1, i, -i} and support:

    Euclidean algorithm (the OG from 300 BC)

    Bézout's identity: gcd(a,b) = sa + tb (up to units because nothing is simple)

    Unique factorization (mostly)

Testing

bash
cargo test --release -- --nocapture

Watch 27 tests pass and feel superior.
Roadmap

    Make it work

    Make it fast

    Make it correct

    Make it FASTER (always)

    ARM NEON support

    World domination

Warning

May cause sudden urges to explain complex number theory at parties.
License

MIT or Apache-2.0. We're not your dad.
