#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use crate::types::{CInt, HInt, OInt};

/// SIMD lattice operations (AVX2)
pub struct LatticeSimd;

impl LatticeSimd {
    // ════════════════════ Z² (A₂) ════════════════════
    
    pub fn z2_to_lattice_batch(points: &[CInt]) -> Vec<(i32, i32)> {
        points.iter().map(|p| p.to_lattice_vector()).collect()
    }

    pub fn z2_from_lattice_batch(vecs: &[(i32, i32)]) -> Vec<CInt> {
        vecs.iter().map(|&v| CInt::from_lattice_vector(v)).collect()
    }

    #[cfg(target_arch = "x86_64")]
    pub fn z2_distance_squared_batch(points: &[CInt], target: CInt) -> Vec<i32> {
        unsafe {
            points.chunks(4).flat_map(|chunk| {
                let mut dists = vec![];
                for p in chunk {
                    dists.push(p.lattice_distance_squared(target));
                }
                dists
            }).collect()
        }
    }

    pub fn z2_norm_squared_batch(points: &[CInt]) -> Vec<i32> {
        points.iter().map(|p| p.lattice_norm_squared()).collect()
    }

    pub fn z2_closest_point_batch(targets: &[(i32, i32)]) -> Vec<CInt> {
        targets.iter().map(|&t| CInt::closest_lattice_point_int(t)).collect()
    }

    pub fn z2_fundamental_domain_batch(count: usize) -> Vec<((i32, i32), (i32, i32))> {
        vec![CInt::fundamental_domain(); count]
    }

    pub fn z2_volume_batch(count: usize) -> Vec<i32> {
        vec![CInt::lattice_volume(); count]
    }

    pub fn z2_in_lattice_batch(points: &[(i32, i32)]) -> Vec<bool> {
        points.iter().map(|&p| CInt::is_in_lattice(p)).collect()
    }

    // ════════════════════ D₄ ════════════════════

    pub fn d4_to_lattice_batch(points: &[HInt]) -> Vec<(i32, i32, i32, i32)> {
        points.iter().map(|p| p.to_lattice_vector()).collect()
    }

    pub fn d4_from_lattice_batch(vecs: &[(i32, i32, i32, i32)]) -> Vec<HInt> {
        vecs.iter().map(|&v| HInt::from_lattice_vector(v)).collect()
    }

    #[cfg(target_arch = "x86_64")]
    pub fn d4_distance_squared_batch(points: &[HInt], target: HInt) -> Vec<i32> {
        unsafe {
            points.chunks(2).flat_map(|chunk| {
                let mut dists = vec![];
                for p in chunk {
                    dists.push(p.lattice_distance_squared(target));
                }
                dists
            }).collect()
        }
    }

    pub fn d4_norm_squared_batch(points: &[HInt]) -> Vec<i32> {
        points.iter().map(|p| p.lattice_norm_squared()).collect()
    }

    pub fn d4_closest_point_batch(targets: &[(i32, i32, i32, i32)]) -> Vec<HInt> {
        targets.iter().map(|&t| HInt::closest_lattice_point_int(t)).collect()
    }

    pub fn d4_fundamental_domain_batch(count: usize) -> Vec<((i32, i32, i32, i32), (i32, i32, i32, i32))> {
        vec![HInt::fundamental_domain(); count]
    }

    pub fn d4_volume_batch(count: usize) -> Vec<i32> {
        vec![HInt::lattice_volume(); count]
    }

    pub fn d4_in_lattice_batch(points: &[(i32, i32, i32, i32)]) -> Vec<bool> {
        points.iter().map(|&p| HInt::is_in_lattice(p)).collect()
    }

    // ════════════════════ E₈ ════════════════════

    pub fn e8_to_lattice_batch(points: &[OInt]) -> Vec<(i32, i32, i32, i32, i32, i32, i32, i32)> {
        points.iter().map(|p| p.to_lattice_vector()).collect()
    }

    pub fn e8_from_lattice_batch(vecs: &[(i32, i32, i32, i32, i32, i32, i32, i32)]) -> Vec<OInt> {
        vecs.iter().map(|&v| OInt::from_lattice_vector(v)).collect()
    }

    #[cfg(target_arch = "x86_64")]
    pub fn e8_distance_squared_batch(points: &[OInt], target: OInt) -> Vec<i32> {
        unsafe {
            points.iter().map(|p| p.lattice_distance_squared(target)).collect()
        }
    }

    pub fn e8_norm_squared_batch(points: &[OInt]) -> Vec<i32> {
        points.iter().map(|p| p.lattice_norm_squared()).collect()
    }

    pub fn e8_closest_point_batch(targets: &[(i32, i32, i32, i32, i32, i32, i32, i32)]) -> Vec<OInt> {
        targets.iter().map(|&t| OInt::closest_lattice_point_int(t)).collect()
    }

    pub fn e8_fundamental_domain_batch(count: usize) -> Vec<((i32, i32, i32, i32, i32, i32, i32, i32), (i32, i32, i32, i32, i32, i32, i32, i32))> {
        vec![OInt::fundamental_domain(); count]
    }

    pub fn e8_volume_batch(count: usize) -> Vec<i32> {
        vec![OInt::lattice_volume(); count]
    }

    pub fn e8_in_lattice_batch(points: &[(i32, i32, i32, i32, i32, i32, i32, i32)]) -> Vec<bool> {
        points.iter().map(|&p| OInt::is_in_lattice(p)).collect()
    }
}

