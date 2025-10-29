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
mod comprehensive_tests {
    use super::*;
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;

    // ========== SIMD vs SCALAR CORRECTNESS ==========

    #[test]
    fn test_simd_scalar_equivalence_add() {
        let mut rng = StdRng::seed_from_u64(42);
        
        for _ in 0..100 {
            let a: [ZInt; 4] = [
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
            ];
            let b: [ZInt; 4] = [
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
            ];
            
            let simd_result = simd_engine::add_batch(&a, &b);
            let scalar_result = [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]];
            
            assert_eq!(simd_result, scalar_result, "SIMD add differs from scalar");
        }
    }

    #[test]
    fn test_simd_scalar_equivalence_sub() {
        let mut rng = StdRng::seed_from_u64(43);
        
        for _ in 0..100 {
            let a: [ZInt; 4] = [
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
            ];
            let b: [ZInt; 4] = [
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
            ];
            
            let simd_result = simd_engine::sub_batch(&a, &b);
            let scalar_result = [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]];
            
            assert_eq!(simd_result, scalar_result, "SIMD sub differs from scalar");
        }
    }

    #[test]
    fn test_simd_scalar_equivalence_mul() {
        let mut rng = StdRng::seed_from_u64(44);
        
        for _ in 0..100 {
            let a: [ZInt; 4] = [
                ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000)),
                ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000)),
                ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000)),
                ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000)),
            ];
            let b: [ZInt; 4] = [
                ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000)),
                ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000)),
                ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000)),
                ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000)),
            ];
            
            let simd_result = simd_engine::mul_batch(&a, &b);
            let scalar_result = [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]];
            
            assert_eq!(simd_result, scalar_result, "SIMD mul differs from scalar for a={:?}, b={:?}", a, b);
        }
    }

    #[test]
    fn test_simd_scalar_equivalence_conj() {
        let mut rng = StdRng::seed_from_u64(45);
        
        for _ in 0..100 {
            let a: [ZInt; 4] = [
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
            ];
            
            let simd_result = simd_engine::conj_batch(&a);
            let scalar_result = [a[0].conj(), a[1].conj(), a[2].conj(), a[3].conj()];
            
            assert_eq!(simd_result, scalar_result, "SIMD conj differs from scalar");
        }
    }

    #[test]
    fn test_simd_scalar_equivalence_neg() {
        let mut rng = StdRng::seed_from_u64(46);
        
        for _ in 0..100 {
            let a: [ZInt; 4] = [
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
                ZInt::new(rng.gen(), rng.gen()),
            ];
            
            let simd_result = simd_engine::neg_batch(&a);
            let scalar_result = [-a[0], -a[1], -a[2], -a[3]];
            
            assert_eq!(simd_result, scalar_result, "SIMD neg differs from scalar");
        }
    }

    // ========== ARRAY OPERATIONS STRESS TESTS ==========

    #[test]
    fn test_add_arrays_large() {
        let size = 10000;
        let mut rng = StdRng::seed_from_u64(100);
        
        let a: Vec<ZInt> = (0..size)
            .map(|_| ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000)))
            .collect();
        let b: Vec<ZInt> = (0..size)
            .map(|_| ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000)))
            .collect();
        let mut out = vec![ZInt::zero(); size];
        
        simd_engine::add_arrays(&a, &b, &mut out);
        
        for i in 0..size {
            assert_eq!(out[i], a[i] + b[i], "Mismatch at index {}", i);
        }
    }

    #[test]
    fn test_mul_arrays_large() {
        let size = 10000;
        let mut rng = StdRng::seed_from_u64(101);
        
        let a: Vec<ZInt> = (0..size)
            .map(|_| ZInt::new(rng.gen_range(-100..100), rng.gen_range(-100..100)))
            .collect();
        let b: Vec<ZInt> = (0..size)
            .map(|_| ZInt::new(rng.gen_range(-100..100), rng.gen_range(-100..100)))
            .collect();
        let mut out = vec![ZInt::zero(); size];
        
        simd_engine::mul_arrays(&a, &b, &mut out);
        
        for i in 0..size {
            assert_eq!(out[i], a[i] * b[i], "Mismatch at index {}", i);
        }
    }

    #[test]
    fn test_array_ops_edge_sizes() {
        // Test various sizes including non-multiple-of-4
        for size in [1, 3, 4, 5, 7, 8, 15, 16, 17, 100] {
            let a: Vec<ZInt> = (0..size).map(|i| ZInt::new(i as i32, i as i32 * 2)).collect();
            let b: Vec<ZInt> = (0..size).map(|i| ZInt::new(i as i32 * 3, i as i32 * 4)).collect();
            let mut out = vec![ZInt::zero(); size];
            
            simd_engine::add_arrays(&a, &b, &mut out);
            
            for i in 0..size {
                let expected = a[i] + b[i];
                assert_eq!(out[i], expected, "Size {} failed at index {}", size, i);
            }
        }
    }

    // ========== MATHEMATICAL PROPERTIES ==========

    #[test]
    fn test_complex_multiplication_properties() {
        let mut rng = StdRng::seed_from_u64(200);
        
        for _ in 0..1000 {
            let a = ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000));
            let b = ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000));
            let c = ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000));
            
            // Commutativity: a * b = b * a
            assert_eq!(a * b, b * a, "Multiplication not commutative");
            
            // Associativity: (a * b) * c = a * (b * c)
            assert_eq!((a * b) * c, a * (b * c), "Multiplication not associative");
            
            // Distributivity: a * (b + c) = a*b + a*c
            assert_eq!(a * (b + c), a * b + a * c, "Multiplication not distributive");
        }
    }

    #[test]
    fn test_norm_multiplicative_property() {
        let mut rng = StdRng::seed_from_u64(201);
        
        for _ in 0..1000 {
            let a = ZInt::new(rng.gen_range(-100..100), rng.gen_range(-100..100));
            let b = ZInt::new(rng.gen_range(-100..100), rng.gen_range(-100..100));
            
            let product = a * b;
            
            // Norm is multiplicative: N(a*b) = N(a) * N(b)
            assert_eq!(
                product.norm_squared(),
                a.norm_squared() * b.norm_squared(),
                "Norm not multiplicative for a={:?}, b={:?}",
                a, b
            );
        }
    }

    #[test]
    fn test_conjugate_properties() {
        let mut rng = StdRng::seed_from_u64(202);
        
        for _ in 0..1000 {
            let a = ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000));
            let b = ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000));
            
            // Double conjugate is identity
            assert_eq!(a.conj().conj(), a, "Double conjugate failed");
            
            // Conjugate is linear: conj(a + b) = conj(a) + conj(b)
            assert_eq!((a + b).conj(), a.conj() + b.conj(), "Conjugate not linear (add)");
            
            // Conjugate is multiplicative: conj(a * b) = conj(a) * conj(b)
            assert_eq!((a * b).conj(), a.conj() * b.conj(), "Conjugate not multiplicative");
        }
    }

    // ========== DIVISION ALGORITHM PROPERTIES ==========

    #[test]
    fn test_euclidean_division_stress() {
        let mut rng = StdRng::seed_from_u64(300);
        
        for _ in 0..1000 {
            let a = ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000));
            let d = ZInt::new(rng.gen_range(-1000..1000), rng.gen_range(-1000..1000));
            
            if d.is_zero() {
                continue;
            }
            
            let (q, r) = a.div_rem(d).unwrap();
            
            // Division identity: a = q*d + r
            assert_eq!(a, q * d + r, "Division identity failed for a={:?}, d={:?}", a, d);
            
            // Euclidean property: N(r) < N(d)
            assert!(
                r.norm_squared() < d.norm_squared(),
                "Euclidean property failed: N(r)={} >= N(d)={}",
                r.norm_squared(),
                d.norm_squared()
            );
        }
    }

    #[test]
    fn test_gcd_properties() {
        let mut rng = StdRng::seed_from_u64(400);
        
        for _ in 0..100 {
            let a = ZInt::new(rng.gen_range(-500..500), rng.gen_range(-500..500));
            let b = ZInt::new(rng.gen_range(-500..500), rng.gen_range(-500..500));
            
            if a.is_zero() || b.is_zero() {
                continue;
            }
            
            let g = ZInt::gcd(a, b);
            
            // GCD divides both a and b
            assert!(a.div_exact(g).is_ok(), "GCD doesn't divide a");
            assert!(b.div_exact(g).is_ok(), "GCD doesn't divide b");
            
            // Symmetry: gcd(a, b) = gcd(b, a)
            assert_eq!(ZInt::gcd(a, b).norm_squared(), ZInt::gcd(b, a).norm_squared());
            
            // Identity: gcd(a, 0) = a (up to normalization)
            assert_eq!(ZInt::gcd(a, ZInt::zero()), a.normalize());
        }
    }

    #[test]
    fn test_xgcd_bezout_identity() {
        let mut rng = StdRng::seed_from_u64(500);
        
        for _ in 0..100 {
            let a = ZInt::new(rng.gen_range(-500..500), rng.gen_range(-500..500));
            let b = ZInt::new(rng.gen_range(-500..500), rng.gen_range(-500..500));
            
            if a.is_zero() || b.is_zero() {
                continue;
            }
            
            let (gcd, s, t) = ZInt::xgcd(a, b);
            let bezout = s * a + t * b;
            
            // Bézout identity (up to units): s*a + t*b is associate of gcd
            assert_eq!(
                bezout.norm_squared(),
                gcd.norm_squared(),
                "Bézout identity failed for a={:?}, b={:?}",
                a, b
            );
            
            // GCD divides both inputs
            assert!(a.div_rem(gcd).is_ok());
            assert!(b.div_rem(gcd).is_ok());
        }
    }

    // ========== EDGE CASES ==========

    #[test]
    fn test_zero_operations() {
        let z = ZInt::zero();
        let a = ZInt::new(5, 3);
        
        assert_eq!(z + a, a);
        assert_eq!(a + z, a);
        assert_eq!(z * a, z);
        assert_eq!(a * z, z);
        assert_eq!(z - a, -a);
        assert_eq!(a - z, a);
        assert_eq!(-z, z);
        assert_eq!(z.conj(), z);
        assert_eq!(z.norm_squared(), 0);
        assert!(z.is_zero());
        assert!(!z.is_unit());
    }

    #[test]
    fn test_unit_operations() {
        let units = [
            ZInt::new(1, 0),
            ZInt::new(-1, 0),
            ZInt::new(0, 1),
            ZInt::new(0, -1),
        ];
        
        for &u in &units {
            assert!(u.is_unit(), "{:?} should be a unit", u);
            assert_eq!(u.norm_squared(), 1);
            assert!(u.inv_unit().is_ok(), "Unit should have inverse");
            
            let inv = u.inv_unit().unwrap();
            assert_eq!(u * inv, ZInt::one(), "Unit * inverse should be 1");
        }
    }

    #[test]
    fn test_boundary_values() {
        let max_val = ZInt::new(i32::MAX, i32::MAX);
        let min_val = ZInt::new(i32::MIN, i32::MIN);
        
        // These should not panic (wrapping arithmetic for add/sub)
        let _ = max_val + ZInt::one();
        let _ = min_val - ZInt::one();
        
        // Norm should still work
        let _ = max_val.norm_squared();
        let _ = min_val.norm_squared();
    }

    // ========== PERFORMANCE SANITY CHECKS ==========

    #[test]
    fn test_simd_performance_sanity() {
        use std::time::Instant;
        
        let size = 100000;
        let mut rng = StdRng::seed_from_u64(999);
        
        let a: Vec<ZInt> = (0..size)
            .map(|_| ZInt::new(rng.gen_range(-100..100), rng.gen_range(-100..100)))
            .collect();
        let b: Vec<ZInt> = (0..size)
            .map(|_| ZInt::new(rng.gen_range(-100..100), rng.gen_range(-100..100)))
            .collect();
        
        // SIMD version
        let mut out_simd = vec![ZInt::zero(); size];
        let start = Instant::now();
        simd_engine::mul_arrays(&a, &b, &mut out_simd);
        let simd_time = start.elapsed();
        
        // Scalar version
        let mut out_scalar = vec![ZInt::zero(); size];
        let start = Instant::now();
        for i in 0..size {
            out_scalar[i] = a[i] * b[i];
        }
        let scalar_time = start.elapsed();
        
        println!("SIMD time: {:?}, Scalar time: {:?}", simd_time, scalar_time);
        println!("Speedup: {:.2}x", scalar_time.as_secs_f64() / simd_time.as_secs_f64());
        
        // Verify results match
        assert_eq!(out_simd, out_scalar, "SIMD and scalar results differ");
        
        // Sanity check: SIMD shouldn't be slower (allow some variance)
        // Comment this out if running on systems without AVX2
        // assert!(simd_time < scalar_time * 2, "SIMD is suspiciously slow");
    }

    // ========== REGRESSION TESTS ==========

    #[test]
    fn test_known_good_values() {
        // Test specific known-good computations
        assert_eq!(ZInt::new(3, 4).norm_squared(), 25);
        assert_eq!(ZInt::new(5, 12).norm_squared(), 169);
        
        let z1 = ZInt::new(1, 2);
        let z2 = ZInt::new(3, 4);
        assert_eq!(z1 * z2, ZInt::new(-5, 10));
        
        assert_eq!(ZInt::new(10, 0).div_exact(ZInt::new(2, 0)).unwrap(), ZInt::new(5, 0));
        
        let (gcd, _, _) = ZInt::xgcd(ZInt::new(10, 0), ZInt::new(6, 0));
        assert_eq!(gcd.norm_squared(), 4);
    }
}

