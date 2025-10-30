use std::ops::{Add, Sub, Mul, Neg};
use crate::{I32, I64, U64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OIntError {
    Overflow,
    DivisionByZero,
    NotDivisible,
    NoInverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OIFraction {
    pub num: OInt,
    pub den: U64,
}

impl std::fmt::Display for OIFraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) / {}", self.num, self.den)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct OInt {
    pub a: I32,  // scalar
    pub b: I32,  // e1
    pub c: I32,  // e2
    pub d: I32,  // e3
    pub e: I32,  // e4
    pub f: I32,  // e5
    pub g: I32,  // e6
    pub h: I32,  // e7
}

mod fano_plane {
    use super::OInt;

    pub fn multiply_basis(i: usize, j: usize) -> (i32, usize) {
        if i == 0 { return (1, j); }
        if j == 0 { return (1, i); }
        if i == j { return (-1, 0); }

        match (i, j) {
            (1, 2) => (1, 4), (2, 1) => (-1, 4),
            (2, 3) => (1, 5), (3, 2) => (-1, 5),
            (3, 1) => (1, 6), (1, 3) => (-1, 6),
            (1, 4) => (-1, 2), (4, 1) => (1, 2),
            (4, 2) => (1, 1), (2, 4) => (-1, 1),
            (1, 5) => (1, 3), (5, 1) => (-1, 3),
            (5, 3) => (1, 1), (3, 5) => (-1, 1),
            (1, 6) => (-1, 5), (6, 1) => (1, 5),
            (6, 5) => (1, 1), (5, 6) => (-1, 1),
            (1, 7) => (1, 6), (7, 1) => (-1, 6),
            (7, 6) => (1, 1), (6, 7) => (-1, 1),
            (2, 5) => (-1, 7), (5, 2) => (1, 7),
            (2, 6) => (1, 7), (6, 2) => (-1, 7),
            (3, 4) => (1, 7), (4, 3) => (-1, 7),
            (3, 7) => (-1, 4), (7, 3) => (1, 4),
            (4, 5) => (1, 6), (5, 4) => (-1, 6),
            (4, 6) => (-1, 5), (6, 4) => (1, 5),
            (4, 7) => (1, 2), (7, 4) => (-1, 2),
            (5, 7) => (-1, 4), (7, 5) => (1, 4),
            _ => (1, 0),
        }
    }
}

mod num_utils {
    use super::U64;
    pub fn integer_gcd(mut a: U64, mut b: U64) -> U64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}

impl OInt {
    pub fn new(a: I32, b: I32, c: I32, d: I32, e: I32, f: I32, g: I32, h: I32) -> Self {
        OInt { a, b, c, d, e, f, g, h }
    }

    pub fn zero() -> Self {
        OInt::new(0, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn one() -> Self {
        OInt::new(1, 0, 0, 0, 0, 0, 0, 0)
    }

    pub fn e1() -> Self { OInt::new(0, 1, 0, 0, 0, 0, 0, 0) }
    pub fn e2() -> Self { OInt::new(0, 0, 1, 0, 0, 0, 0, 0) }
    pub fn e3() -> Self { OInt::new(0, 0, 0, 1, 0, 0, 0, 0) }
    pub fn e4() -> Self { OInt::new(0, 0, 0, 0, 1, 0, 0, 0) }
    pub fn e5() -> Self { OInt::new(0, 0, 0, 0, 0, 1, 0, 0) }
    pub fn e6() -> Self { OInt::new(0, 0, 0, 0, 0, 0, 1, 0) }
    pub fn e7() -> Self { OInt::new(0, 0, 0, 0, 0, 0, 0, 1) }

    pub fn from_halves(a: I32, b: I32, c: I32, d: I32, e: I32, f: I32, g: I32, h: I32) 
        -> Result<Self, OIntError> {
        Ok(OInt::new(a, b, c, d, e, f, g, h))
    }

    pub fn is_zero(self) -> bool {
        self.a == 0 && self.b == 0 && self.c == 0 && self.d == 0
            && self.e == 0 && self.f == 0 && self.g == 0 && self.h == 0
    }

    pub fn is_unit(self) -> bool {
        self.norm_squared() == 1
    }

    pub fn conj(self) -> Self {
        OInt::new(self.a, -self.b, -self.c, -self.d, -self.e, -self.f, -self.g, -self.h)
    }

    pub fn norm_squared(self) -> U64 {
        let a2: I64 = self.a as I64 * self.a as I64;
        let b2: I64 = self.b as I64 * self.b as I64;
        let c2: I64 = self.c as I64 * self.c as I64;
        let d2: I64 = self.d as I64 * self.d as I64;
        let e2: I64 = self.e as I64 * self.e as I64;
        let f2: I64 = self.f as I64 * self.f as I64;
        let g2: I64 = self.g as I64 * self.g as I64;
        let h2: I64 = self.h as I64 * self.h as I64;
        (a2 + b2 + c2 + d2 + e2 + f2 + g2 + h2) as U64
    }

    pub fn div_rem(self, d: OInt) -> Result<(OInt, OInt), OIntError> {
        if d.is_zero() {
            return Err(OIntError::DivisionByZero);
        }

        let d_norm = d.norm_squared() as I64;
        let d_conj = d.conj();

        let num_prod = self * d_conj;
        let q_a = num_prod.a / (d_norm as I32);
        let q_b = num_prod.b / (d_norm as I32);
        let q_c = num_prod.c / (d_norm as I32);
        let q_d = num_prod.d / (d_norm as I32);
        let q_e = num_prod.e / (d_norm as I32);
        let q_f = num_prod.f / (d_norm as I32);
        let q_g = num_prod.g / (d_norm as I32);
        let q_h = num_prod.h / (d_norm as I32);

        let q = OInt::new(q_a, q_b, q_c, q_d, q_e, q_f, q_g, q_h);
        let r = self - (q * d);

        Ok((q, r))
    }

    pub fn div_exact(self, d: OInt) -> Result<OInt, OIntError> {
        let (q, r) = self.div_rem(d)?;
        if r.is_zero() {
            Ok(q)
        } else {
            Err(OIntError::NotDivisible)
        }
    }

    // ============ FRACTIONS (THE BIG PART!) ============

    pub fn div_to_fraction(self, den: OInt) -> Result<OIFraction, OIntError> {
        if den.is_zero() {
            return Err(OIntError::DivisionByZero);
        }
        Ok(OIFraction {
            num: self,
            den: den.norm_squared(),
        })
    }

    pub fn reduce_fraction(frac: OIFraction) -> OIFraction {
        let g = num_utils::integer_gcd(frac.num.norm_squared(), frac.den);
        OIFraction {
            num: frac.num,
            den: frac.den / g,
        }
    }

    pub fn inv_fraction(self) -> Result<OIFraction, OIntError> {
        if self.is_zero() {
            return Err(OIntError::NoInverse);
        }
        Ok(OIFraction {
            num: self.conj(),
            den: self.norm_squared(),
        })
    }

    pub fn inv_unit(self) -> Result<OInt, OIntError> {
        if !self.is_unit() {
            return Err(OIntError::NoInverse);
        }
        Ok(self.conj())
    }

    // ============ GCD & NORMALIZATION ============

    pub fn gcd(mut a: OInt, mut b: OInt) -> OInt {
        while !b.is_zero() {
            let (_, r) = a.div_rem(b).unwrap_or((OInt::zero(), a));
            a = b;
            b = r;
        }
        a.normalize()
    }

    pub fn normalize(self) -> OInt {
        self
    }

    pub fn associates(self) -> [OInt; 8] {
        [
            self,
            OInt::new(self.a, -self.b, -self.c, -self.d, -self.e, -self.f, -self.g, -self.h),
            OInt::new(self.a, self.b, -self.c, -self.d, self.e, self.f, -self.g, -self.h),
            OInt::new(self.a, self.b, self.c, -self.d, self.e, -self.f, self.g, -self.h),
            OInt::new(self.a, self.b, self.c, self.d, -self.e, -self.f, -self.g, -self.h),
            OInt::new(self.a, self.b, self.c, self.d, self.e, self.f, -self.g, -self.h),
            OInt::new(self.a, self.b, self.c, self.d, self.e, -self.f, self.g, -self.h),
            OInt::new(self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h),
        ]
    }

    pub fn to_float_components(self) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        (
            self.a as f64, self.b as f64, self.c as f64, self.d as f64,
            self.e as f64, self.f as f64, self.g as f64, self.h as f64,
        )
    }

    // ============ ALGEBRA PROPERTIES ============

    pub fn is_non_commutative_pair(a: OInt, b: OInt) -> bool {
        a * b != b * a
    }

    pub fn is_non_associative_triple(a: OInt, b: OInt, c: OInt) -> bool {
        (a * b) * c != a * (b * c)
    }

    pub fn moufang_identity(a: OInt, b: OInt, c: OInt) -> bool {
        ((a * b) * (c * a)) == (a * ((b * c) * a))
    }

    pub fn alternative_identity(a: OInt, b: OInt) -> bool {
        ((a * a) * b == a * (a * b)) && ((a * b) * b == a * (b * b))
    }
}

impl std::fmt::Display for OInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} + {}e₁ + {}e₂ + {}e₃ + {}e₄ + {}e₅ + {}e₆ + {}e₇",
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h
        )
    }
}

impl Add for OInt {
    type Output = OInt;
    fn add(self, other: OInt) -> OInt {
        OInt::new(
            self.a + other.a, self.b + other.b, self.c + other.c, self.d + other.d,
            self.e + other.e, self.f + other.f, self.g + other.g, self.h + other.h,
        )
    }
}

impl Sub for OInt {
    type Output = OInt;
    fn sub(self, other: OInt) -> OInt {
        OInt::new(
            self.a - other.a, self.b - other.b, self.c - other.c, self.d - other.d,
            self.e - other.e, self.f - other.f, self.g - other.g, self.h - other.h,
        )
    }
}

impl Mul for OInt {
    type Output = OInt;
    fn mul(self, other: OInt) -> OInt {
        let mut result = [0i64; 8];
        let sa = [self.a as i64, self.b as i64, self.c as i64, self.d as i64,
                  self.e as i64, self.f as i64, self.g as i64, self.h as i64];
        let oa = [other.a as i64, other.b as i64, other.c as i64, other.d as i64,
                  other.e as i64, other.f as i64, other.g as i64, other.h as i64];

        for i in 0..8 {
            for j in 0..8 {
                let (sign, idx) = fano_plane::multiply_basis(i, j);
                result[idx] += sa[i] * oa[j] * (sign as i64);
            }
        }

        OInt::new(
            result[0] as I32, result[1] as I32, result[2] as I32, result[3] as I32,
            result[4] as I32, result[5] as I32, result[6] as I32, result[7] as I32,
        )
    }
}

impl Neg for OInt {
    type Output = OInt;
    fn neg(self) -> OInt {
        OInt::new(-self.a, -self.b, -self.c, -self.d, -self.e, -self.f, -self.g, -self.h)
    }
}

