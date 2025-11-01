use std::ops::{Add, Sub, Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OIntError {
    Overflow,
    DivisionByZero,
    NotDivisible,
    NoInverse,
    InvalidHalfInteger,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct OIFraction {
    pub num: OInt,
    pub den: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct OInt {
    pub a: i32,  // scalar (stored as 2*actual for half-integer support)
    pub b: i32,  // e1
    pub c: i32,  // e2
    pub d: i32,  // e3
    pub e: i32,  // e4
    pub f: i32,  // e5
    pub g: i32,  // e6
    pub h: i32,  // e7
}

// Fano plane multiplication table
// Returns (sign, target_index) for basis multiplication e_i * e_j
mod fano_plane {
    pub fn multiply_basis(i: usize, j: usize) -> (i32, usize) {
        if i == 0 { return (1, j); }
        if j == 0 { return (1, i); }
        if i == j { return (-1, 0); }  // e_i * e_i = -1

        // Fano plane multiplication rules (Cayley-Dickson construction)
        match (i, j) {
            (1, 2) => (1, 4),   // e1*e2 = e4
            (2, 1) => (-1, 4),  // e2*e1 = -e4
            (2, 3) => (1, 5),   // e2*e3 = e5
            (3, 2) => (-1, 5),
            (3, 1) => (1, 6),   // e3*e1 = e6
            (1, 3) => (-1, 6),
            (1, 4) => (-1, 2),  // e1*e4 = -e2
            (4, 1) => (1, 2),
            (4, 2) => (1, 1),   // e4*e2 = e1
            (2, 4) => (-1, 1),
            (1, 5) => (1, 3),   // e1*e5 = e3
            (5, 1) => (-1, 3),
            (5, 3) => (1, 1),   // e5*e3 = e1
            (3, 5) => (-1, 1),
            (1, 6) => (-1, 5),  // e1*e6 = -e5
            (6, 1) => (1, 5),
            (6, 5) => (1, 1),   // e6*e5 = e1
            (5, 6) => (-1, 1),
            (1, 7) => (1, 6),   // e1*e7 = e6
            (7, 1) => (-1, 6),
            (7, 6) => (1, 1),   // e7*e6 = e1
            (6, 7) => (-1, 1),
            (2, 5) => (-1, 7),  // e2*e5 = -e7
            (5, 2) => (1, 7),
            (2, 6) => (1, 7),   // e2*e6 = e7
            (6, 2) => (-1, 7),
            (3, 4) => (1, 7),   // e3*e4 = e7
            (4, 3) => (-1, 7),
            (3, 7) => (-1, 4),  // e3*e7 = -e4
            (7, 3) => (1, 4),
            (4, 5) => (1, 6),   // e4*e5 = e6
            (5, 4) => (-1, 6),
            (4, 6) => (-1, 5),  // e4*e6 = -e5
            (6, 4) => (1, 5),
            (4, 7) => (1, 2),   // e4*e7 = e2
            (7, 4) => (-1, 2),
            (5, 7) => (-1, 4),  // e5*e7 = -e4
            (7, 5) => (1, 4),
            (6, 3) => (1, 7),   // e6*e3 = e7
            (3, 6) => (-1, 7),
            (7, 2) => (1, 5),   // e7*e2 = e5
            (2, 7) => (-1, 5),
            _ => (1, 0),  // Shouldn't reach here
        }
    }
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

impl OInt {
    // Create from integers (stored as 2*actual)
    pub fn new(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> Self {
        OInt {
            a: a * 2,
            b: b * 2,
            c: c * 2,
            d: d * 2,
            e: e * 2,
            f: f * 2,
            g: g * 2,
            h: h * 2,
        }
    }

    // Create from half-integers (all same parity)
    pub fn from_halves(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) 
        -> Result<Self, OIntError> {
        let components = [a, b, c, d, e, f, g, h];
        let first_odd = components[0] % 2 != 0;
        
        // All must have same parity
        if !components.iter().all(|&x| (x % 2 != 0) == first_odd) {
            return Err(OIntError::InvalidHalfInteger);
        }

        Ok(OInt { a, b, c, d, e, f, g, h })
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

    pub fn is_zero(self) -> bool {
        self.a == 0 && self.b == 0 && self.c == 0 && self.d == 0
            && self.e == 0 && self.f == 0 && self.g == 0 && self.h == 0
    }

    pub fn is_unit(self) -> bool {
        self.norm_squared() == 1
    }

    pub fn conj(self) -> Self {
        OInt {
            a: self.a,
            b: -self.b,
            c: -self.c,
            d: -self.d,
            e: -self.e,
            f: -self.f,
            g: -self.g,
            h: -self.h,
        }
    }

    pub fn norm_squared(self) -> u64 {
        let components = [self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h];
        let sum: i64 = components.iter()
            .map(|&x| (x as i64) * (x as i64))
            .sum();
        (sum / 4) as u64  // Divide by 4 for *2 storage
    }

    pub fn div_rem(self, d: Self) -> Result<(Self, Self), OIntError> {
        if d.is_zero() {
            return Err(OIntError::DivisionByZero);
        }

        let d_norm = d.norm_squared() as i64;
        let d_conj = d.conj();
        let num_prod = self * d_conj;

        // Round each component
        let components = [
            num_prod.a, num_prod.b, num_prod.c, num_prod.d,
            num_prod.e, num_prod.f, num_prod.g, num_prod.h
        ];

        let q_components: Vec<i32> = components.iter()
            .map(|&x| {
                let val = (x as f64) / (d_norm as f64 * 2.0);
                (val.round() * 2.0) as i32
            })
            .collect();

        let q = OInt {
            a: q_components[0],
            b: q_components[1],
            c: q_components[2],
            d: q_components[3],
            e: q_components[4],
            f: q_components[5],
            g: q_components[6],
            h: q_components[7],
        };

        let r = self - (q * d);
        Ok((q, r))
    }

    pub fn div_exact(self, d: Self) -> Result<Self, OIntError> {
        let (q, r) = self.div_rem(d)?;
        if r.is_zero() {
            Ok(q)
        } else {
            Err(OIntError::NotDivisible)
        }
    }

    pub fn div_to_fraction(self, den: Self) -> Result<OIFraction, OIntError> {
        if den.is_zero() {
            return Err(OIntError::DivisionByZero);
        }
        Ok(OIFraction {
            num: self,
            den: den.norm_squared(),
        })
    }

    pub fn reduce_fraction(frac: OIFraction) -> OIFraction {
        let components = [
            frac.num.a.abs() as u64, frac.num.b.abs() as u64,
            frac.num.c.abs() as u64, frac.num.d.abs() as u64,
            frac.num.e.abs() as u64, frac.num.f.abs() as u64,
            frac.num.g.abs() as u64, frac.num.h.abs() as u64,
        ];
        
        let mut g = components[0];
        for &comp in &components[1..] {
            g = num_utils::integer_gcd(g, comp);
        }
        g = num_utils::integer_gcd(g, frac.den);
        
        if g <= 1 {
            return frac;
        }

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

    pub fn inv_unit(self) -> Result<Self, OIntError> {
        if !self.is_unit() {
            return Err(OIntError::NoInverse);
        }
        Ok(self.conj())
    }

    pub fn gcd(mut a: Self, mut b: Self) -> Self {
        while !b.is_zero() {
            let (_, r) = a.div_rem(b).unwrap_or((Self::zero(), a));
            a = b;
            b = r;
        }
        a.normalize()
    }

    pub fn normalize(self) -> Self {
        if self.is_zero() {
            return self;
        }
        
        if self.a > 0 {
            return self;
        }
        
        let neg = -self;
        if neg.a > 0 {
            return neg;
        }
        
        self
    }

    pub fn associates(self) -> [Self; 8] {
        let units = [
            Self::one(),
            -Self::one(),
            Self::e1(),
            -Self::e1(),
            Self::e2(),
            -Self::e2(),
            Self::e3(),
            -Self::e3(),
        ];

        let mut result = [Self::zero(); 8];
        for (i, u) in units.iter().enumerate() {
            result[i] = self * (*u);
        }
        result
    }

    pub fn to_float_components(self) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        (
            self.a as f64 / 2.0,
            self.b as f64 / 2.0,
            self.c as f64 / 2.0,
            self.d as f64 / 2.0,
            self.e as f64 / 2.0,
            self.f as f64 / 2.0,
            self.g as f64 / 2.0,
            self.h as f64 / 2.0,
        )
    }

    // Non-commutative check
    pub fn is_non_commutative_pair(a: Self, b: Self) -> bool {
        a * b != b * a
    }

    // Non-associative check
    pub fn is_non_associative_triple(a: Self, b: Self, c: Self) -> bool {
        (a * b) * c != a * (b * c)
    }

    // Alternative algebra property: (a*a)*b = a*(a*b) and (a*b)*b = a*(b*b)
    pub fn alternative_identity(a: Self, b: Self) -> bool {
        let aa = a * a;
        let ab = a * b;
        let bb = b * b;
        
        let left1 = aa * b;
        let right1 = a * ab;
        
        let left2 = ab * b;
        let right2 = a * bb;
        
        left1 == right1 && left2 == right2
    }

    // Moufang identity: (a*b)*(c*a) = a*(b*c)*a
    pub fn moufang_identity(a: Self, b: Self, c: Self) -> bool {
        let ab = a * b;
        let ca = c * a;
        let bc = b * c;
        
        let left = ab * ca;
        let right = a * (bc * a);
        
        left == right
    }
}

impl Add for OInt {
    type Output = OInt;
    fn add(self, other: OInt) -> OInt {
        OInt {
            a: self.a + other.a,
            b: self.b + other.b,
            c: self.c + other.c,
            d: self.d + other.d,
            e: self.e + other.e,
            f: self.f + other.f,
            g: self.g + other.g,
            h: self.h + other.h,
        }
    }
}

impl Sub for OInt {
    type Output = OInt;
    fn sub(self, other: OInt) -> OInt {
        OInt {
            a: self.a - other.a,
            b: self.b - other.b,
            c: self.c - other.c,
            d: self.d - other.d,
            e: self.e - other.e,
            f: self.f - other.f,
            g: self.g - other.g,
            h: self.h - other.h,
        }
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

        // Divide by 2 to maintain *2 storage
        OInt {
            a: (result[0] / 2) as i32,
            b: (result[1] / 2) as i32,
            c: (result[2] / 2) as i32,
            d: (result[3] / 2) as i32,
            e: (result[4] / 2) as i32,
            f: (result[5] / 2) as i32,
            g: (result[6] / 2) as i32,
            h: (result[7] / 2) as i32,
        }
    }
}

impl Neg for OInt {
    type Output = OInt;
    fn neg(self) -> OInt {
        OInt {
            a: -self.a,
            b: -self.b,
            c: -self.c,
            d: -self.d,
            e: -self.e,
            f: -self.f,
            g: -self.g,
            h: -self.h,
        }
    }
}

