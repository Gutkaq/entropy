use std::ops::{Add, Sub, Mul, Neg};

use crate::{I32, I64, U64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZIntError {
    Overflow,
    DivisionByZero,
    NotDivisible,
    NoInverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZIFraction {
    pub num: ZInt,
    pub den: U64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ZInt {
    pub a: I32,
    pub b: I32,
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

impl ZInt {
    pub fn new(a: I32, b: I32) -> Self {
        ZInt { a, b }
    }

    pub fn zero() -> Self {
        ZInt::new(0, 0)
    }

    pub fn one() -> Self {
        ZInt::new(1, 0)
    }

    pub fn i() -> Self {
        ZInt::new(0, 1)
    }

    pub fn is_zero(self) -> bool {
        self.a == 0 && self.b == 0
    }

    pub fn is_unit(self) -> bool {
        self.norm_squared() == 1
    }

    pub fn conj(self) -> Self {
        ZInt { a: self.a, b: -self.b }
    }

    pub fn norm_squared(self) -> U64 {
        let a2: I64 = self.a as I64 * self.a as I64;
        let b2: I64 = self.b as I64 * self.b as I64;
        (a2 + b2) as U64
    }

    pub fn associates(self) -> [Self; 4] {
        [
            self,
            Self::new(-self.b, self.a),
            Self::new(-self.a, -self.b),
            Self::new(self.b, -self.a),
        ]
    }

    pub fn normalize(self) -> Self {
        if self.is_zero() {
            return self;
        }
        
        if self.a == 0 && self.b != 0 {
            return Self::new(self.b.abs(), 0);
        }
        
        if self.a > 0 && self.b >= 0 {
            return self;
        }
        let assocs = self.associates();
        for candidate in &assocs {
            if candidate.a > 0 && candidate.b >= 0 {
                return *candidate;
            }
        }
        for candidate in &assocs {
            if candidate.a > 0 {
                return *candidate;
            }
        }
        assocs[0]
    }

    pub fn div_rem(self, d: Self) -> Result<(Self, Self), ZIntError> {
        if d.is_zero() {
            return Err(ZIntError::DivisionByZero);
        }

        let norm_d = d.norm_squared() as I64;
        let d_conj = d.conj();
        
        let num_a = self.a as I64 * d_conj.a as I64 - self.b as I64 * d_conj.b as I64;
        let num_b = self.a as I64 * d_conj.b as I64 + self.b as I64 * d_conj.a as I64;

        let q_real_f = num_a as f64 / norm_d as f64;
        let q_imag_f = num_b as f64 / norm_d as f64;

        let q_real = q_real_f.round() as I32;
        let q_imag = q_imag_f.round() as I32;
        let q = ZInt::new(q_real, q_imag);

        let r = self - (q * d);
        Ok((q, r))
    }

    pub fn div_exact(self, d: Self) -> Result<Self, ZIntError> {
        let (q, r) = self.div_rem(d)?;
        if r.is_zero() {
            Ok(q)
        } else {
            Err(ZIntError::NotDivisible)
        }
    }

    pub fn inv_unit(self) -> Result<Self, ZIntError> {
        if !self.is_unit() {
            return Err(ZIntError::NoInverse);
        }
        let assocs = self.associates();
        for &unit in &assocs {
            if (self * unit).a == 1 && (self * unit).b == 0 {
                return Ok(unit);
            }
        }
        Err(ZIntError::NoInverse)
    }

    pub fn div_to_fraction(self, d: Self) -> Result<ZIFraction, ZIntError> {
        if d.is_zero() {
            return Err(ZIntError::DivisionByZero);
        }
        let d_conj = d.conj();
        
        let num_a = self.a as I64 * d_conj.a as I64 - self.b as I64 * d_conj.b as I64;
        let num_b = self.a as I64 * d_conj.b as I64 + self.b as I64 * d_conj.a as I64;
        let num = ZInt::new(num_a as I32, num_b as I32);
        
        let den = d.norm_squared();
        Ok(ZIFraction { num, den })
    }

    pub fn inv_fraction(self) -> Result<ZIFraction, ZIntError> {
        if self.is_zero() {
            return Err(ZIntError::DivisionByZero);
        }
        let conj = self.conj();
        let den = self.norm_squared();
        Ok(ZIFraction { num: conj, den })
    }

    pub fn reduce_fraction(frac: ZIFraction) -> ZIFraction {
        let a_abs = frac.num.a.abs() as U64;
        let b_abs = frac.num.b.abs() as U64;
        
        let g1 = num_utils::integer_gcd(a_abs, b_abs);
        let g = num_utils::integer_gcd(g1, frac.den);
        
        if g <= 1 {
            return frac;
        }
        
        let new_num = ZInt::new(
            (frac.num.a as I64 / g as I64) as I32,
            (frac.num.b as I64 / g as I64) as I32
        );
        
        ZIFraction {
            num: new_num,
            den: frac.den / g,
        }
    }

    pub fn gcd(a: Self, b: Self) -> Self {
        let mut x = a.normalize();
        let mut y = b.normalize();
        while !y.is_zero() {
            let (_, r) = x.div_rem(y).unwrap();
            x = y;
            y = r;
        }
        x.normalize()
    }

    pub fn xgcd(a: Self, b: Self) -> (Self, Self, Self) {
        if b.is_zero() {
            return (a.normalize(), Self::one(), Self::zero());
        }

        let mut old_r = a;
        let mut r = b;
        let mut old_s = Self::one();
        let mut s = Self::zero();
        let mut old_t = Self::zero();
        let mut t = Self::one();

        while !r.is_zero() {
            let (q, remainder) = old_r.div_rem(r).unwrap();
            
            old_r = r;
            r = remainder;
            
            let new_s = old_s - (q * s);
            old_s = s;
            s = new_s;
            
            let new_t = old_t - (q * t);
            old_t = t;
            t = new_t;
        }

        (old_r.normalize(), old_s, old_t)
    }
}

impl Add for ZInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            a: self.a.wrapping_add(rhs.a),
            b: self.b.wrapping_add(rhs.b),
        }
    }
}

impl Sub for ZInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            a: self.a.wrapping_sub(rhs.a),
            b: self.b.wrapping_sub(rhs.b),
        }
    }
}

impl Mul for ZInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        // Use i64 arithmetic to match div_rem behavior
        let real = self.a as I64 * rhs.a as I64 - self.b as I64 * rhs.b as I64;
        let imag = self.a as I64 * rhs.b as I64 + self.b as I64 * rhs.a as I64;
        
        // Check for overflow
        if real > I32::MAX as I64 || real < I32::MIN as I64 ||
           imag > I32::MAX as I64 || imag < I32::MIN as I64 {
            panic!("ZInt multiplication overflow");
        }
        
        Self {
            a: real as I32,
            b: imag as I32,
        }
    }
}

impl Neg for ZInt {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            a: self.a.wrapping_neg(),
            b: self.b.wrapping_neg(),
        }
    }
}

impl std::fmt::Display for ZInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.a, self.b)
    }
}

