use std::ops::{Add, Sub, Mul, Neg};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CIntError {
    Overflow,
    DivisionByZero,
    NotDivisible,
    NoInverse,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CIFraction {
    pub num: CInt,
    pub den: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct CInt {
    pub a: i32,
    pub b: i32,
}

mod num_utils {
    pub fn integer_gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}

impl CInt {
    pub fn new(a: i32, b: i32) -> Self {
        CInt { a, b }
    }

    pub fn zero() -> Self {
        CInt::new(0, 0)
    }

    pub fn one() -> Self {
        CInt::new(1, 0)
    }

    pub fn i() -> Self {
        CInt::new(0, 1)
    }

    pub fn is_zero(self) -> bool {
        self.a == 0 && self.b == 0
    }

    pub fn is_unit(self) -> bool {
        self.norm_squared() == 1
    }

    pub fn conj(self) -> Self {
        CInt { a: self.a, b: -self.b }
    }

    pub fn norm_squared(self) -> u64 {
        let a2: i64 = self.a as i64 * self.a as i64;
        let b2: i64 = self.b as i64 * self.b as i64;
        (a2 + b2) as u64
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

    pub fn div_rem(self, d: Self) -> Result<(Self, Self), CIntError> {
        if d.is_zero() {
            return Err(CIntError::DivisionByZero);
        }

        let norm_d = d.norm_squared() as i64;
        let d_conj = d.conj();
        let num_a = self.a as i64 * d_conj.a as i64 - self.b as i64 * d_conj.b as i64;
        let num_b = self.a as i64 * d_conj.b as i64 + self.b as i64 * d_conj.a as i64;

        let q_real_f = num_a as f64 / norm_d as f64;
        let q_imag_f = num_b as f64 / norm_d as f64;

        let q_real = q_real_f.round() as i32;
        let q_imag = q_imag_f.round() as i32;

        let q = CInt::new(q_real, q_imag);
        let r = self - (q * d);

        Ok((q, r))
    }

    pub fn div_exact(self, d: Self) -> Result<Self, CIntError> {
        let (q, r) = self.div_rem(d)?;
        if r.is_zero() {
            Ok(q)
        } else {
            Err(CIntError::NotDivisible)
        }
    }

    pub fn inv_unit(self) -> Result<Self, CIntError> {
        if !self.is_unit() {
            return Err(CIntError::NoInverse);
        }

        let assocs = self.associates();
        for &unit in &assocs {
            if (self * unit).a == 1 && (self * unit).b == 0 {
                return Ok(unit);
            }
        }

        Err(CIntError::NoInverse)
    }

    pub fn div_to_fraction(self, d: Self) -> Result<CIFraction, CIntError> {
        if d.is_zero() {
            return Err(CIntError::DivisionByZero);
        }

        let d_conj = d.conj();
        let num_a = self.a as i64 * d_conj.a as i64 - self.b as i64 * d_conj.b as i64;
        let num_b = self.a as i64 * d_conj.b as i64 + self.b as i64 * d_conj.a as i64;
        let num = CInt::new(num_a as i32, num_b as i32);
        let den = d.norm_squared();

        Ok(CIFraction { num, den })
    }

    pub fn inv_fraction(self) -> Result<CIFraction, CIntError> {
        if self.is_zero() {
            return Err(CIntError::DivisionByZero);
        }

        let conj = self.conj();
        let den = self.norm_squared();
        Ok(CIFraction { num: conj, den })
    }

    pub fn reduce_fraction(frac: CIFraction) -> CIFraction {
        let a_abs = frac.num.a.abs() as u64;
        let b_abs = frac.num.b.abs() as u64;
        let g1 = num_utils::integer_gcd(a_abs, b_abs);
        let g = num_utils::integer_gcd(g1, frac.den);

        if g <= 1 {
            return frac;
        }

        let new_num = CInt::new(
            (frac.num.a as i64 / g as i64) as i32,
            (frac.num.b as i64 / g as i64) as i32
        );
        CIFraction {
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

impl Add for CInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            a: self.a.wrapping_add(rhs.a),
            b: self.b.wrapping_add(rhs.b),
        }
    }
}

impl Sub for CInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            a: self.a.wrapping_sub(rhs.a),
            b: self.b.wrapping_sub(rhs.b),
        }
    }
}

impl Mul for CInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let real = self.a as i64 * rhs.a as i64 - self.b as i64 * rhs.b as i64;
        let imag = self.a as i64 * rhs.b as i64 + self.b as i64 * rhs.a as i64;

        if real > i32::MAX as i64 || real < i32::MIN as i64 ||
           imag > i32::MAX as i64 || imag < i32::MIN as i64 {
            panic!("CInt multiplication overflow");
        }

        Self {
            a: real as i32,
            b: imag as i32,
        }
    }
}

impl Neg for CInt {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            a: self.a.wrapping_neg(),
            b: self.b.wrapping_neg(),
        }
    }
}

