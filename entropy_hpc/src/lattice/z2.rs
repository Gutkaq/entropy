use crate::types::CInt;

impl CInt {
    /// 1. Convert Gaussian integer to Z² lattice vector
    pub fn to_lattice_vector(self) -> (i32, i32) {
        (self.a, self.b)
    }

    /// 2. Convert Z² lattice vector to Gaussian integer
    pub fn from_lattice_vector(v: (i32, i32)) -> Self {
        CInt::new(v.0, v.1)
    }

    /// 3. Squared Euclidean distance (avoid sqrt)
    pub fn lattice_distance_squared(self, other: Self) -> i32 {
        let da = self.a - other.a;
        let db = self.b - other.b;
        da * da + db * db
    }

    /// 4. Norm squared from origin
    pub fn lattice_norm_squared(self) -> i32 {
        self.a * self.a + self.b * self.b
    }

    /// 5. Find closest lattice point (compare squared distances)
    pub fn closest_lattice_point_int(target: (i32, i32)) -> Self {
        CInt::new(target.0, target.1)
    }

    /// 6. Fundamental domain basis vectors
    pub fn fundamental_domain() -> ((i32, i32), (i32, i32)) {
        ((1, 0), (0, 1))
    }

    /// 7. Volume of fundamental parallelotope
    pub fn lattice_volume() -> i32 {
        1
    }

    /// 8. Check if point lies on Z² lattice
    pub fn is_in_lattice(_v: (i32, i32)) -> bool {
        true
    }
}

