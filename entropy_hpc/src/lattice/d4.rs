use crate::types::HInt;

impl HInt {
    pub fn to_lattice_vector(self) -> (i32, i32, i32, i32) {
        (self.a, self.b, self.c, self.d)
    }

    pub fn from_lattice_vector(v: (i32, i32, i32, i32)) -> Self {
        HInt::new(v.0, v.1, v.2, v.3)
    }

    pub fn lattice_distance_squared(self, other: Self) -> i32 {
        let da = self.a - other.a;
        let db = self.b - other.b;
        let dc = self.c - other.c;
        let dd = self.d - other.d;
        (da * da + db * db + dc * dc + dd * dd) / 4
    }

    pub fn lattice_norm_squared(self) -> i32 {
        (self.a * self.a + self.b * self.b + self.c * self.c + self.d * self.d) / 4
    }

    pub fn closest_lattice_point_int(target: (i32, i32, i32, i32)) -> Self {
        HInt::new(target.0, target.1, target.2, target.3)
    }

    pub fn fundamental_domain() -> ((i32, i32, i32, i32), (i32, i32, i32, i32)) {
        ((2, 0, 0, 0), (0, 2, 2, 2))
    }

    pub fn lattice_volume() -> i32 {
        1
    }

    pub fn is_in_lattice(v: (i32, i32, i32, i32)) -> bool {
        let sum = v.0 + v.1 + v.2 + v.3;
        let all_even = v.0 % 2 == 0 && v.1 % 2 == 0 && v.2 % 2 == 0 && v.3 % 2 == 0;
        let all_odd = v.0 % 2 != 0 && v.1 % 2 != 0 && v.2 % 2 != 0 && v.3 % 2 != 0;
        (all_even || all_odd) && sum % 4 == 0
    }
}
