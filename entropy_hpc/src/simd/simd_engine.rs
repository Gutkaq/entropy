// src/simd/simd_engine.rs

use crate::types::cint::CInt;
use crate::types::hint::HInt;
use crate::types::oint::OInt;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// ========================================================================
// CINT (Complex Integers) SIMD - 4 at a time (8 i32s = 256 bits)
// ========================================================================

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn cint_add_batch_avx2(a: &[CInt; 4], b: &[CInt; 4]) -> [CInt; 4] {
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let b_vec = _mm256_loadu_si256(b.as_ptr() as *const __m256i);
    let result = _mm256_add_epi32(a_vec, b_vec);
    
    let mut out = [CInt::zero(); 4];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn cint_add_batch(a: &[CInt; 4], b: &[CInt; 4]) -> [CInt; 4] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { cint_add_batch_avx2(a, b) };
        }
    }
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn cint_sub_batch_avx2(a: &[CInt; 4], b: &[CInt; 4]) -> [CInt; 4] {
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let b_vec = _mm256_loadu_si256(b.as_ptr() as *const __m256i);
    let result = _mm256_sub_epi32(a_vec, b_vec);
    
    let mut out = [CInt::zero(); 4];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn cint_sub_batch(a: &[CInt; 4], b: &[CInt; 4]) -> [CInt; 4] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { cint_sub_batch_avx2(a, b) };
        }
    }
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

// Mul: Scalar (complex mul is complex for SIMD)
pub fn cint_mul_batch(a: &[CInt; 4], b: &[CInt; 4]) -> [CInt; 4] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

// Array operations: Chunked + tail
pub fn cint_add_arrays(a: &[CInt], b: &[CInt], out: &mut [CInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    let len = a.len();
    let chunks = len / 4;
    
    for i in 0..chunks {
        let idx = i * 4;
        let a_chunk: &[CInt; 4] = a[idx..idx+4].try_into().unwrap();
        let b_chunk: &[CInt; 4] = b[idx..idx+4].try_into().unwrap();
        let result = cint_add_batch(a_chunk, b_chunk);
        out[idx..idx+4].copy_from_slice(&result);
    }
    
    // Tail
    for i in (chunks * 4)..len {
        out[i] = a[i] + b[i];
    }
}

pub fn cint_sub_arrays(a: &[CInt], b: &[CInt], out: &mut [CInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    let len = a.len();
    let chunks = len / 4;
    
    for i in 0..chunks {
        let idx = i * 4;
        let a_chunk: &[CInt; 4] = a[idx..idx+4].try_into().unwrap();
        let b_chunk: &[CInt; 4] = b[idx..idx+4].try_into().unwrap();
        let result = cint_sub_batch(a_chunk, b_chunk);
        out[idx..idx+4].copy_from_slice(&result);
    }
    
    for i in (chunks * 4)..len {
        out[i] = a[i] - b[i];
    }
}

pub fn cint_mul_arrays(a: &[CInt], b: &[CInt], out: &mut [CInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    for i in 0..a.len() {
        out[i] = a[i] * b[i];
    }
}

// ========================================================================
// HINT (Hurwitz Quaternions) SIMD - 2 at a time (8 i32s = 256 bits)
// ========================================================================

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

// Mul: Scalar (quaternion mul too complex for SIMD)
pub fn hint_mul_batch(a: &[HInt; 2], b: &[HInt; 2]) -> [HInt; 2] {
    [a[0] * b[0], a[1] * b[1]]
}

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
// OINT (Integer Octonions) SIMD - 1 at a time (8 i32s = 256 bits)
// ========================================================================

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

// Mul: Scalar (octonion mul with Fano plane is complex)
pub fn oint_mul_batch(a: &[OInt; 1], b: &[OInt; 1]) -> [OInt; 1] {
    [a[0] * b[0]]
}

pub fn oint_add_arrays(a: &[OInt], b: &[OInt], out: &mut [OInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    for i in 0..a.len() {
        let chunk_a = [a[i]];
        let chunk_b = [b[i]];
        let result = oint_add_batch(&chunk_a, &chunk_b);
        out[i] = result[0];
    }
}

pub fn oint_sub_arrays(a: &[OInt], b: &[OInt], out: &mut [OInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    for i in 0..a.len() {
        let chunk_a = [a[i]];
        let chunk_b = [b[i]];
        let result = oint_sub_batch(&chunk_a, &chunk_b);
        out[i] = result[0];
    }
}

pub fn oint_mul_arrays(a: &[OInt], b: &[OInt], out: &mut [OInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    for i in 0..a.len() {
        out[i] = a[i] * b[i];
    }
}

