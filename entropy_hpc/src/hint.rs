use std::ops::{Add, Sub, Mul, Neg};
use crate::{I32, I64, U64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HIntError {
    Overflow,
    DivisionByZero,
    NotDivisible,
    NoInverse,
    InvalidHalfInteger,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HIFraction {
    pub num: HInt,
    pub den: U64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct HInt {
    pub a: I32,  // Stored as 2*actual_value (even for integers, odd for half-integers)
    pub b: I32,
    pub c: I32,
    pub d: I32,
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

impl HInt {
    // Create from integers (e.g., new(1,2,3,4) = 1 + 2i + 3j + 4k)
    pub fn new(a: I32, b: I32, c: I32, d: I32) -> Self {
        HInt {
            a: a * 2,
            b: b * 2,
            c: c * 2,
            d: d * 2,
        }
    }

    // Create from half-integers: all components must have same parity
    // e.g., from_halves(1,1,1,1) = 0.5 + 0.5i + 0.5j + 0.5k (all odd = half-integers)
    // from_halves(2,2,2,2) = 1 + 1i + 1j + 1k (all even = integers)
    pub fn from_halves(a: I32, b: I32, c: I32, d: I32) -> Result<Self, HIntError> {
        let a_odd = a % 2 != 0;
        let b_odd = b % 2 != 0;
        let c_odd = c % 2 != 0;
        let d_odd = d % 2 != 0;

        // All must be odd (half-integers) or all even (integers) - no mixing
        if a_odd != b_odd || a_odd != c_odd || a_odd != d_odd {
            return Err(HIntError::InvalidHalfInteger);
        }

        Ok(HInt { a, b, c, d })
    }

    pub fn zero() -> Self {
        HInt::new(0, 0, 0, 0)
    }

    pub fn one() -> Self {
        HInt::new(1, 0, 0, 0)
    }

    pub fn i() -> Self {
        HInt::new(0, 1, 0, 0)
    }

    pub fn j() -> Self {
        HInt::new(0, 0, 1, 0)
    }

    pub fn k() -> Self {
        HInt::new(0, 0, 0, 1)
    }

    pub fn is_zero(self) -> bool {
        self.a == 0 && self.b == 0 && self.c == 0 && self.d == 0
    }

    pub fn is_unit(self) -> bool {
        self.norm_squared() == 1
    }

    pub fn conj(self) -> Self {
        HInt {
            a: self.a,
            b: -self.b,
            c: -self.c,
            d: -self.d,
        }
    }

    pub fn norm_squared(self) -> U64 {
        // N(q) = (a^2 + b^2 + c^2 + d^2) / 4 since stored as 2*value
        let a2: I64 = self.a as I64 * self.a as I64;
        let b2: I64 = self.b as I64 * self.b as I64;
        let c2: I64 = self.c as I64 * self.c as I64;
        let d2: I64 = self.d as I64 * self.d as I64;
        ((a2 + b2 + c2 + d2) / 4) as U64
    }

    pub fn div_rem(self, d: HInt) -> Result<(HInt, HInt), HIntError> {
        if d.is_zero() {
            return Err(HIntError::DivisionByZero);
        }

        let d_norm = d.norm_squared() as I64;
        let d_conj = d.conj();

        // Compute self * conj(d) (result is already stored * 2)
        let num_prod = self * d_conj;

        // Divide by norm and round (need to divide by 2 more for storage)
        let q_a_f = (num_prod.a as f64) / (d_norm as f64 * 2.0);
        let q_b_f = (num_prod.b as f64) / (d_norm as f64 * 2.0);
        let q_c_f = (num_prod.c as f64) / (d_norm as f64 * 2.0);
        let q_d_f = (num_prod.d as f64) / (d_norm as f64 * 2.0);

        // Round and store as *2
        let q = HInt {
            a: (q_a_f.round() * 2.0) as I32,
            b: (q_b_f.round() * 2.0) as I32,
            c: (q_c_f.round() * 2.0) as I32,
            d: (q_d_f.round() * 2.0) as I32,
        };

        let r = self - (q * d);

        Ok((q, r))
    }

    pub fn div_exact(self, d: HInt) -> Result<HInt, HIntError> {
        let (q, r) = self.div_rem(d)?;
        if r.is_zero() {
            Ok(q)
        } else {
            Err(HIntError::NotDivisible)
        }
    }

    pub fn div_to_fraction(self, den: HInt) -> Result<HIFraction, HIntError> {
        if den.is_zero() {
            return Err(HIntError::DivisionByZero);
        }
        Ok(HIFraction {
            num: self,
            den: den.norm_squared(),
        })
    }

    pub fn reduce_fraction(frac: HIFraction) -> HIFraction {
        let a_abs = frac.num.a.abs() as U64;
        let b_abs = frac.num.b.abs() as U64;
        let c_abs = frac.num.c.abs() as U64;
        let d_abs = frac.num.d.abs() as U64;
        
        let g1 = num_utils::integer_gcd(a_abs, b_abs);
        let g2 = num_utils::integer_gcd(c_abs, d_abs);
        let g3 = num_utils::integer_gcd(g1, g2);
        let g = num_utils::integer_gcd(g3, frac.den);
        
        if g <= 1 {
            return frac;
        }

        HIFraction {
            num: frac.num,
            den: frac.den / g,
        }
    }

    pub fn inv_fraction(self) -> Result<HIFraction, HIntError> {
        if self.is_zero() {
            return Err(HIntError::NoInverse);
        }
        Ok(HIFraction {
            num: self.conj(),
            den: self.norm_squared(),
        })
    }

    pub fn inv_unit(self) -> Result<HInt, HIntError> {
        if !self.is_unit() {
            return Err(HIntError::NoInverse);
        }
        Ok(self.conj())
    }

    pub fn gcd(mut a: HInt, mut b: HInt) -> HInt {
        while !b.is_zero() {
            let (_, r) = a.div_rem(b).unwrap_or((HInt::zero(), a));
            a = b;
            b = r;
        }
        a.normalize()
    }

    pub fn normalize(self) -> HInt {
        // Normalize by multiplying by unit if needed
        // For quaternions: prefer positive real part
        if self.is_zero() {
            return self;
        }
        
        if self.a > 0 {
            return self;
        }
        
        // Try multiplying by -1
        let neg = -self;
        if neg.a > 0 {
            return neg;
        }
        
        self
    }

    pub fn associates(self) -> [HInt; 8] {
        let one = HInt::one();
        let neg_one = -one;
        let i = HInt::i();
        let neg_i = -i;
        let j = HInt::j();
        let neg_j = -j;
        let k = HInt::k();
        let neg_k = -k;

        [
            self * one,
            self * neg_one,
            self * i,
            self * neg_i,
            self * j,
            self * neg_j,
            self * k,
            self * neg_k,
        ]
    }

    pub fn to_float_components(self) -> (f64, f64, f64, f64) {
        (
            self.a as f64 / 2.0,
            self.b as f64 / 2.0,
            self.c as f64 / 2.0,
            self.d as f64 / 2.0,
        )
    }

    pub fn is_anticommutative_pair(a: HInt, b: HInt) -> bool {
        a * b == -(b * a)
    }

    pub fn is_associative_triple(a: HInt, b: HInt, c: HInt) -> bool {
        (a * b) * c == a * (b * c)
    }
}

impl Add for HInt {
    type Output = HInt;
    fn add(self, other: HInt) -> HInt {
        HInt {
            a: self.a + other.a,
            b: self.b + other.b,
            c: self.c + other.c,
            d: self.d + other.d,
        }
    }
}

impl Sub for HInt {
    type Output = HInt;
    fn sub(self, other: HInt) -> HInt {
        HInt {
            a: self.a - other.a,
            b: self.b - other.b,
            c: self.c - other.c,
            d: self.d - other.d,
        }
    }
}

impl Mul for HInt {
    type Output = HInt;
    fn mul(self, other: HInt) -> HInt {
        // Quaternion multiplication: (a+bi+cj+dk)(e+fi+gj+hk)
        // i²=j²=k²=ijk=-1, ij=k, jk=i, ki=j, ji=-k, kj=-i, ik=-j
        
        // Working with 2*values, result needs /4 total (but we keep *2 storage)
        // So multiply and divide by 2 once
        let a = self.a as I64 * other.a as I64
            - self.b as I64 * other.b as I64
            - self.c as I64 * other.c as I64
            - self.d as I64 * other.d as I64;

        let b = self.a as I64 * other.b as I64
            + self.b as I64 * other.a as I64
            + self.c as I64 * other.d as I64
            - self.d as I64 * other.c as I64;

        let c = self.a as I64 * other.c as I64
            - self.b as I64 * other.d as I64
            + self.c as I64 * other.a as I64
            + self.d as I64 * other.b as I64;

        let d = self.a as I64 * other.d as I64
            + self.b as I64 * other.c as I64
            - self.c as I64 * other.b as I64
            + self.d as I64 * other.a as I64;

        // Divide by 2 to maintain *2 storage (since we multiplied *2 * *2 = *4)
        HInt {
            a: (a / 2) as I32,
            b: (b / 2) as I32,
            c: (c / 2) as I32,
            d: (d / 2) as I32,
        }
    }
}

impl Neg for HInt {
    type Output = HInt;
    fn neg(self) -> HInt {
        HInt {
            a: -self.a,
            b: -self.b,
            c: -self.c,
            d: -self.d,
        }
    }
}

