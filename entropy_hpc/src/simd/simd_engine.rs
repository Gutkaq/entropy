// src/simd_engine.rs

use crate::ZInt;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// ========== SIMD ADDITION (4x ZInt at once) ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn add_batch_avx2(a: &[ZInt; 4], b: &[ZInt; 4]) -> [ZInt; 4] {
    // Load 8 i32s from a (each ZInt is 2 i32s)
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let b_vec = _mm256_loadu_si256(b.as_ptr() as *const __m256i);
    
    // Add all 8 components in parallel
    let result = _mm256_add_epi32(a_vec, b_vec);
    
    // Store result
    let mut out = [ZInt::new(0, 0); 4];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn add_batch(a: &[ZInt; 4], b: &[ZInt; 4]) -> [ZInt; 4] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { add_batch_avx2(a, b) };
        }
    }
    
    // Fallback for non-AVX2 or non-x86_64
    [
        a[0] + b[0],
        a[1] + b[1],
        a[2] + b[2],
        a[3] + b[3],
    ]
}

// ========== SIMD SUBTRACTION ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn sub_batch_avx2(a: &[ZInt; 4], b: &[ZInt; 4]) -> [ZInt; 4] {
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let b_vec = _mm256_loadu_si256(b.as_ptr() as *const __m256i);
    
    let result = _mm256_sub_epi32(a_vec, b_vec);
    
    let mut out = [ZInt::new(0, 0); 4];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn sub_batch(a: &[ZInt; 4], b: &[ZInt; 4]) -> [ZInt; 4] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { sub_batch_avx2(a, b) };
        }
    }
    
    [
        a[0] - b[0],
        a[1] - b[1],
        a[2] - b[2],
        a[3] - b[3],
    ]
}

// ========== SIMD NEGATION ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn neg_batch_avx2(a: &[ZInt; 4]) -> [ZInt; 4] {
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let zero = _mm256_setzero_si256();
    
    let result = _mm256_sub_epi32(zero, a_vec);
    
    let mut out = [ZInt::new(0, 0); 4];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn neg_batch(a: &[ZInt; 4]) -> [ZInt; 4] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { neg_batch_avx2(a) };
        }
    }
    
    [-a[0], -a[1], -a[2], -a[3]]
}

// ========== SIMD CONJUGATE ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn conj_batch_avx2(a: &[ZInt; 4]) -> [ZInt; 4] {
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    
    // Create mask: [1, -1, 1, -1, 1, -1, 1, -1]
    // This negates every other i32 (the 'b' components)
    let mask = _mm256_setr_epi32(1, -1, 1, -1, 1, -1, 1, -1);
    
    // Multiply by mask (equivalent to negating b components)
    let result = _mm256_sign_epi32(a_vec, mask);
    
    let mut out = [ZInt::new(0, 0); 4];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn conj_batch(a: &[ZInt; 4]) -> [ZInt; 4] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { conj_batch_avx2(a) };
        }
    }
    
    [a[0].conj(), a[1].conj(), a[2].conj(), a[3].conj()]
}

// ========== SIMD MULTIPLICATION ==========

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn mul_batch_avx2(a: &[ZInt; 4], b: &[ZInt; 4]) -> [ZInt; 4] {
    // Complex multiply: (ar + ai*i)(br + bi*i) = (ar*br - ai*bi) + (ar*bi + ai*br)*i
    
    // Extract components with shuffling
    // a = [a0.r, a0.i, a1.r, a1.i, a2.r, a2.i, a3.r, a3.i]
    let a_vec = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let b_vec = _mm256_loadu_si256(b.as_ptr() as *const __m256i);
    
    // Broadcast real parts: [ar, ar, ar, ar, ...]
    let a_real = _mm256_shuffle_epi32(a_vec, 0b10100000); // Even indices
    let b_real = _mm256_shuffle_epi32(b_vec, 0b10100000);
    
    // Broadcast imag parts: [ai, ai, ai, ai, ...]
    let a_imag = _mm256_shuffle_epi32(a_vec, 0b11110101); // Odd indices
    let b_imag = _mm256_shuffle_epi32(b_vec, 0b11110101);
    
    // Compute ar*br - ai*bi (real part)
    let ar_br = _mm256_mullo_epi32(a_real, b_real);
    let ai_bi = _mm256_mullo_epi32(a_imag, b_imag);
    let real_part = _mm256_sub_epi32(ar_br, ai_bi);
    
    // Compute ar*bi + ai*br (imag part)
    let ar_bi = _mm256_mullo_epi32(a_real, b_imag);
    let ai_br = _mm256_mullo_epi32(a_imag, b_real);
    let imag_part = _mm256_add_epi32(ar_bi, ai_br);
    
    // Interleave real and imag parts back
    let result = _mm256_blend_epi32(real_part, imag_part, 0b10101010);
    
    let mut out = [ZInt::new(0, 0); 4];
    _mm256_storeu_si256(out.as_mut_ptr() as *mut __m256i, result);
    out
}

pub fn mul_batch(a: &[ZInt; 4], b: &[ZInt; 4]) -> [ZInt; 4] {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { mul_batch_avx2(a, b) };
        }
    }
    
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

// ========== SIMD NORM SQUARED ==========

pub fn norm_squared_batch(a: &[ZInt; 4]) -> [u64; 4] {
    // This requires i64 multiplication, scalar is fine for now
    [
        a[0].norm_squared(),
        a[1].norm_squared(),
        a[2].norm_squared(),
        a[3].norm_squared(),
    ]
}

// ========== ARRAY OPERATIONS ==========

pub fn add_arrays(a: &[ZInt], b: &[ZInt], out: &mut [ZInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    let len = a.len();
    let chunks = len / 4;
    
    // Process 4 at a time with SIMD
    for i in 0..chunks {
        let idx = i * 4;
        let a_chunk: &[ZInt; 4] = a[idx..idx+4].try_into().unwrap();
        let b_chunk: &[ZInt; 4] = b[idx..idx+4].try_into().unwrap();
        let result = add_batch(a_chunk, b_chunk);
        out[idx..idx+4].copy_from_slice(&result);
    }
    
    // Handle remainder with scalar
    for i in (chunks * 4)..len {
        out[i] = a[i] + b[i];
    }
}

pub fn sub_arrays(a: &[ZInt], b: &[ZInt], out: &mut [ZInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    let len = a.len();
    let chunks = len / 4;
    
    for i in 0..chunks {
        let idx = i * 4;
        let a_chunk: &[ZInt; 4] = a[idx..idx+4].try_into().unwrap();
        let b_chunk: &[ZInt; 4] = b[idx..idx+4].try_into().unwrap();
        let result = sub_batch(a_chunk, b_chunk);
        out[idx..idx+4].copy_from_slice(&result);
    }
    
    for i in (chunks * 4)..len {
        out[i] = a[i] - b[i];
    }
}

pub fn mul_arrays(a: &[ZInt], b: &[ZInt], out: &mut [ZInt]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), out.len());
    
    let len = a.len();
    let chunks = len / 4;
    
    for i in 0..chunks {
        let idx = i * 4;
        let a_chunk: &[ZInt; 4] = a[idx..idx+4].try_into().unwrap();
        let b_chunk: &[ZInt; 4] = b[idx..idx+4].try_into().unwrap();
        let result = mul_batch(a_chunk, b_chunk);
        out[idx..idx+4].copy_from_slice(&result);
    }
    
    for i in (chunks * 4)..len {
        out[i] = a[i] * b[i];
    }
}

pub fn conj_array(a: &[ZInt], out: &mut [ZInt]) {
    assert_eq!(a.len(), out.len());
    
    let len = a.len();
    let chunks = len / 4;
    
    for i in 0..chunks {
        let idx = i * 4;
        let a_chunk: &[ZInt; 4] = a[idx..idx+4].try_into().unwrap();
        let result = conj_batch(a_chunk);
        out[idx..idx+4].copy_from_slice(&result);
    }
    
    for i in (chunks * 4)..len {
        out[i] = a[i].conj();
    }
}

// ========== TESTS ==========

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_batch() {
        let a = [
            ZInt::new(1, 2),
            ZInt::new(3, 4),
            ZInt::new(5, 6),
            ZInt::new(7, 8),
        ];
        let b = [
            ZInt::new(10, 20),
            ZInt::new(30, 40),
            ZInt::new(50, 60),
            ZInt::new(70, 80),
        ];
        
        let result = add_batch(&a, &b);
        
        assert_eq!(result[0], ZInt::new(11, 22));
        assert_eq!(result[1], ZInt::new(33, 44));
        assert_eq!(result[2], ZInt::new(55, 66));
        assert_eq!(result[3], ZInt::new(77, 88));
    }
    
    #[test]
    fn test_sub_batch() {
        let a = [
            ZInt::new(10, 20),
            ZInt::new(30, 40),
            ZInt::new(50, 60),
            ZInt::new(70, 80),
        ];
        let b = [
            ZInt::new(1, 2),
            ZInt::new(3, 4),
            ZInt::new(5, 6),
            ZInt::new(7, 8),
        ];
        
        let result = sub_batch(&a, &b);
        
        assert_eq!(result[0], ZInt::new(9, 18));
        assert_eq!(result[1], ZInt::new(27, 36));
        assert_eq!(result[2], ZInt::new(45, 54));
        assert_eq!(result[3], ZInt::new(63, 72));
    }
    
    #[test]
    fn test_neg_batch() {
        let a = [
            ZInt::new(1, 2),
            ZInt::new(-3, 4),
            ZInt::new(5, -6),
            ZInt::new(-7, -8),
        ];
        
        let result = neg_batch(&a);
        
        assert_eq!(result[0], ZInt::new(-1, -2));
        assert_eq!(result[1], ZInt::new(3, -4));
        assert_eq!(result[2], ZInt::new(-5, 6));
        assert_eq!(result[3], ZInt::new(7, 8));
    }
    
    #[test]
    fn test_conj_batch() {
        let a = [
            ZInt::new(1, 2),
            ZInt::new(3, -4),
            ZInt::new(-5, 6),
            ZInt::new(-7, -8),
        ];
        
        let result = conj_batch(&a);
        
        assert_eq!(result[0], ZInt::new(1, -2));
        assert_eq!(result[1], ZInt::new(3, 4));
        assert_eq!(result[2], ZInt::new(-5, -6));
        assert_eq!(result[3], ZInt::new(-7, 8));
    }
    
    #[test]
    fn test_mul_batch() {
        let a = [
            ZInt::new(1, 2),
            ZInt::new(3, 4),
            ZInt::new(1, 0),
            ZInt::new(0, 1),
        ];
        let b = [
            ZInt::new(3, 4),
            ZInt::new(1, 2),
            ZInt::new(5, 0),
            ZInt::new(0, 1),
        ];
        
        let result = mul_batch(&a, &b);
        
        // (1+2i)(3+4i) = 3+4i+6i+8i² = 3+10i-8 = -5+10i
        assert_eq!(result[0], ZInt::new(-5, 10));
        // (3+4i)(1+2i) = 3+6i+4i+8i² = 3+10i-8 = -5+10i
        assert_eq!(result[1], ZInt::new(-5, 10));
        // 1*5 = 5
        assert_eq!(result[2], ZInt::new(5, 0));
        // i*i = -1
        assert_eq!(result[3], ZInt::new(-1, 0));
    }
    
    #[test]
    fn test_norm_squared_batch() {
        let a = [
            ZInt::new(3, 4),
            ZInt::new(5, 12),
            ZInt::new(0, 0),
            ZInt::new(1, 0),
        ];
        
        let result = norm_squared_batch(&a);
        
        assert_eq!(result[0], 25);
        assert_eq!(result[1], 169);
        assert_eq!(result[2], 0);
        assert_eq!(result[3], 1);
    }
    
    #[test]
    fn test_add_arrays() {
        let a = vec![
            ZInt::new(1, 2),
            ZInt::new(3, 4),
            ZInt::new(5, 6),
            ZInt::new(7, 8),
            ZInt::new(9, 10), // Remainder test
        ];
        let b = vec![
            ZInt::new(10, 20),
            ZInt::new(30, 40),
            ZInt::new(50, 60),
            ZInt::new(70, 80),
            ZInt::new(90, 100),
        ];
        let mut out = vec![ZInt::new(0, 0); 5];
        
        add_arrays(&a, &b, &mut out);
        
        assert_eq!(out[0], ZInt::new(11, 22));
        assert_eq!(out[1], ZInt::new(33, 44));
        assert_eq!(out[2], ZInt::new(55, 66));
        assert_eq!(out[3], ZInt::new(77, 88));
        assert_eq!(out[4], ZInt::new(99, 110));
    }
    
    #[test]
    fn test_mul_arrays() {
        let a = vec![
            ZInt::new(1, 2),
            ZInt::new(3, 4),
            ZInt::new(5, 6),
            ZInt::new(7, 8),
        ];
        let b = vec![
            ZInt::new(1, 0),
            ZInt::new(0, 1),
            ZInt::new(2, 0),
            ZInt::new(1, 1),
        ];
        let mut out = vec![ZInt::new(0, 0); 4];
        
        mul_arrays(&a, &b, &mut out);
        
        assert_eq!(out[0], ZInt::new(1, 2)); // (1+2i)*1
        assert_eq!(out[1], ZInt::new(-4, 3)); // (3+4i)*i
        assert_eq!(out[2], ZInt::new(10, 12)); // (5+6i)*2
        assert_eq!(out[3], ZInt::new(-1, 15)); // (7+8i)*(1+i)
    }
}

