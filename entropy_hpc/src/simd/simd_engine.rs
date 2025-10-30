// src/simd/simd_engine.rs

use crate::zint::ZInt;
use crate::hint::HInt;
use crate::oint::OInt;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// ========================================================================
// ZINT (GAUSSIAN INTEGERS) SIMD
// ========================================================================

// ========== ADD ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn zint_add_batch_avx2(a: &[ZInt; 4], b: &[ZInt; 4]) -> [ZInt; 4] {
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let b_vec = _mm256_loadu_si256(b.as_ptr() as *const __m256i);
    let result = _mm256_add_epi32(a_vec, b_vec);
    
    let mut out = [ZInt::new(0, 0); 4];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn zint_add_batch(a: &[ZInt; 4], b: &[ZInt; 4]) -> [ZInt; 4] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { zint_add_batch_avx2(a, b) };
        }
    }
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

// ========== SUB ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn zint_sub_batch_avx2(a: &[ZInt; 4], b: &[ZInt; 4]) -> [ZInt; 4] {
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let b_vec = _mm256_loadu_si256(b.as_ptr() as *const __m256i);
    let result = _mm256_sub_epi32(a_vec, b_vec);
    
    let mut out = [ZInt::new(0, 0); 4];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn zint_sub_batch(a: &[ZInt; 4], b: &[ZInt; 4]) -> [ZInt; 4] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { zint_sub_batch_avx2(a, b) };
        }
    }
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

// ========== NEG ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn zint_neg_batch_avx2(a: &[ZInt; 4]) -> [ZInt; 4] {
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let zero = _mm256_setzero_si256();
    let result = _mm256_sub_epi32(zero, a_vec);
    
    let mut out = [ZInt::new(0, 0); 4];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn zint_neg_batch(a: &[ZInt; 4]) -> [ZInt; 4] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { zint_neg_batch_avx2(a) };
        }
    }
    [-a[0], -a[1], -a[2], -a[3]]
}

// ========== CONJ ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn zint_conj_batch_avx2(a: &[ZInt; 4]) -> [ZInt; 4] {
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let mask = _mm256_setr_epi32(1, -1, 1, -1, 1, -1, 1, -1);
    let result = _mm256_sign_epi32(a_vec, mask);
    
    let mut out = [ZInt::new(0, 0); 4];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn zint_conj_batch(a: &[ZInt; 4]) -> [ZInt; 4] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { zint_conj_batch_avx2(a) };
        }
    }
    [a[0].conj(), a[1].conj(), a[2].conj(), a[3].conj()]
}

// ========== MUL ==========

pub fn zint_mul_batch(a: &[ZInt; 4], b: &[ZInt; 4]) -> [ZInt; 4] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

// ========== ARRAY OPS ==========

pub fn zint_add_arrays(a: &[ZInt], b: &[ZInt], out: &mut [ZInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    let len = a.len();
    let chunks = len / 4;
    
    for i in 0..chunks {
        let idx = i * 4;
        let a_chunk: &[ZInt; 4] = a[idx..idx+4].try_into().unwrap();
        let b_chunk: &[ZInt; 4] = b[idx..idx+4].try_into().unwrap();
        let result = zint_add_batch(a_chunk, b_chunk);
        out[idx..idx+4].copy_from_slice(&result);
    }
    
    for i in (chunks * 4)..len {
        out[i] = a[i] + b[i];
    }
}

pub fn zint_sub_arrays(a: &[ZInt], b: &[ZInt], out: &mut [ZInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    let len = a.len();
    let chunks = len / 4;
    
    for i in 0..chunks {
        let idx = i * 4;
        let a_chunk: &[ZInt; 4] = a[idx..idx+4].try_into().unwrap();
        let b_chunk: &[ZInt; 4] = b[idx..idx+4].try_into().unwrap();
        let result = zint_sub_batch(a_chunk, b_chunk);
        out[idx..idx+4].copy_from_slice(&result);
    }
    
    for i in (chunks * 4)..len {
        out[i] = a[i] - b[i];
    }
}

pub fn zint_mul_arrays(a: &[ZInt], b: &[ZInt], out: &mut [ZInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    for i in 0..a.len() {
        out[i] = a[i] * b[i];
    }
}

// ========================================================================
// HINT (HURWITZ QUATERNIONS) SIMD
// ========================================================================

// ========== ADD ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn hint_add_batch_avx2(a: &[HInt; 2], b: &[HInt; 2]) -> [HInt; 2] {
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let b_vec = _mm256_loadu_si256(b.as_ptr() as *const __m256i);
    let result = _mm256_add_epi32(a_vec, b_vec);
    
    let mut out = [HInt::zero(); 2];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn hint_add_batch(a: &[HInt; 2], b: &[HInt; 2]) -> [HInt; 2] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { hint_add_batch_avx2(a, b) };
        }
    }
    [a[0] + b[0], a[1] + b[1]]
}

// ========== SUB ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn hint_sub_batch_avx2(a: &[HInt; 2], b: &[HInt; 2]) -> [HInt; 2] {
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let b_vec = _mm256_loadu_si256(b.as_ptr() as *const __m256i);
    let result = _mm256_sub_epi32(a_vec, b_vec);
    
    let mut out = [HInt::zero(); 2];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn hint_sub_batch(a: &[HInt; 2], b: &[HInt; 2]) -> [HInt; 2] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { hint_sub_batch_avx2(a, b) };
        }
    }
    [a[0] - b[0], a[1] - b[1]]
}

// ========== NEG ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn hint_neg_batch_avx2(a: &[HInt; 2]) -> [HInt; 2] {
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let zero = _mm256_setzero_si256();
    let result = _mm256_sub_epi32(zero, a_vec);
    
    let mut out = [HInt::zero(); 2];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn hint_neg_batch(a: &[HInt; 2]) -> [HInt; 2] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { hint_neg_batch_avx2(a) };
        }
    }
    [-a[0], -a[1]]
}

// ========== CONJ ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn hint_conj_batch_avx2(a: &[HInt; 2]) -> [HInt; 2] {
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let mask = _mm256_setr_epi32(1, -1, -1, -1, 1, -1, -1, -1);
    let result = _mm256_sign_epi32(a_vec, mask);
    
    let mut out = [HInt::zero(); 2];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn hint_conj_batch(a: &[HInt; 2]) -> [HInt; 2] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { hint_conj_batch_avx2(a) };
        }
    }
    [a[0].conj(), a[1].conj()]
}

// ========== MUL ==========

pub fn hint_mul_batch(a: &[HInt; 2], b: &[HInt; 2]) -> [HInt; 2] {
    [a[0] * b[0], a[1] * b[1]]
}

// ========== ARRAY OPS ==========

pub fn hint_add_arrays(a: &[HInt], b: &[HInt], out: &mut [HInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    let len = a.len();
    let chunks = len / 2;
    
    for i in 0..chunks {
        let idx = i * 2;
        let a_chunk: &[HInt; 2] = a[idx..idx+2].try_into().unwrap();
        let b_chunk: &[HInt; 2] = b[idx..idx+2].try_into().unwrap();
        let result = hint_add_batch(a_chunk, b_chunk);
        out[idx..idx+2].copy_from_slice(&result);
    }
    
    for i in (chunks * 2)..len {
        out[i] = a[i] + b[i];
    }
}

pub fn hint_sub_arrays(a: &[HInt], b: &[HInt], out: &mut [HInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    let len = a.len();
    let chunks = len / 2;
    
    for i in 0..chunks {
        let idx = i * 2;
        let a_chunk: &[HInt; 2] = a[idx..idx+2].try_into().unwrap();
        let b_chunk: &[HInt; 2] = b[idx..idx+2].try_into().unwrap();
        let result = hint_sub_batch(a_chunk, b_chunk);
        out[idx..idx+2].copy_from_slice(&result);
    }
    
    for i in (chunks * 2)..len {
        out[i] = a[i] - b[i];
    }
}

pub fn hint_mul_arrays(a: &[HInt], b: &[HInt], out: &mut [HInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    for i in 0..a.len() {
        out[i] = a[i] * b[i];
    }
}

// ========================================================================
// OINT (INTEGER OCTONIONS) SIMD - AVX2 for 8D
// ========================================================================

// ========== ADD ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn oint_add_batch_avx2(a: &[OInt; 1], b: &[OInt; 1]) -> [OInt; 1] {
    let a_ptr = &a[0] as *const OInt as *const i32;
    let b_ptr = &b[0] as *const OInt as *const i32;
    
    let a_vec = _mm256_loadu_si256(a_ptr as *const __m256i);
    let b_vec = _mm256_loadu_si256(b_ptr as *const __m256i);
    let result = _mm256_add_epi32(a_vec, b_vec);
    
    let mut out = [OInt::zero(); 1];
    _mm256_storeu_si256(&mut out[0] as *mut OInt as *mut __m256i, result);
    out
}

pub fn oint_add_batch(a: &[OInt; 1], b: &[OInt; 1]) -> [OInt; 1] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { oint_add_batch_avx2(a, b) };
        }
    }
    [a[0] + b[0]]
}

// ========== SUB ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn oint_sub_batch_avx2(a: &[OInt; 1], b: &[OInt; 1]) -> [OInt; 1] {
    let a_ptr = &a[0] as *const OInt as *const i32;
    let b_ptr = &b[0] as *const OInt as *const i32;
    
    let a_vec = _mm256_loadu_si256(a_ptr as *const __m256i);
    let b_vec = _mm256_loadu_si256(b_ptr as *const __m256i);
    let result = _mm256_sub_epi32(a_vec, b_vec);
    
    let mut out = [OInt::zero(); 1];
    _mm256_storeu_si256(&mut out[0] as *mut OInt as *mut __m256i, result);
    out
}

pub fn oint_sub_batch(a: &[OInt; 1], b: &[OInt; 1]) -> [OInt; 1] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { oint_sub_batch_avx2(a, b) };
        }
    }
    [a[0] - b[0]]
}

// ========== NEG ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn oint_neg_batch_avx2(a: &[OInt; 1]) -> [OInt; 1] {
    let a_ptr = &a[0] as *const OInt as *const i32;
    
    let a_vec = _mm256_loadu_si256(a_ptr as *const __m256i);
    let zero = _mm256_setzero_si256();
    let result = _mm256_sub_epi32(zero, a_vec);
    
    let mut out = [OInt::zero(); 1];
    _mm256_storeu_si256(&mut out[0] as *mut OInt as *mut __m256i, result);
    out
}

pub fn oint_neg_batch(a: &[OInt; 1]) -> [OInt; 1] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { oint_neg_batch_avx2(a) };
        }
    }
    [-a[0]]
}

// ========== CONJ (8D sign mask) ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn oint_conj_batch_avx2(a: &[OInt; 1]) -> [OInt; 1] {
    let a_ptr = &a[0] as *const OInt as *const i32;
    
    let a_vec = _mm256_loadu_si256(a_ptr as *const __m256i);
    let mask = _mm256_setr_epi32(1, -1, -1, -1, -1, -1, -1, -1);
    let result = _mm256_sign_epi32(a_vec, mask);
    
    let mut out = [OInt::zero(); 1];
    _mm256_storeu_si256(&mut out[0] as *mut OInt as *mut __m256i, result);
    out
}

pub fn oint_conj_batch(a: &[OInt; 1]) -> [OInt; 1] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { oint_conj_batch_avx2(a) };
        }
    }
    [a[0].conj()]
}

// ========== MUL (Scalar - Fano plane too complex) ==========

pub fn oint_mul_batch(a: &[OInt; 1], b: &[OInt; 1]) -> [OInt; 1] {
    [a[0] * b[0]]
}

// ========== ARRAY OPS WITH AVX2 ==========

pub fn oint_add_arrays(a: &[OInt], b: &[OInt], out: &mut [OInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    let len = a.len();
    
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                for i in 0..len {
                    let a_ptr = &a[i] as *const OInt as *const i32;
                    let b_ptr = &b[i] as *const OInt as *const i32;
                    
                    let a_vec = _mm256_loadu_si256(a_ptr as *const __m256i);
                    let b_vec = _mm256_loadu_si256(b_ptr as *const __m256i);
                    let result = _mm256_add_epi32(a_vec, b_vec);
                    
                    _mm256_storeu_si256(&mut out[i] as *mut OInt as *mut __m256i, result);
                }
            }
            return;
        }
    }
    
    for i in 0..len {
        out[i] = a[i] + b[i];
    }
}

pub fn oint_sub_arrays(a: &[OInt], b: &[OInt], out: &mut [OInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    let len = a.len();
    
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            unsafe {
                for i in 0..len {
                    let a_ptr = &a[i] as *const OInt as *const i32;
                    let b_ptr = &b[i] as *const OInt as *const i32;
                    
                    let a_vec = _mm256_loadu_si256(a_ptr as *const __m256i);
                    let b_vec = _mm256_loadu_si256(b_ptr as *const __m256i);
                    let result = _mm256_sub_epi32(a_vec, b_vec);
                    
                    _mm256_storeu_si256(&mut out[i] as *mut OInt as *mut __m256i, result);
                }
            }
            return;
        }
    }
    
    for i in 0..len {
        out[i] = a[i] - b[i];
    }
}

pub fn oint_mul_arrays(a: &[OInt], b: &[OInt], out: &mut [OInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    for i in 0..a.len() {
        out[i] = a[i] * b[i];
    }
}

