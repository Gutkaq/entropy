// Global type aliases
pub type I32 = i32;
pub type I64 = i64;
pub type U64 = u64;

// Declare core modules
pub mod simd;
pub mod zint;

// Exposing public types
pub use zint::{ZInt, ZIFraction, ZIntError};
pub use simd::simd_engine;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;

    // Constants
    const ONE: ZInt = ZInt { a: 1, b: 0 };
    const ZERO: ZInt = ZInt { a: 0, b: 0 };
    const I: ZInt = ZInt { a: 0, b: 1 };

    fn assert_euclidean(a: ZInt, q: ZInt, r: ZInt, d: ZInt) {
        assert_eq!(a, (q * d) + r);
        assert!(r.norm_squared() < d.norm_squared());
    }

    // ========== ALL 40 ZINT UNIT TESTS (unchanged) ==========
    
    #[test]
    fn test_div_rem_basic() -> Result<(), ZIntError> {
        let a = ZInt::new(7, 0);
        let d = ZInt::new(2, 0);
        let (q, r) = a.div_rem(d)?;
        assert_eq!(q, ZInt::new(4, 0));
        assert_eq!(r, ZInt::new(-1, 0));
        assert_euclidean(a, q, r, d);
        Ok(())
    }

    #[test]
    fn test_gcd() {
        let a = ZInt::new(5, 3);
        let b = ZInt::new(1, 7);
        assert_eq!(ZInt::gcd(a, b), ZInt::new(1, 1));
    }

    #[test]
    fn test_fractional_arithmetic() -> Result<(), ZIntError> {
        let f_div = ONE.div_to_fraction(ZInt::new(1, 1))?;
        assert_eq!(f_div.num, ZInt::new(1, -1));
        assert_eq!(f_div.den, 2);
        Ok(())
    }

    #[test]
    fn test_zint_constructors() {
        assert_eq!(ZInt::zero(), ZERO);
        assert_eq!(ZInt::one(), ONE);
        assert_eq!(ZInt::i(), I);
        let z = ZInt::new(3, 4);
        assert_eq!(z.a, 3);
        assert_eq!(z.b, 4);
    }

    #[test]
    fn test_zint_is_zero() {
        assert!(ZERO.is_zero());
        assert!(!ONE.is_zero());
        assert!(!I.is_zero());
        assert!(!ZInt::new(1, 1).is_zero());
    }

    #[test]
    fn test_zint_is_unit() {
        assert!(ONE.is_unit());
        assert!(ZInt::new(-1, 0).is_unit());
        assert!(I.is_unit());
        assert!(ZInt::new(0, -1).is_unit());
        assert!(!ZInt::new(2, 0).is_unit());
        assert!(!ZInt::new(1, 1).is_unit());
    }

    #[test]
    fn test_zint_conj() {
        let z = ZInt::new(3, 4);
        assert_eq!(z.conj(), ZInt::new(3, -4));
        let z2 = ZInt::new(-2, -5);
        assert_eq!(z2.conj(), ZInt::new(-2, 5));
    }

    #[test]
    fn test_zint_norm_squared() {
        assert_eq!(ZInt::new(3, 4).norm_squared(), 25);
        assert_eq!(ZInt::new(5, 12).norm_squared(), 169);
        assert_eq!(ZERO.norm_squared(), 0);
        assert_eq!(ONE.norm_squared(), 1);
        assert_eq!(I.norm_squared(), 1);
    }

    #[test]
    fn test_zint_addition() {
        let z1 = ZInt::new(1, 2);
        let z2 = ZInt::new(3, 4);
        assert_eq!(z1 + z2, ZInt::new(4, 6));
        assert_eq!(z1 + ZERO, z1);
        assert_eq!(z1 + z2, z2 + z1);
    }

    #[test]
    fn test_zint_subtraction() {
        let z1 = ZInt::new(5, 7);
        let z2 = ZInt::new(2, 3);
        assert_eq!(z1 - z2, ZInt::new(3, 4));
        assert_eq!(z1 - ZERO, z1);
        assert_eq!(z1 - z1, ZERO);
    }

    #[test]
    fn test_zint_negation() {
        let z = ZInt::new(3, -4);
        assert_eq!(-z, ZInt::new(-3, 4));
        assert_eq!(-ZERO, ZERO);
        assert_eq!(-(-z), z);
    }

    #[test]
    fn test_zint_multiplication() {
        let z1 = ZInt::new(1, 2);
        let z2 = ZInt::new(3, 4);
        assert_eq!(z1 * z2, ZInt::new(-5, 10));
        assert_eq!(z1 * ONE, z1);
        assert_eq!(z1 * ZERO, ZERO);
        assert_eq!(I * I, ZInt::new(-1, 0));
        assert_eq!(z1 * z2, z2 * z1);
    }

    #[test]
    fn test_zint_multiply_by_i() {
        let z = ZInt::new(3, 4);
        assert_eq!(z * I, ZInt::new(-4, 3));
    }

    #[test]
    fn test_zint_div_rem_exact_division() {
        let z1 = ZInt::new(6, 8);
        let z2 = ZInt::new(2, 0);
        let (q, r) = z1.div_rem(z2).unwrap();
        assert_eq!(q, ZInt::new(3, 4));
        assert!(r.is_zero());
        assert_eq!(z1, q * z2 + r);
    }

    #[test]
    fn test_zint_div_rem_with_remainder() {
        let z1 = ZInt::new(7, 3);
        let z2 = ZInt::new(2, 1);
        let (q, r) = z1.div_rem(z2).unwrap();
        assert_eq!(z1, q * z2 + r);
        assert!(r.norm_squared() < z2.norm_squared());
    }

    #[test]
    fn test_zint_div_rem_by_zero() {
        let z = ZInt::new(5, 3);
        assert!(matches!(z.div_rem(ZERO), Err(ZIntError::DivisionByZero)));
    }

    #[test]
    fn test_zint_div_exact_success() {
        let z1 = ZInt::new(10, 5);
        let z2 = ZInt::new(5, 0);
        let q = z1.div_exact(z2).unwrap();
        assert_eq!(q, ZInt::new(2, 1));
        assert_eq!(z1, q * z2);
    }

    #[test]
    fn test_zint_div_exact_not_divisible() {
        let z1 = ZInt::new(7, 3);
        let z2 = ZInt::new(2, 0);
        assert!(matches!(z1.div_exact(z2), Err(ZIntError::NotDivisible)));
    }

    #[test]
    fn test_zint_inv_unit_success() {
        assert_eq!(I.inv_unit().unwrap(), ZInt::new(0, -1));
        assert_eq!(ONE.inv_unit().unwrap(), ONE);
        assert_eq!(ZInt::new(-1, 0).inv_unit().unwrap(), ZInt::new(-1, 0));
    }

    #[test]
    fn test_zint_inv_unit_not_a_unit() {
        let z = ZInt::new(2, 0);
        assert!(matches!(z.inv_unit(), Err(ZIntError::NoInverse)));
    }

    #[test]
    fn test_zint_div_to_fraction_simple() {
        let z1 = ZInt::new(1, 0);
        let z2 = ZInt::new(2, 0);
        let frac = z1.div_to_fraction(z2).unwrap();
        assert_eq!(frac.num, ZInt::new(1, 0));
        assert_eq!(frac.den, 2);
    }

    #[test]
    fn test_zint_div_to_fraction_complex() {
        let z1 = ZInt::new(1, 1);
        let z2 = ZInt::new(1, -1);
        let frac = z1.div_to_fraction(z2).unwrap();
        let reduced = ZInt::reduce_fraction(frac);
        assert_eq!(reduced.num, I);
        assert_eq!(reduced.den, 1);
    }

    #[test]
    fn test_zint_div_to_fraction_by_zero() {
        let z = ZInt::new(1, 0);
        assert!(matches!(z.div_to_fraction(ZERO), Err(ZIntError::DivisionByZero)));
    }

    #[test]
    fn test_zint_inv_fraction() {
        let z = ZInt::new(2, 0);
        let inv = z.inv_fraction().unwrap();
        assert_eq!(inv.num, ZInt::new(1, 0));
        assert_eq!(inv.den, 2);
    }

    #[test]
    fn test_zint_reduce_fraction() {
        let frac = ZIFraction {
            num: ZInt::new(4, 6),
            den: 8,
        };
        let reduced = ZInt::reduce_fraction(frac);
        assert_eq!(reduced.num, ZInt::new(2, 3));
        assert_eq!(reduced.den, 4);
    }

    #[test]
    fn test_zint_associates() {
        let z = ZInt::new(3, 4);
        let assocs = z.associates();
        assert_eq!(assocs[0], ZInt::new(3, 4));
        assert_eq!(assocs[1], ZInt::new(-4, 3));
        assert_eq!(assocs[2], ZInt::new(-3, -4));
        assert_eq!(assocs[3], ZInt::new(4, -3));
        for a in &assocs {
            assert_eq!(a.norm_squared(), z.norm_squared());
        }
    }

    #[test]
    fn test_zint_normalize_first_quadrant() {
        let z = ZInt::new(3, 4);
        assert_eq!(z.normalize(), z);
    }

    #[test]
    fn test_zint_normalize_other_quadrants() {
        let z1 = ZInt::new(-3, 4);
        let n1 = z1.normalize();
        assert!(n1.a > 0 && n1.b >= 0);
        
        let z2 = ZInt::new(-3, -4);
        let n2 = z2.normalize();
        assert!(n2.a > 0 && n2.b >= 0);
        
        let z3 = ZInt::new(3, -4);
        let n3 = z3.normalize();
        assert!(n3.a > 0 && n3.b >= 0);
    }

    #[test]
    fn test_zint_normalize_zero() {
        assert_eq!(ZERO.normalize(), ZERO);
    }

    #[test]
    fn test_zint_normalize_on_axes() {
        assert_eq!(ZInt::new(5, 0).normalize(), ZInt::new(5, 0));
        assert_eq!(ZInt::new(-5, 0).normalize(), ZInt::new(5, 0));
        assert_eq!(ZInt::new(0, 5).normalize(), ZInt::new(0, 5));
        assert_eq!(ZInt::new(0, -5).normalize(), ZInt::new(5, 0));
    }

    #[test]
    fn test_zint_gcd_coprime() {
        let z1 = ZInt::new(3, 0);
        let z2 = ZInt::new(2, 0);
        let g = ZInt::gcd(z1, z2);
        assert!(g.is_unit());
    }

    #[test]
    fn test_zint_gcd_common_factor() {
        let z1 = ZInt::new(10, 0);
        let z2 = ZInt::new(6, 0);
        let g = ZInt::gcd(z1, z2);
        assert_eq!(g.norm_squared(), 4);
    }

    #[test]
    fn test_zint_gcd_with_zero() {
        let z = ZInt::new(5, 3);
        assert_eq!(ZInt::gcd(z, ZERO), z.normalize());
        assert_eq!(ZInt::gcd(ZERO, z), z.normalize());
    }

    #[test]
    fn test_zint_gcd_gaussian() {
        let z1 = ZInt::new(3, 0);
        let z2 = ZInt::new(0, 3);
        let g = ZInt::gcd(z1, z2);
        assert_eq!(g, g.normalize());
    }

    #[test]
    fn test_zint_gcd_symmetry() {
        let z1 = ZInt::new(15, 10);
        let z2 = ZInt::new(5, 5);
        assert_eq!(ZInt::gcd(z1, z2), ZInt::gcd(z2, z1));
    }

    #[test]
    fn test_zint_display() {
        assert_eq!(format!("{}", ZInt::new(3, 4)), "3 + 4i");
        assert_eq!(format!("{}", ZInt::new(-2, -5)), "-2 + -5i");
        assert_eq!(format!("{}", ZERO), "0 + 0i");
        assert_eq!(format!("{}", ONE), "1 + 0i");
        assert_eq!(format!("{}", I), "0 + 1i");
    }

    #[test]
    fn test_zint_large_values() {
        let z1 = ZInt::new(1000, 2000);
        let z2 = ZInt::new(500, 1000);
        let sum = z1 + z2;
        assert_eq!(sum, ZInt::new(1500, 3000));
        let diff = z1 - z2;
        assert_eq!(diff, ZInt::new(500, 1000));
    }

    #[test]
    #[should_panic(expected = "ZInt component overflow during multiplication")]
    fn test_zint_mul_overflow() {
        let z1 = ZInt::new(i32::MAX / 2, i32::MAX / 2);
        let z2 = ZInt::new(10, 10);
        let _ = z1 * z2;
    }

    #[test]
    fn test_zint_conjugate_involution() {
        let z = ZInt::new(3, 4);
        assert_eq!(z.conj().conj(), z);
    }

    #[test]
    fn test_zint_norm_multiplicative() {
        let z1 = ZInt::new(3, 4);
        let z2 = ZInt::new(5, 12);
        let product = z1 * z2;
        assert_eq!(product.norm_squared(), z1.norm_squared() * z2.norm_squared());
    }

    // ========== EXTREME BRUTAL SIMD BENCHMARK WITH RANDOM DATA ==========
    
    const BENCH_SIZE: usize = 16_777_216; // 16M elements - CRANKED UP!
    const DIV_SIZE: usize = 4_194_304;    // 4M for division ops - MASSIVE!

    fn verify_results<T: PartialEq + std::fmt::Debug>(
        scalar: &[T],
        simd: &[T],
        op_name: &str,
        scalar_ns: u128,
        simd_ns: u128
    ) {
        let errors = scalar.iter().zip(simd.iter())
            .filter(|&(s, g)| s != g)
            .count();
        
        let scalar_ms = scalar_ns as f64 / 1_000_000.0;
        let simd_ms = simd_ns as f64 / 1_000_000.0;
        let speedup = if simd_ns > 0 { scalar_ns as f64 / simd_ns as f64 } else { 0.0 };
        
        println!("│ {:12} │ {:9.2} │ {:9.2} │ {:7.2}x │ {:6} │",
            op_name, scalar_ms, simd_ms, speedup,
            if errors == 0 { "✓" } else { "✗" }
        );
        
        if errors > 0 {
            eprintln!("ERROR: {} mismatches in {}", errors, op_name);
            panic!("SIMD verification failed for {}", op_name);
        }
    }

    #[test]
    #[ignore]
    fn benchmark_simd_operations() {
        println!("\n╔═══════════════════════════════════════════════════════════════════════╗");
        println!("║       EXTREME BRUTAL SIMD BENCHMARK - RANDOM DATA (16M elements)     ║");
        println!("╚═══════════════════════════════════════════════════════════════════════╝\n");
        
        if !is_x86_feature_detected!("avx2") {
            println!("⚠ AVX2 not detected. Skipping benchmark.");
            return;
        }
        
        assert!(BENCH_SIZE % 4 == 0);
        assert!(DIV_SIZE % 4 == 0);
        
        println!("🎲 Generating {} random Gaussian integers...", BENCH_SIZE);
        
        // Generate RANDOM data with seeded RNG for reproducibility
        let mut rng = StdRng::seed_from_u64(42);
        let mut in_a = Vec::with_capacity(BENCH_SIZE);
        let mut in_b = Vec::with_capacity(BENCH_SIZE);
        
        for _ in 0..BENCH_SIZE {
            in_a.push(ZInt::new(rng.gen_range(-10000..10000), rng.gen_range(-10000..10000)));
            in_b.push(ZInt::new(rng.gen_range(-10000..10000), rng.gen_range(-10000..10000)));
        }

        println!("┌──────────────┬───────────┬───────────┬─────────┬────────┐");
        println!("│  Operation   │ Scalar(ms)│ SIMD(ms)  │ Speedup │ Status │");
        println!("├──────────────┼───────────┼───────────┼─────────┼────────┤");

        // Allocate output buffers once
        let mut out_s = vec![ZERO; BENCH_SIZE];
        let mut out_g = vec![ZERO; BENCH_SIZE];

        // ADD
        let t_s = { let t = Instant::now(); for i in 0..BENCH_SIZE { out_s[i] = in_a[i] + in_b[i]; } t.elapsed().as_nanos() };
        let t_g = { let t = Instant::now(); unsafe { simd_engine::zint_batch_add_simd(&mut out_g, &in_a, &in_b, BENCH_SIZE); } t.elapsed().as_nanos() };
        verify_results(&out_s, &out_g, "ADD", t_s, t_g);

        // SUB
        let t_s = { let t = Instant::now(); for i in 0..BENCH_SIZE { out_s[i] = in_a[i] - in_b[i]; } t.elapsed().as_nanos() };
        let t_g = { let t = Instant::now(); unsafe { simd_engine::zint_batch_sub_simd(&mut out_g, &in_a, &in_b, BENCH_SIZE); } t.elapsed().as_nanos() };
        verify_results(&out_s, &out_g, "SUB", t_s, t_g);

        // MUL
        let t_s = { let t = Instant::now(); for i in 0..BENCH_SIZE { out_s[i] = in_a[i] * in_b[i]; } t.elapsed().as_nanos() };
        let t_g = { let t = Instant::now(); unsafe { simd_engine::zint_batch_mul_simd(&mut out_g, &in_a, &in_b, BENCH_SIZE); } t.elapsed().as_nanos() };
        verify_results(&out_s, &out_g, "MUL", t_s, t_g);

        // NEG
        let t_s = { let t = Instant::now(); for i in 0..BENCH_SIZE { out_s[i] = -in_a[i]; } t.elapsed().as_nanos() };
        let t_g = { let t = Instant::now(); unsafe { simd_engine::zint_batch_neg_simd(&mut out_g, &in_a, BENCH_SIZE); } t.elapsed().as_nanos() };
        verify_results(&out_s, &out_g, "NEG", t_s, t_g);

        // CONJ
        let t_s = { let t = Instant::now(); for i in 0..BENCH_SIZE { out_s[i] = in_a[i].conj(); } t.elapsed().as_nanos() };
        let t_g = { let t = Instant::now(); unsafe { simd_engine::zint_batch_conj_simd(&mut out_g, &in_a, BENCH_SIZE); } t.elapsed().as_nanos() };
        verify_results(&out_s, &out_g, "CONJ", t_s, t_g);

        // NORM²
        let mut norm_s = vec![0u64; BENCH_SIZE];
        let mut norm_g = vec![0u64; BENCH_SIZE];
        let t_s = { let t = Instant::now(); for i in 0..BENCH_SIZE { norm_s[i] = in_a[i].norm_squared(); } t.elapsed().as_nanos() };
        let t_g = { let t = Instant::now(); unsafe { simd_engine::zint_batch_norm_squared_simd(&mut norm_g, &in_a, BENCH_SIZE); } t.elapsed().as_nanos() };
        verify_results(&norm_s, &norm_g, "NORM²", t_s, t_g);

        // IS_ZERO
        let mut bool_s = vec![false; BENCH_SIZE];
        let mut bool_g = vec![false; BENCH_SIZE];
        let t_s = { let t = Instant::now(); for i in 0..BENCH_SIZE { bool_s[i] = in_a[i].is_zero(); } t.elapsed().as_nanos() };
        let t_g = { let t = Instant::now(); unsafe { simd_engine::zint_batch_is_zero_simd(&mut bool_g, &in_a, BENCH_SIZE); } t.elapsed().as_nanos() };
        verify_results(&bool_s, &bool_g, "IS_ZERO", t_s, t_g);

        // IS_UNIT
        let t_s = { let t = Instant::now(); for i in 0..BENCH_SIZE { bool_s[i] = in_a[i].is_unit(); } t.elapsed().as_nanos() };
        let t_g = { let t = Instant::now(); unsafe { simd_engine::zint_batch_is_unit_simd(&mut bool_g, &in_a, BENCH_SIZE); } t.elapsed().as_nanos() };
        verify_results(&bool_s, &bool_g, "IS_UNIT", t_s, t_g);

        // NORMALIZE
        let t_s = { let t = Instant::now(); for i in 0..BENCH_SIZE { out_s[i] = in_a[i].normalize(); } t.elapsed().as_nanos() };
        let t_g = { let t = Instant::now(); unsafe { simd_engine::zint_batch_normalize_simd(&mut out_g, &in_a, BENCH_SIZE); } t.elapsed().as_nanos() };
        verify_results(&out_s, &out_g, "NORMALIZE", t_s, t_g);

        // ASSOCIATES
        let mut assoc_s = vec![ZERO; BENCH_SIZE * 4];
        let mut assoc_g = vec![ZERO; BENCH_SIZE * 4];
        let t_s = { let t = Instant::now(); for i in 0..BENCH_SIZE { let a = in_a[i].associates(); assoc_s[i*4..(i+1)*4].copy_from_slice(&a); } t.elapsed().as_nanos() };
        let t_g = { let t = Instant::now(); unsafe { simd_engine::zint_batch_associates_simd(&mut assoc_g, &in_a, BENCH_SIZE); } t.elapsed().as_nanos() };
        verify_results(&assoc_s, &assoc_g, "ASSOCIATES", t_s, t_g);

        println!("├──────────────┴───────────┴───────────┴─────────┴────────┤");
        println!("│  Division & Number Theory Operations ({} elements) │", DIV_SIZE);
        println!("├──────────────┬───────────┬───────────┬─────────┬────────┤");

        let a_div = &in_a[..DIV_SIZE];
        let b_div = &in_b[..DIV_SIZE];

        // DIV_REM
        let mut q_s = vec![ZERO; DIV_SIZE];
        let mut r_s = vec![ZERO; DIV_SIZE];
        let mut q_g = vec![ZERO; DIV_SIZE];
        let mut r_g = vec![ZERO; DIV_SIZE];
        let t_s = { let t = Instant::now(); for i in 0..DIV_SIZE { if let Ok((q, r)) = a_div[i].div_rem(b_div[i]) { q_s[i] = q; r_s[i] = r; } } t.elapsed().as_nanos() };
        let t_g = { let t = Instant::now(); unsafe { let _ = simd_engine::zint_batch_div_rem_simd(&mut q_g, &mut r_g, a_div, b_div, DIV_SIZE); } t.elapsed().as_nanos() };
        verify_results(&q_s, &q_g, "DIV_REM", t_s, t_g);

        // GCD
        let mut gcd_s = vec![ZERO; DIV_SIZE];
        let mut gcd_g = vec![ZERO; DIV_SIZE];
        let t_s = { let t = Instant::now(); for i in 0..DIV_SIZE { gcd_s[i] = ZInt::gcd(a_div[i], b_div[i]); } t.elapsed().as_nanos() };
        let t_g = { let t = Instant::now(); unsafe { simd_engine::zint_batch_gcd_simd(&mut gcd_g, a_div, b_div, DIV_SIZE); } t.elapsed().as_nanos() };
        verify_results(&gcd_s, &gcd_g, "GCD", t_s, t_g);

        // DIV_TO_FRAC
        let mut frac_s = vec![ZIFraction { num: ZERO, den: 1 }; DIV_SIZE];
        let mut frac_g = vec![ZIFraction { num: ZERO, den: 1 }; DIV_SIZE];
        let t_s = { let t = Instant::now(); for i in 0..DIV_SIZE { if let Ok(f) = a_div[i].div_to_fraction(b_div[i]) { frac_s[i] = f; } } t.elapsed().as_nanos() };
        let t_g = { let t = Instant::now(); unsafe { let _ = simd_engine::zint_batch_div_to_fraction_simd(&mut frac_g, a_div, b_div, DIV_SIZE); } t.elapsed().as_nanos() };
        verify_results(&frac_s, &frac_g, "DIV_TO_FRAC", t_s, t_g);

        println!("└──────────────┴───────────┴───────────┴─────────┴────────┘");
        println!("\n╔═══════════════════════════════════════════════════════════════════════╗");
        println!("║         ✓ ALL EXTREME SIMD OPERATIONS VERIFIED! 🚀                   ║");
        println!("║           Random data, 16M elements, nanosecond precision            ║");
        println!("╚═══════════════════════════════════════════════════════════════════════╝\n");
    }
}

