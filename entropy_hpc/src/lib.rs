// src/lib.rs

// Global type aliases
pub type I32 = i32;
pub type I64 = i64;
pub type U64 = u64;

// Declare core modules
pub mod simd;
pub mod zint;
pub mod hint;

// Export public types
pub use zint::{ZInt, ZIFraction, ZIntError};
pub use hint::{HInt, HIFraction, HIntError};
pub use simd::simd_engine;

#[cfg(test)]
mod comprehensive_demo {
    use super::*;
    use rand::{Rng, SeedableRng};
    use rand::rngs::StdRng;
    use std::time::Instant;

    #[test]
    fn test_complete_api_showcase() {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  ENTROPY HPC: COMPREHENSIVE API DEMONSTRATION                ║");
        println!("║  Gaussian & Hurwitz Integers with SIMD Acceleration          ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        demo_zint_all();
        demo_hint_all();
        demo_simd_all();
        
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  ALL APIS DEMONSTRATED SUCCESSFULLY! ✅                       ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
    }

    fn demo_zint_all() {
        println!("═══════════════════════════════════════════════════════");
        println!("  ZINT: GAUSSIAN INTEGERS - ALL FUNCTIONS");
        println!("═══════════════════════════════════════════════════════\n");
        
        println!("1. CONSTRUCTORS:");
        let z1 = ZInt::new(3, 4);
        let zero = ZInt::zero();
        let one = ZInt::one();
        let i = ZInt::i();
        println!("  new(3, 4) = {}", z1);
        println!("  zero() = {}", zero);
        println!("  one() = {}", one);
        println!("  i() = {}", i);
        
        println!("\n2. PROPERTIES:");
        println!("  {}.is_zero() = {}", zero, zero.is_zero());
        println!("  {}.is_unit() = {}", i, i.is_unit());
        println!("  {}.norm_squared() = {}", z1, z1.norm_squared());
        
        println!("\n3. ARITHMETIC:");
        let a = ZInt::new(3, 4);
        let b = ZInt::new(1, 2);
        println!("  {} + {} = {}", a, b, a + b);
        println!("  {} - {} = {}", a, b, a - b);
        println!("  {} * {} = {}", a, b, a * b);
        println!("  -{} = {}", a, -a);
        
        println!("\n4. CONJUGATE:");
        println!("  conj({}) = {}", a, a.conj());
        println!("  {} * conj({}) = {}", a, a, a * a.conj());
        
        println!("\n5. EUCLIDEAN DIVISION:");
        let num = ZInt::new(10, 5);
        let den = ZInt::new(3, 2);
        let (q, r) = num.div_rem(den).unwrap();
        println!("  {} = {} * {} + {}", num, q, den, r);
        println!("  N(r)={} < N(d)={}", r.norm_squared(), den.norm_squared());
        
        println!("\n6. EXACT DIVISION:");
        let x = ZInt::new(6, 8);
        let y = ZInt::new(2, 0);
        match x.div_exact(y) {
            Ok(res) => println!("  {} / {} = {}", x, y, res),
            Err(_) => println!("  {} not divisible by {}", x, y),
        }
        
        println!("\n7. FRACTIONS:");
        let frac = a.div_to_fraction(b).unwrap();
        println!("  {}/{}: num={}, den={}", a, b, frac.num, frac.den);
        let reduced = ZInt::reduce_fraction(frac);
        println!("  Reduced: num={}, den={}", reduced.num, reduced.den);
        
        let inv = a.inv_fraction().unwrap();
        println!("  1/{}: num={}, den={}", a, inv.num, inv.den);
        
        let i_inv = i.inv_unit().unwrap();
        println!("  i^(-1) = {}", i_inv);
        
        println!("\n8. GCD:");
        let p = ZInt::new(12, 0);
        let q = ZInt::new(18, 0);
        let g = ZInt::gcd(p, q);
        println!("  gcd({}, {}) = {}", p, q, g);
        
        println!("\n9. EXTENDED GCD:");
        let (gcd, x, y) = ZInt::xgcd(p, q);
        println!("  gcd={}, x={}, y={}", gcd, x, y);
        println!("  {}*{} + {}*{} = {}", p, x, q, y, p*x + q*y);
        
        println!("\n10. NORMALIZE & ASSOCIATES:");
        let w = ZInt::new(-3, 4);
        println!("  normalize({}) = {}", w, w.normalize());
        let assoc = w.associates();
        println!("  associates: {:?}", assoc);
        
        println!();
    }

    fn demo_hint_all() {
        println!("═══════════════════════════════════════════════════════");
        println!("  HINT: HURWITZ QUATERNIONS - ALL FUNCTIONS");
        println!("═══════════════════════════════════════════════════════\n");
        
        println!("1. CONSTRUCTORS:");
        let q1 = HInt::new(1, 2, 3, 4);
        let zero = HInt::zero();
        let one = HInt::one();
        let i = HInt::i();
        let j = HInt::j();
        let k = HInt::k();
        println!("  new(1,2,3,4) = {}", q1);
        println!("  zero() = {}", zero);
        println!("  one() = {}", one);
        println!("  i() = {}", i);
        println!("  j() = {}", j);
        println!("  k() = {}", k);
        
        let h = HInt::from_halves(1, 1, 1, 1).unwrap();
        println!("  from_halves(1,1,1,1) = {}", h);
        
        println!("\n2. PROPERTIES:");
        println!("  {}.is_zero() = {}", zero, zero.is_zero());
        println!("  {}.is_unit() = {}", i, i.is_unit());
        println!("  {}.norm_squared() = {}", q1, q1.norm_squared());
        
        println!("\n3. QUATERNION ALGEBRA:");
        println!("  i² = {}", i * i);
        println!("  j² = {}", j * j);
        println!("  k² = {}", k * k);
        println!("  ijk = {}", i * j * k);
        println!("  ij = {}", i * j);
        println!("  ji = {}", j * i);
        println!("  Anti-commutative? {}", HInt::is_anticommutative_pair(i, j));
        println!("  Associative? {}", HInt::is_associative_triple(i, j, k));
        
        println!("\n4. ARITHMETIC:");
        let a = HInt::new(1, 2, 3, 4);
        let b = HInt::new(5, 6, 7, 8);
        println!("  {} + {} = {}", a, b, a + b);
        println!("  {} - {} = {}", a, b, a - b);
        println!("  {} * {} = {}", a, b, a * b);
        println!("  -{} = {}", a, -a);
        
        println!("\n5. CONJUGATE:");
        println!("  conj({}) = {}", a, a.conj());
        println!("  {} * conj({}) = {}", a, a, a * a.conj());
        println!("  N(a*b) = {}, N(a)*N(b) = {}", 
                 (a*b).norm_squared(), a.norm_squared() * b.norm_squared());
        
        println!("\n6. EUCLIDEAN DIVISION:");
        let num = HInt::new(10, 5, 3, 2);
        let den = HInt::new(2, 1, 0, 0);
        let (q, r) = num.div_rem(den).unwrap();
        println!("  {} = {} * {} + {}", num, q, den, r);
        println!("  N(r)={} < N(d)={}", r.norm_squared(), den.norm_squared());
        
        println!("\n7. EXACT DIVISION:");
        let x = HInt::new(4, 2, 0, 0);
        let y = HInt::new(2, 0, 0, 0);
        match x.div_exact(y) {
            Ok(res) => println!("  {} / {} = {}", x, y, res),
            Err(_) => println!("  {} not divisible by {}", x, y),
        }
        
        println!("\n8. FRACTIONS:");
        let frac = a.div_to_fraction(b).unwrap();
        println!("  {}/{}: num={}, den={}", a, b, frac.num, frac.den);
        let reduced = HInt::reduce_fraction(frac);
        println!("  Reduced: num={}, den={}", reduced.num, reduced.den);
        
        let inv = a.inv_fraction().unwrap();
        println!("  1/{}: num={}, den={}", a, inv.num, inv.den);
        
        let i_inv = i.inv_unit().unwrap();
        println!("  i^(-1) = {}", i_inv);
        
        println!("\n9. GCD:");
        let p = HInt::new(12, 0, 0, 0);
        let q = HInt::new(18, 0, 0, 0);
        let g = HInt::gcd(p, q);
        println!("  gcd({}, {}) = {}", p, q, g);
        
        println!("\n10. FLOAT COMPONENTS:");
        let (fa, fb, fc, fd) = q1.to_float_components();
        println!("  {}: ({}, {}, {}, {})", q1, fa, fb, fc, fd);
        
        println!();
    }

    fn demo_simd_all() {
        println!("═══════════════════════════════════════════════════════");
        println!("  SIMD: BATCH OPERATIONS - ALL FUNCTIONS");
        println!("═══════════════════════════════════════════════════════\n");
        
        println!("1. ZINT SIMD (4 at a time):");
        let za = [ZInt::new(1,2), ZInt::new(3,4), ZInt::new(5,6), ZInt::new(7,8)];
        let zb = [ZInt::new(2,1), ZInt::new(4,3), ZInt::new(6,5), ZInt::new(8,7)];
        
        let z_add = simd_engine::zint_add_batch(&za, &zb);
        println!("  add_batch: {:?}", z_add);
        
        let z_sub = simd_engine::zint_sub_batch(&za, &zb);
        println!("  sub_batch: {:?}", z_sub);
        
        let z_neg = simd_engine::zint_neg_batch(&za);
        println!("  neg_batch: {:?}", z_neg);
        
        let z_conj = simd_engine::zint_conj_batch(&za);
        println!("  conj_batch: {:?}", z_conj);
        
        let z_mul = simd_engine::zint_mul_batch(&za, &zb);
        println!("  mul_batch: {:?}", z_mul);
        
        println!("\n2. HINT SIMD (2 at a time):");
        let ha = [HInt::new(1,2,3,4), HInt::new(5,6,7,8)];
        let hb = [HInt::new(2,1,0,0), HInt::new(1,1,1,1)];
        
        let h_add = simd_engine::hint_add_batch(&ha, &hb);
        println!("  add_batch: {:?}", h_add);
        
        let h_sub = simd_engine::hint_sub_batch(&ha, &hb);
        println!("  sub_batch: {:?}", h_sub);
        
        let h_neg = simd_engine::hint_neg_batch(&ha);
        println!("  neg_batch: {:?}", h_neg);
        
        let h_conj = simd_engine::hint_conj_batch(&ha);
        println!("  conj_batch: {:?}", h_conj);
        
        let h_mul = simd_engine::hint_mul_batch(&ha, &hb);
        println!("  mul_batch: {:?}", h_mul);
        
        println!("\n3. ARRAY OPERATIONS:");
        let size = 10000;
        let mut rng = StdRng::seed_from_u64(42);
        
        let vec_za: Vec<ZInt> = (0..size)
            .map(|_| ZInt::new(rng.gen_range(-100..100), rng.gen_range(-100..100)))
            .collect();
        let vec_zb: Vec<ZInt> = (0..size)
            .map(|_| ZInt::new(rng.gen_range(-100..100), rng.gen_range(-100..100)))
            .collect();
        
        let mut z_result = vec![ZInt::zero(); size];
        
        let start = Instant::now();
        simd_engine::zint_add_arrays(&vec_za, &vec_zb, &mut z_result);
        println!("  zint_add_arrays({}) in {:?}", size, start.elapsed());
        
        let start = Instant::now();
        simd_engine::zint_sub_arrays(&vec_za, &vec_zb, &mut z_result);
        println!("  zint_sub_arrays({}) in {:?}", size, start.elapsed());
        
        let start = Instant::now();
        simd_engine::zint_mul_arrays(&vec_za, &vec_zb, &mut z_result);
        println!("  zint_mul_arrays({}) in {:?}", size, start.elapsed());
        
        let vec_ha: Vec<HInt> = (0..size)
            .map(|_| HInt::new(
                rng.gen_range(-20..20), rng.gen_range(-20..20),
                rng.gen_range(-20..20), rng.gen_range(-20..20)
            ))
            .collect();
        let vec_hb: Vec<HInt> = (0..size)
            .map(|_| HInt::new(
                rng.gen_range(-20..20), rng.gen_range(-20..20),
                rng.gen_range(-20..20), rng.gen_range(-20..20)
            ))
            .collect();
        
        let mut h_result = vec![HInt::zero(); size];
        
        let start = Instant::now();
        simd_engine::hint_add_arrays(&vec_ha, &vec_hb, &mut h_result);
        println!("  hint_add_arrays({}) in {:?}", size, start.elapsed());
        
        let start = Instant::now();
        simd_engine::hint_sub_arrays(&vec_ha, &vec_hb, &mut h_result);
        println!("  hint_sub_arrays({}) in {:?}", size, start.elapsed());
        
        let start = Instant::now();
        simd_engine::hint_mul_arrays(&vec_ha, &vec_hb, &mut h_result);
        println!("  hint_mul_arrays({}) in {:?}", size, start.elapsed());
        
        println!();
    }
}

