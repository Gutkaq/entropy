use crate::types::CInt;

impl CInt {
    /// Convert Gaussian integer to Z² lattice vector
    pub fn to_lattice_vector(self) -> (i32, i32) {
        (self.a, self.b)
    }

    /// Convert Z² lattice vector to Gaussian integer
    pub fn from_lattice_vector(v: (i32, i32)) -> Self {
        CInt::new(v.0, v.1)
    }

    /// Euclidean distance in Z² lattice
    pub fn lattice_distance(self, other: Self) -> f64 {
        let da = (self.a - other.a) as f64;
        let db = (self.b - other.b) as f64;
        (da * da + db * db).sqrt()
    }
}
