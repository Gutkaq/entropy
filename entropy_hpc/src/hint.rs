use std::ops::{Add, Sub, Mul, Neg};
use crate::{I32, I64, U64};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HIntError {
    Overflow,
    DivisionByZero,
    NotDivisible,
    NoInverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HIFraction {
    pub num: HInt,
    pub den: U64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct HInt {
    pub a: I32,
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
    pub fn new(a: I32, b: I32, c: I32, d: I32) -> Self {
        HInt { a, b, c, d }
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

    pub fn from_halves(a: I32, b: I32, c: I32, d: I32) -> Result<Self, HIntError> {
        Ok(HInt::new(a, b, c, d))
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
        let a2: I64 = self.a as I64 * self.a as I64;
        let b2: I64 = self.b as I64 * self.b as I64;
        let c2: I64 = self.c as I64 * self.c as I64;
        let d2: I64 = self.d as I64 * self.d as I64;
        (a2 + b2 + c2 + d2) as U64
    }

    pub fn div_rem(self, d: HInt) -> Result<(HInt, HInt), HIntError> {
        if d.is_zero() {
            return Err(HIntError::DivisionByZero);
        }

        let d_norm = d.norm_squared() as I64;
        let d_conj = d.conj();

        let num_prod = self * d_conj;
        let q_a = num_prod.a / (d_norm as I32);
        let q_b = num_prod.b / (d_norm as I32);
        let q_c = num_prod.c / (d_norm as I32);
        let q_d = num_prod.d / (d_norm as I32);

        let q = HInt::new(q_a, q_b, q_c, q_d);
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
        let g = num_utils::integer_gcd(frac.num.norm_squared(), frac.den);
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
        let units = [
            HInt::new(1, 0, 0, 0),
            HInt::new(-1, 0, 0, 0),
            HInt::new(0, 1, 0, 0),
            HInt::new(0, -1, 0, 0),
            HInt::new(0, 0, 1, 0),
            HInt::new(0, 0, -1, 0),
            HInt::new(0, 0, 0, 1),
            HInt::new(0, 0, 0, -1),
        ];

        let mut best = self;
        let mut best_norm = self.norm_squared();

        for u in &units {
            let prod = self * (*u);
            let norm = prod.norm_squared();
            if norm < best_norm {
                best = prod;
                best_norm = norm;
            }
        }
        best
    }

    pub fn associates(self) -> [HInt; 8] {
        let units = [
            HInt::new(1, 0, 0, 0),
            HInt::new(-1, 0, 0, 0),
            HInt::new(0, 1, 0, 0),
            HInt::new(0, -1, 0, 0),
            HInt::new(0, 0, 1, 0),
            HInt::new(0, 0, -1, 0),
            HInt::new(0, 0, 0, 1),
            HInt::new(0, 0, 0, -1),
        ];

        let mut result = [HInt::zero(); 8];
        for (i, u) in units.iter().enumerate() {
            result[i] = self * (*u);
        }
        result
    }

    pub fn to_float_components(self) -> (f64, f64, f64, f64) {
        (
            self.a as f64,
            self.b as f64,
            self.c as f64,
            self.d as f64,
        )
    }

    pub fn is_anticommutative_pair(a: HInt, b: HInt) -> bool {
        a * b == -(b * a)
    }

    pub fn is_associative_triple(a: HInt, b: HInt, c: HInt) -> bool {
        (a * b) * c == a * (b * c)
    }
}

impl std::fmt::Display for HInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} + {}i + {}j + {}k",
            self.a, self.b, self.c, self.d
        )
    }
}

impl Add for HInt {
    type Output = HInt;
    fn add(self, other: HInt) -> HInt {
        HInt::new(
            self.a + other.a,
            self.b + other.b,
            self.c + other.c,
            self.d + other.d,
        )
    }
}

impl Sub for HInt {
    type Output = HInt;
    fn sub(self, other: HInt) -> HInt {
        HInt::new(
            self.a - other.a,
            self.b - other.b,
            self.c - other.c,
            self.d - other.d,
        )
    }
}

impl Mul for HInt {
    type Output = HInt;
    fn mul(self, other: HInt) -> HInt {
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

        HInt::new(a as I32, b as I32, c as I32, d as I32)
    }
}

impl Neg for HInt {
    type Output = HInt;
    fn neg(self) -> HInt {
        HInt::new(-self.a, -self.b, -self.c, -self.d)
    }
}

