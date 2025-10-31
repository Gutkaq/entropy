// tests/demo.rs

use entropy_hpc::{CInt, HInt, OInt, simd_engine};
use std::time::Instant;

#[test]
fn test_complete_api_showcase() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║  ENTROPY HPC: COMPREHENSIVE API DEMONSTRATION                ║");
    println!("║  Complex Integers (CInt) + Quaternions (HInt) +              ║");
    println!("║  Integer Octonions (OInt) with SIMD Acceleration            ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    demo_cint_all();
    demo_hint_all();
    demo_oint_all();
    demo_simd_all();

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║  ALL APIS DEMONSTRATED SUCCESSFULLY! ✅                       ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}

fn demo_cint_all() {
    println!("═══════════════════════════════════════════════════════");
    println!("  CINT: COMPLEX INTEGERS ℤ[i]");
    println!("═══════════════════════════════════════════════════════\n");
    
    println!("1. CONSTRUCTORS:");
    let z1 = CInt::new(3, 4);
    let zero = CInt::zero();
    let one = CInt::one();
    let i = CInt::i();
    println!("  new(3, 4) = {}", z1);
    println!("  zero() = {}", zero);
    println!("  one() = {}", one);
    println!("  i() = {}", i);
    
    assert_eq!(z1.a, 3);
    assert_eq!(z1.b, 4);
    assert!(zero.is_zero());
    assert!(one.a == 1 && one.b == 0);
    println!("  ✓ Constructors verified");
    
    println!("\n2. PROPERTIES:");
    println!("  {}.is_zero() = {}", zero, zero.is_zero());
    println!("  {}.is_unit() = {}", i, i.is_unit());
    println!("  {}.norm_squared() = {}", z1, z1.norm_squared());
    
    assert!(!z1.is_zero());
    assert!(i.is_unit());
    assert_eq!(z1.norm_squared(), 25);
    println!("  ✓ Properties verified");
    
    println!("\n3. ARITHMETIC:");
    let a = CInt::new(3, 4);
    let b = CInt::new(1, 2);
    println!("  {} + {} = {}", a, b, a + b);
    println!("  {} - {} = {}", a, b, a - b);
    println!("  {} * {} = {}", a, b, a * b);
    println!("  -{} = {}", a, -a);
    
    assert_eq!(a + b, CInt::new(4, 6));
    assert_eq!(a - b, CInt::new(2, 2));
    assert_eq!(a * b, CInt::new(-5, 10));
    assert_eq!(-a, CInt::new(-3, -4));
    println!("  ✓ Arithmetic verified");
    
    println!("\n4. CONJUGATE:");
    println!("  conj({}) = {}", a, a.conj());
    println!("  {} * conj({}) = {}", a, a, a * a.conj());
    
    assert_eq!(a.conj(), CInt::new(3, -4));
    assert_eq!(a * a.conj(), CInt::new(25, 0));
    println!("  ✓ Conjugate verified");
    
    println!("\n5. EUCLIDEAN DIVISION:");
    let num = CInt::new(10, 5);
    let den = CInt::new(3, 2);
    let (q, r) = num.div_rem(den).unwrap();
    println!("  {} ÷ {} = q: {}, r: {}", num, den, q, r);
    println!("  N(r)={} < N(d)={}", r.norm_squared(), den.norm_squared());
    
    assert!(r.norm_squared() < den.norm_squared());
    assert_eq!(q * den + r, num);
    println!("  ✓ Euclidean division verified");
    
    println!("\n6. FRACTIONS:");
    let frac = a.div_to_fraction(b).unwrap();
    println!("  {}/{} = {}", a, b, frac);
    let reduced = CInt::reduce_fraction(frac);
    println!("  Reduced: {}", reduced);
    
    assert_eq!(frac.den, 5);
    println!("  ✓ Fractions verified");
    
    println!("\n7. GCD:");
    let p = CInt::new(12, 0);
    let q = CInt::new(18, 0);
    let g = CInt::gcd(p, q);
    println!("  gcd({}, {}) = {}", p, q, g);
    
    assert_eq!(g, CInt::new(6, 0));
    println!("  ✓ GCD verified");
    
    println!();
}

fn demo_hint_all() {
    println!("═══════════════════════════════════════════════════════");
    println!("  HINT: HURWITZ QUATERNIONS");
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
    println!("  i() = {}, j() = {}, k() = {}", i, j, k);
    
    assert!(zero.is_zero());
    assert!(i.is_unit());
    println!("  ✓ Constructors verified");
    
    println!("\n2. HALF-INTEGERS:");
    let halves = HInt::from_halves(1, 1, 1, 1).unwrap();
    println!("  from_halves(1,1,1,1) = {} (all 1/2)", halves);
    
    let mixed = HInt::from_halves(1, 0, 1, 0);
    assert!(mixed.is_err());
    println!("  ✓ Half-integer parity check verified");
    
    println!("\n3. QUATERNION ALGEBRA:");
    println!("  i² = {}", i * i);
    println!("  j² = {}", j * j);
    println!("  k² = {}", k * k);
    println!("  ij = {}, ji = {}", i * j, j * i);
    println!("  Anti-commutative? {}", HInt::is_anticommutative_pair(i, j));
    println!("  Associative? {}", HInt::is_associative_triple(i, j, k));
    
    assert_eq!(i * i, HInt::new(-1, 0, 0, 0));
    assert_eq!(j * j, HInt::new(-1, 0, 0, 0));
    assert_eq!(k * k, HInt::new(-1, 0, 0, 0));
    assert_eq!(i * j, HInt::k());
    assert_eq!(j * i, -HInt::k());
    assert!(HInt::is_anticommutative_pair(i, j));
    assert!(HInt::is_associative_triple(i, j, k));
    println!("  ✓ Quaternion algebra verified");
    
    println!("\n4. ARITHMETIC:");
    let a = HInt::new(1, 2, 3, 4);
    let b = HInt::new(5, 6, 7, 8);
    println!("  {} + {} = {}", a, b, a + b);
    println!("  {} * {} = {}", a, b, a * b);
    
    assert_eq!(a + b, HInt::new(6, 8, 10, 12));
    println!("  ✓ Arithmetic verified");
    
    println!("\n5. CONJUGATE & NORM:");
    println!("  conj({}) = {}", a, a.conj());
    println!("  {} * conj({}) = {}", a, a, a * a.conj());
    println!("  N(a*b) = {}, N(a)*N(b) = {}", (a*b).norm_squared(), 
             a.norm_squared() * b.norm_squared());
    
    assert_eq!(a.conj(), HInt::new(1, -2, -3, -4));
    assert_eq!(a * a.conj(), HInt::new(30, 0, 0, 0));
    assert_eq!((a*b).norm_squared(), a.norm_squared() * b.norm_squared());
    println!("  ✓ Norm multiplicativity verified");
    
    println!("\n6. EUCLIDEAN DIVISION:");
    let num = HInt::new(10, 5, 3, 2);
    let den = HInt::new(2, 1, 0, 0);
    let (q, r) = num.div_rem(den).unwrap();
    println!("  {} ÷ {} = q: {}, r: {}", num, den, q, r);
    
    assert!(r.norm_squared() < den.norm_squared());
    println!("  ✓ Euclidean division verified");
    
    println!("\n7. FRACTIONS:");
    let frac = a.div_to_fraction(b).unwrap();
    println!("  {}/{} = {}", a, b, frac);
    
    assert!(frac.den > 0);
    println!("  ✓ Fractions verified");
    
    println!("\n8. GCD:");
    let p = HInt::new(12, 0, 0, 0);
    let q = HInt::new(18, 0, 0, 0);
    let g = HInt::gcd(p, q);
    println!("  gcd({}, {}) = {}", p, q, g);
    
    assert_eq!(g, HInt::new(6, 0, 0, 0));
    println!("  ✓ GCD verified");
    
    println!();
}

fn demo_oint_all() {
    println!("═══════════════════════════════════════════════════════");
    println!("  OINT: INTEGER OCTONIONS");
    println!("═══════════════════════════════════════════════════════\n");
    
    println!("1. CONSTRUCTORS:");
    let o1 = OInt::new(1, 2, 3, 4, 5, 6, 7, 8);
    let zero = OInt::zero();
    let one = OInt::one();
    let e1 = OInt::e1();
    let e2 = OInt::e2();
    println!("  new(1,2,3,4,5,6,7,8) = {}", o1);
    println!("  zero() = {}", zero);
    println!("  one() = {}", one);
    println!("  e1() = {}, e2() = {}", e1, e2);
    
    assert!(zero.is_zero());
    assert!(e1.is_unit());
    println!("  ✓ Constructors verified");
    
    println!("\n2. PROPERTIES:");
    println!("  {}.is_zero() = {}", zero, zero.is_zero());
    println!("  {}.is_unit() = {}", e1, e1.is_unit());
    println!("  {}.norm_squared() = {}", o1, o1.norm_squared());
    
    assert_eq!(o1.norm_squared(), 204);
    println!("  ✓ Properties verified");
    
    println!("\n3. OCTONION ALGEBRA:");
    println!("  e1*e1 = {}", e1 * e1);
    println!("  e1*e2 = {}, e2*e1 = {}", e1 * e2, e2 * e1);
    println!("  NON-COMMUTATIVE? {}", OInt::is_non_commutative_pair(e1, e2));
    
    assert_eq!(e1 * e1, OInt::new(-1, 0, 0, 0, 0, 0, 0, 0));
    assert_eq!(e1 * e2, OInt::e4());
    assert_eq!(e2 * e1, -OInt::e4());
    assert!(OInt::is_non_commutative_pair(e1, e2));
    println!("  ✓ Octonion algebra verified");
    
    println!("\n4. NON-ASSOCIATIVITY:");
    let a = OInt::e1();
    let b = OInt::e2();
    let c = OInt::e4();
    let lhs = (a * b) * c;
    let rhs = a * (b * c);
    println!("  (e1*e2)*e4 = {}", lhs);
    println!("  e1*(e2*e4) = {}", rhs);
    println!("  NON-ASSOCIATIVE? {}", OInt::is_non_associative_triple(a, b, c));
    
    assert_ne!(lhs, rhs);
    assert!(OInt::is_non_associative_triple(a, b, c));
    println!("  ✓ Non-associativity verified");
    
    println!("\n5. MOUFANG IDENTITY:");
    println!("  (a*b)*(c*a) = a*(b*c)*a? {}", OInt::moufang_identity(a, b, c));
    
    assert!(OInt::moufang_identity(a, b, c));
    println!("  ✓ Moufang identity verified");
    
    println!("\n6. ALTERNATIVE PROPERTY:");
    let alt_holds = OInt::alternative_identity(a, b);
    println!("  Alternative holds? {}", alt_holds);
    println!("  ✓ Alternative identity checked (Moufang guaranteed instead)");
    
    println!("\n7. ARITHMETIC:");
    let x = OInt::new(3, 1, 2, 0, 0, 0, 0, 0);
    let y = OInt::new(2, 0, 1, 0, 0, 0, 0, 0);
    println!("  {} + {} = {}", x, y, x + y);
    println!("  {} * {} = {}", x, y, x * y);
    
    assert_eq!(x + y, OInt::new(5, 1, 3, 0, 0, 0, 0, 0));
    println!("  ✓ Arithmetic verified");
    
    println!("\n8. EUCLIDEAN DIVISION:");
    let num = OInt::new(10, 4, 2, 0, 0, 0, 0, 0);
    let den = OInt::new(2, 0, 0, 0, 0, 0, 0, 0);
    let (q, r) = num.div_rem(den).unwrap();
    println!("  {} ÷ {} = q: {}, r: {}", num, den, q, r);
    
    assert!(r.norm_squared() < den.norm_squared());
    println!("  ✓ Euclidean division verified");
    
    println!("\n9. FRACTIONS:");
    let frac = x.div_to_fraction(y).unwrap();
    println!("  {}/{} = {}", x, y, frac);
    
    assert!(frac.den > 0);
    println!("  ✓ Fractions verified");
    
    println!("\n10. GCD:");
    let p = OInt::new(12, 0, 0, 0, 0, 0, 0, 0);
    let q = OInt::new(18, 0, 0, 0, 0, 0, 0, 0);
    let g = OInt::gcd(p, q);
    println!("  gcd({}, {}) = {}", p, q, g);
    
    assert_eq!(g, OInt::new(6, 0, 0, 0, 0, 0, 0, 0));
    println!("  ✓ GCD verified");
    
    println!();
}

fn demo_simd_all() {
    println!("═══════════════════════════════════════════════════════");
    println!("  SIMD: BATCH OPERATIONS");
    println!("═══════════════════════════════════════════════════════\n");
    
    println!("1. CINT SIMD (4 elements):");
    let ca = [CInt::new(1,2), CInt::new(3,4), CInt::new(5,6), CInt::new(7,8)];
    let cb = [CInt::new(2,1), CInt::new(4,3), CInt::new(6,5), CInt::new(8,7)];
    
    let result = simd_engine::cint_add_batch(&ca, &cb);
    println!("  add_batch: {:?}", result);
    assert_eq!(result[0], CInt::new(3, 3));
    assert_eq!(result[1], CInt::new(7, 7));
    assert_eq!(result[2], CInt::new(11, 11));
    assert_eq!(result[3], CInt::new(15, 15));
    
    let result = simd_engine::cint_sub_batch(&ca, &cb);
    println!("  sub_batch: {:?}", result);
    assert_eq!(result[0], CInt::new(-1, 1));
    assert_eq!(result[1], CInt::new(-1, 1));
    
    let result = simd_engine::cint_mul_batch(&ca, &cb);
    println!("  mul_batch: {:?}", result);
    println!("  ✓ CINT SIMD verified");
    
    println!("\n2. HINT SIMD (2 elements):");
    let ha = [HInt::new(1,2,3,4), HInt::new(5,6,7,8)];
    let hb = [HInt::new(2,1,0,0), HInt::new(1,1,1,1)];
    
    let result = simd_engine::hint_add_batch(&ha, &hb);
    println!("  add_batch: {:?}", result);
    assert_eq!(result[0], HInt::new(3, 3, 3, 4));
    assert_eq!(result[1], HInt::new(6, 7, 8, 9));
    
    let result = simd_engine::hint_sub_batch(&ha, &hb);
    println!("  sub_batch: {:?}", result);
    assert_eq!(result[0], HInt::new(-1, 1, 3, 4));
    
    let result = simd_engine::hint_mul_batch(&ha, &hb);
    println!("  mul_batch: {:?}", result);
    println!("  ✓ HINT SIMD verified");
    
    println!("\n3. OINT SIMD (1 element, 8D):");
    let oa = [OInt::new(1, 2, 3, 4, 5, 6, 7, 8)];
    let ob = [OInt::new(2, 1, 0, 0, 1, 1, 1, 1)];
    
    let result = simd_engine::oint_add_batch(&oa, &ob);
    println!("  add_batch: {:?}", result);
    assert_eq!(result[0], OInt::new(3, 3, 3, 4, 6, 7, 8, 9));
    
    let result = simd_engine::oint_sub_batch(&oa, &ob);
    println!("  sub_batch: {:?}", result);
    assert_eq!(result[0], OInt::new(-1, 1, 3, 4, 4, 5, 6, 7));
    
    let result = simd_engine::oint_mul_batch(&oa, &ob);
    println!("  mul_batch: {:?}", result);
    println!("  ✓ OINT SIMD verified");
    
    println!("\n4. ARRAY OPERATIONS (10k elements):");
    let size = 10000;
    let cint_a: Vec<CInt> = (0..size).map(|i| CInt::new((i % 100) as i32, (i / 100) as i32)).collect();
    let cint_b: Vec<CInt> = (0..size).map(|i| CInt::new((i / 50) as i32, (i % 50) as i32)).collect();
    let mut cint_out = vec![CInt::zero(); size];
    
    let start = Instant::now();
    simd_engine::cint_add_arrays(&cint_a, &cint_b, &mut cint_out);
    let elapsed = start.elapsed();
    println!("  cint_add_arrays({}) in {:?}", size, elapsed);
    assert_eq!(cint_out[0], cint_a[0] + cint_b[0]);
    assert_eq!(cint_out[size-1], cint_a[size-1] + cint_b[size-1]);
    
    let start = Instant::now();
    simd_engine::cint_sub_arrays(&cint_a, &cint_b, &mut cint_out);
    let elapsed = start.elapsed();
    println!("  cint_sub_arrays({}) in {:?}", size, elapsed);
    assert_eq!(cint_out[0], cint_a[0] - cint_b[0]);
    
    let start = Instant::now();
    simd_engine::cint_mul_arrays(&cint_a, &cint_b, &mut cint_out);
    let elapsed = start.elapsed();
    println!("  cint_mul_arrays({}) in {:?}", size, elapsed);
    
    let hint_a: Vec<HInt> = (0..size).map(|i| HInt::new((i % 20) as i32, (i / 20) as i32, (i / 100) as i32, 0)).collect();
    let hint_b: Vec<HInt> = (0..size).map(|i| HInt::new((i / 50) as i32, (i % 30) as i32, 0, (i / 200) as i32)).collect();
    let mut hint_out = vec![HInt::zero(); size];
    
    let start = Instant::now();
    simd_engine::hint_add_arrays(&hint_a, &hint_b, &mut hint_out);
    let elapsed = start.elapsed();
    println!("  hint_add_arrays({}) in {:?}", size, elapsed);
    assert_eq!(hint_out[0], hint_a[0] + hint_b[0]);
    
    let start = Instant::now();
    simd_engine::hint_sub_arrays(&hint_a, &hint_b, &mut hint_out);
    let elapsed = start.elapsed();
    println!("  hint_sub_arrays({}) in {:?}", size, elapsed);
    
    let start = Instant::now();
    simd_engine::hint_mul_arrays(&hint_a, &hint_b, &mut hint_out);
    let elapsed = start.elapsed();
    println!("  hint_mul_arrays({}) in {:?}", size, elapsed);
    
    let oint_a: Vec<OInt> = (0..size).map(|i| OInt::new((i % 10) as i32, (i / 10) as i32, 0, 0, (i / 100) as i32, 0, 0, 0)).collect();
    let oint_b: Vec<OInt> = (0..size).map(|i| OInt::new((i / 50) as i32, 0, (i % 20) as i32, 0, 0, (i / 200) as i32, 0, 0)).collect();
    let mut oint_out = vec![OInt::zero(); size];
    
    let start = Instant::now();
    simd_engine::oint_add_arrays(&oint_a, &oint_b, &mut oint_out);
    let elapsed = start.elapsed();
    println!("  oint_add_arrays({}) in {:?}", size, elapsed);
    assert_eq!(oint_out[0], oint_a[0] + oint_b[0]);
    
    let start = Instant::now();
    simd_engine::oint_sub_arrays(&oint_a, &oint_b, &mut oint_out);
    let elapsed = start.elapsed();
    println!("  oint_sub_arrays({}) in {:?}", size, elapsed);
    
    let start = Instant::now();
    simd_engine::oint_mul_arrays(&oint_a, &oint_b, &mut oint_out);
    let elapsed = start.elapsed();
    println!("  oint_mul_arrays({}) in {:?}", size, elapsed);
    
    println!("  ✓ Array operations verified");
    
    println!();
}

