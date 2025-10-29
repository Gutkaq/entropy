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
        (a2.wrapping_add(b2)) as U64
    }

    fn round_nearest_div(num: I64, den: I64) -> Result<I32, ZIntError> {
        let q_floor = num / den;
        let r = num % den;
        let half_den = den / 2;
        let mut q_final: I64 = q_floor;
        if r.abs() > half_den {
            q_final += r.signum() as I64;
        } else if r.abs() == half_den {
            if q_floor % 2 != 0 {
                q_final += r.signum() as I64;
            }
        }
        if q_final > I32::MAX as I64 || q_final < I32::MIN as I64 {
            return Err(ZIntError::Overflow);
        }
        Ok(q_final as I32)
    }

    pub fn div_rem(self, d: Self) -> Result<(Self, Self), ZIntError> {
        if d.is_zero() {
            return Err(ZIntError::DivisionByZero);
        }
        let n: I64 = d.norm_squared() as I64;
        let temp_num = self * d.conj();
        let qa = ZInt::round_nearest_div(temp_num.a as I64, n)?;
        let qb = ZInt::round_nearest_div(temp_num.b as I64, n)?;
        let q = ZInt::new(qa, qb);
        let r = self - q * d;
        Ok((q, r))
    }

    pub fn div_exact(self, d: Self) -> Result<Self, ZIntError> {
        let (q, r) = self.div_rem(d)?;
        if !r.is_zero() {
            return Err(ZIntError::NotDivisible);
        }
        Ok(q)
    }

    pub fn inv_unit(self) -> Result<Self, ZIntError> {
        if !self.is_unit() { 
            return Err(ZIntError::NoInverse); 
        }
        Ok(self.conj())
    }

    pub fn reduce_fraction(f: ZIFraction) -> ZIFraction {
        let ga = (f.num.a as I64).wrapping_abs() as U64;
        let gb = (f.num.b as I64).wrapping_abs() as U64;
        let g1 = num_utils::integer_gcd(ga, gb);
        let g = num_utils::integer_gcd(g1, f.den);
        if g == 0 || g == 1 { 
            return f; 
        }
        ZIFraction {
            num: ZInt::new(
                (f.num.a as I64 / g as I64) as I32,
                (f.num.b as I64 / g as I64) as I32,
            ),
            den: f.den / g,
        }
    }

    pub fn div_to_fraction(self, d: Self) -> Result<ZIFraction, ZIntError> {
        if d.is_zero() { 
            return Err(ZIntError::DivisionByZero); 
        }
        let num_z = self * d.conj();
        let den = d.norm_squared();
        Ok(ZInt::reduce_fraction(ZIFraction { num: num_z, den }))
    }

    pub fn inv_fraction(self) -> Result<ZIFraction, ZIntError> {
        ZInt::one().div_to_fraction(self)
    }

    pub fn associates(self) -> [Self; 4] {
        let i_u = self * ZInt::i();
        [self, i_u, self.neg(), i_u.neg()]
    }

    pub fn normalize(self) -> Self {
        // Zero case
        if self.is_zero() { 
            return self; 
        }
        
        // Already in first quadrant (a > 0, b >= 0)
        if self.a > 0 && self.b >= 0 { 
            return self; 
        }
        
        // Positive imaginary axis (a == 0, b > 0) - already normalized
        if self.a == 0 && self.b > 0 {
            return self;
        }
        
        let i = ZInt::i();
        let neg_i = -ZInt::i();
        
        // Second quadrant (a < 0, b > 0) - multiply by -i
        if self.a < 0 && self.b > 0 { 
            return self * neg_i; 
        }
        
        // Third quadrant (a < 0, b <= 0) - multiply by -1
        if self.a < 0 && self.b <= 0 { 
            return self.neg(); 
        }
        
        // Fourth quadrant (a > 0, b < 0) or negative imaginary axis (a == 0, b < 0)
        // Multiply by i
        self * i
    }

    pub fn gcd(mut a: Self, mut b: Self) -> Self {
        while !b.is_zero() {
            let (_, r) = match a.div_rem(b) {
                Ok(res) => res,
                Err(_) => return ZInt::one().normalize(),
            };
            a = b;
            b = r;
        }
        a.normalize()
    }
}

impl Add for ZInt {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ZInt { 
            a: self.a.wrapping_add(other.a), 
            b: self.b.wrapping_add(other.b) 
        }
    }
}

impl Sub for ZInt {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        ZInt { 
            a: self.a.wrapping_sub(other.a), 
            b: self.b.wrapping_sub(other.b) 
        }
    }
}

impl Neg for ZInt {
    type Output = Self;
    fn neg(self) -> Self {
        ZInt { a: -self.a, b: -self.b }
    }
}

impl Mul for ZInt {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let ac: I64 = self.a as I64 * other.a as I64;
        let bd: I64 = self.b as I64 * other.b as I64;
        let ad: I64 = self.a as I64 * other.b as I64;
        let bc: I64 = self.b as I64 * other.a as I64;
        
        let res_a = ac.wrapping_sub(bd);
        let res_b = ad.wrapping_add(bc);
        
        if res_a > I32::MAX as I64 || res_a < I32::MIN as I64 ||
           res_b > I32::MAX as I64 || res_b < I32::MIN as I64 {
            panic!("ZInt component overflow during multiplication.");
        }
        
        ZInt { a: res_a as I32, b: res_b as I32 }
    }
}

impl std::fmt::Display for ZInt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} + {}i", self.a, self.b)
    }
}

