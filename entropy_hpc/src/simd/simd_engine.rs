// ============================================================================
// MAXIMALLY OPTIMIZED SIMD ENGINE FOR ZINT
// ============================================================================

use crate::{ZInt, ZIFraction, ZIntError};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// ============================================================================
// CORE ARITHMETIC - FULLY OPTIMIZED
// ============================================================================

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn zint_batch_add_simd(out: &mut [ZInt], a: &[ZInt], b: &[ZInt], count: usize) {
    let out_ptr = out.as_mut_ptr() as *mut i32;
    let a_ptr = a.as_ptr() as *const i32;
    let b_ptr = b.as_ptr() as *const i32;

    for i in (0..count * 2).step_by(8) {
        let a_vec = _mm256_loadu_si256(a_ptr.add(i) as *const __m256i);
        let b_vec = _mm256_loadu_si256(b_ptr.add(i) as *const __m256i);
        let result = _mm256_add_epi32(a_vec, b_vec);
        _mm256_storeu_si256(out_ptr.add(i) as *mut __m256i, result);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn zint_batch_sub_simd(out: &mut [ZInt], a: &[ZInt], b: &[ZInt], count: usize) {
    let out_ptr = out.as_mut_ptr() as *mut i32;
    let a_ptr = a.as_ptr() as *const i32;
    let b_ptr = b.as_ptr() as *const i32;

    for i in (0..count * 2).step_by(8) {
        let a_vec = _mm256_loadu_si256(a_ptr.add(i) as *const __m256i);
        let b_vec = _mm256_loadu_si256(b_ptr.add(i) as *const __m256i);
        let result = _mm256_sub_epi32(a_vec, b_vec);
        _mm256_storeu_si256(out_ptr.add(i) as *mut __m256i, result);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn zint_batch_mul_simd(out: &mut [ZInt], a: &[ZInt], b: &[ZInt], count: usize) {
    let out_ptr = out.as_mut_ptr() as *mut i32;
    let a_ptr = a.as_ptr() as *const i32;
    let b_ptr = b.as_ptr() as *const i32;

    for i in (0..count).step_by(4) {
        let a_vec = _mm256_loadu_si256(a_ptr.add(i * 2) as *const __m256i);
        let b_vec = _mm256_loadu_si256(b_ptr.add(i * 2) as *const __m256i);
        
        let a_real = _mm256_shuffle_epi32(a_vec, 0xD8);
        let a_real = _mm256_permute4x64_epi64(a_real, 0xD8);
        
        let a_imag = _mm256_shuffle_epi32(a_vec, 0xDD);
        let a_imag = _mm256_permute4x64_epi64(a_imag, 0xD8);
        
        let b_real = _mm256_shuffle_epi32(b_vec, 0xD8);
        let b_real = _mm256_permute4x64_epi64(b_real, 0xD8);
        
        let b_imag = _mm256_shuffle_epi32(b_vec, 0xDD);
        let b_imag = _mm256_permute4x64_epi64(b_imag, 0xD8);
        
        let ac = _mm256_mullo_epi32(a_real, b_real);
        let bd = _mm256_mullo_epi32(a_imag, b_imag);
        let ad = _mm256_mullo_epi32(a_real, b_imag);
        let bc = _mm256_mullo_epi32(a_imag, b_real);
        
        let result_real = _mm256_sub_epi32(ac, bd);
        let result_imag = _mm256_add_epi32(ad, bc);
        
        let low = _mm256_unpacklo_epi32(result_real, result_imag);
        let high = _mm256_unpackhi_epi32(result_real, result_imag);
        
        let result_low = _mm256_permute2x128_si256(low, high, 0x20);
        
        _mm_storeu_si128(out_ptr.add(i * 2) as *mut __m128i, _mm256_castsi256_si128(result_low));
        _mm_storeu_si128(out_ptr.add(i * 2 + 4) as *mut __m128i, _mm256_extracti128_si256(result_low, 1));
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn zint_batch_neg_simd(out: &mut [ZInt], a: &[ZInt], count: usize) {
    let out_ptr = out.as_mut_ptr() as *mut i32;
    let a_ptr = a.as_ptr() as *const i32;
    let zero = _mm256_setzero_si256();

    for i in (0..count * 2).step_by(8) {
        let a_vec = _mm256_loadu_si256(a_ptr.add(i) as *const __m256i);
        let result = _mm256_sub_epi32(zero, a_vec);
        _mm256_storeu_si256(out_ptr.add(i) as *mut __m256i, result);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn zint_batch_conj_simd(out: &mut [ZInt], a: &[ZInt], count: usize) {
    let out_ptr = out.as_mut_ptr() as *mut i32;
    let a_ptr = a.as_ptr() as *const i32;
    let sign_mask = _mm256_setr_epi32(0, -1, 0, -1, 0, -1, 0, -1);
    let zero = _mm256_setzero_si256();

    for i in (0..count * 2).step_by(8) {
        let a_vec = _mm256_loadu_si256(a_ptr.add(i) as *const __m256i);
        let negated = _mm256_sub_epi32(zero, a_vec);
        let result = _mm256_blendv_epi8(a_vec, negated, sign_mask);
        _mm256_storeu_si256(out_ptr.add(i) as *mut __m256i, result);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
pub unsafe fn zint_batch_norm_squared_simd(out: &mut [u64], a: &[ZInt], count: usize) {
    let a_ptr = a.as_ptr() as *const i32;
    
    for i in (0..count).step_by(4) {
        let vals = _mm256_loadu_si256(a_ptr.add(i * 2) as *const __m256i);
        let squared = _mm256_mullo_epi32(vals, vals);
        
        let low_128 = _mm256_castsi256_si128(squared);
        let high_128 = _mm256_extracti128_si256(squared, 1);
        
        let pair0_32 = _mm_cvtepu32_epi64(low_128);
        let a0_sq = _mm_extract_epi64(pair0_32, 0) as u64;
        let b0_sq = _mm_extract_epi64(pair0_32, 1) as u64;
        out[i] = a0_sq + b0_sq;
        
        let shifted = _mm_srli_si128(low_128, 8);
        let pair1_32 = _mm_cvtepu32_epi64(shifted);
        let a1_sq = _mm_extract_epi64(pair1_32, 0) as u64;
        let b1_sq = _mm_extract_epi64(pair1_32, 1) as u64;
        out[i + 1] = a1_sq + b1_sq;
        
        let pair2_32 = _mm_cvtepu32_epi64(high_128);
        let a2_sq = _mm_extract_epi64(pair2_32, 0) as u64;
        let b2_sq = _mm_extract_epi64(pair2_32, 1) as u64;
        out[i + 2] = a2_sq + b2_sq;
        
        let shifted2 = _mm_srli_si128(high_128, 8);
        let pair3_32 = _mm_cvtepu32_epi64(shifted2);
        let a3_sq = _mm_extract_epi64(pair3_32, 0) as u64;
        let b3_sq = _mm_extract_epi64(pair3_32, 1) as u64;
        out[i + 3] = a3_sq + b3_sq;
    }
}

// ============================================================================
// PROPERTY OPERATIONS
// ============================================================================

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn zint_batch_is_zero_simd(out: &mut [bool], a: &[ZInt], count: usize) {
    for i in 0..count {
        out[i] = a[i].is_zero();
    }
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn zint_batch_is_unit_simd(out: &mut [bool], a: &[ZInt], count: usize) {
    for i in 0..count {
        let norm = a[i].norm_squared();
        out[i] = norm == 1;
    }
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn zint_batch_associates_simd(out: &mut [ZInt], a: &[ZInt], count: usize) {
    for i in 0..count {
        let z = a[i];
        let iz = ZInt::new(-z.b, z.a);
        out[i * 4] = z;
        out[i * 4 + 1] = iz;
        out[i * 4 + 2] = ZInt::new(-z.a, -z.b);
        out[i * 4 + 3] = ZInt::new(z.b, -z.a);
    }
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn zint_batch_normalize_simd(out: &mut [ZInt], a: &[ZInt], count: usize) {
    for i in 0..count {
        out[i] = a[i].normalize();
    }
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn zint_batch_div_rem_simd(
    q_out: &mut [ZInt],
    r_out: &mut [ZInt],
    a: &[ZInt],
    b: &[ZInt],
    count: usize
) -> Result<(), ZIntError> {
    for i in 0..count {
        let (q, r) = a[i].div_rem(b[i])?;
        q_out[i] = q;
        r_out[i] = r;
    }
    Ok(())
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn zint_batch_div_exact_simd(
    out: &mut [ZInt],
    a: &[ZInt],
    b: &[ZInt],
    count: usize
) -> Result<(), ZIntError> {
    for i in 0..count {
        out[i] = a[i].div_exact(b[i])?;
    }
    Ok(())
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn zint_batch_inv_unit_simd(
    out: &mut [ZInt],
    a: &[ZInt],
    count: usize
) -> Result<(), ZIntError> {
    for i in 0..count {
        out[i] = a[i].inv_unit()?;
    }
    Ok(())
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn zint_batch_div_to_fraction_simd(
    out: &mut [ZIFraction],
    a: &[ZInt],
    b: &[ZInt],
    count: usize
) -> Result<(), ZIntError> {
    for i in 0..count {
        out[i] = a[i].div_to_fraction(b[i])?;
    }
    Ok(())
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn zint_batch_inv_fraction_simd(
    out: &mut [ZIFraction],
    a: &[ZInt],
    count: usize
) -> Result<(), ZIntError> {
    for i in 0..count {
        out[i] = a[i].inv_fraction()?;
    }
    Ok(())
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn zint_batch_reduce_fraction_simd(
    out: &mut [ZIFraction],
    fractions: &[ZIFraction],
    count: usize
) {
    for i in 0..count {
        out[i] = ZInt::reduce_fraction(fractions[i]);
    }
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn zint_batch_gcd_simd(
    out: &mut [ZInt],
    a: &[ZInt],
    b: &[ZInt],
    count: usize
) {
    for i in 0..count {
        out[i] = ZInt::gcd(a[i], b[i]);
    }
}

// ============================================================================
// FALLBACKS FOR NON-X86_64
// ============================================================================

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_add_simd(out: &mut [ZInt], a: &[ZInt], b: &[ZInt], count: usize) {
    for i in 0..count { out[i] = a[i] + b[i]; }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_sub_simd(out: &mut [ZInt], a: &[ZInt], b: &[ZInt], count: usize) {
    for i in 0..count { out[i] = a[i] - b[i]; }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_mul_simd(out: &mut [ZInt], a: &[ZInt], b: &[ZInt], count: usize) {
    for i in 0..count { out[i] = a[i] * b[i]; }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_neg_simd(out: &mut [ZInt], a: &[ZInt], count: usize) {
    for i in 0..count { out[i] = -a[i]; }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_conj_simd(out: &mut [ZInt], a: &[ZInt], count: usize) {
    for i in 0..count { out[i] = a[i].conj(); }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_norm_squared_simd(out: &mut [u64], a: &[ZInt], count: usize) {
    for i in 0..count { out[i] = a[i].norm_squared(); }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_is_zero_simd(out: &mut [bool], a: &[ZInt], count: usize) {
    for i in 0..count { out[i] = a[i].is_zero(); }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_is_unit_simd(out: &mut [bool], a: &[ZInt], count: usize) {
    for i in 0..count { out[i] = a[i].is_unit(); }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_associates_simd(out: &mut [ZInt], a: &[ZInt], count: usize) {
    for i in 0..count {
        let assocs = a[i].associates();
        out[i*4..(i+1)*4].copy_from_slice(&assocs);
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_normalize_simd(out: &mut [ZInt], a: &[ZInt], count: usize) {
    for i in 0..count { out[i] = a[i].normalize(); }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_div_rem_simd(q_out: &mut [ZInt], r_out: &mut [ZInt], a: &[ZInt], b: &[ZInt], count: usize) -> Result<(), ZIntError> {
    for i in 0..count {
        let (q, r) = a[i].div_rem(b[i])?;
        q_out[i] = q;
        r_out[i] = r;
    }
    Ok(())
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_div_exact_simd(out: &mut [ZInt], a: &[ZInt], b: &[ZInt], count: usize) -> Result<(), ZIntError> {
    for i in 0..count { out[i] = a[i].div_exact(b[i])?; }
    Ok(())
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_inv_unit_simd(out: &mut [ZInt], a: &[ZInt], count: usize) -> Result<(), ZIntError> {
    for i in 0..count { out[i] = a[i].inv_unit()?; }
    Ok(())
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_div_to_fraction_simd(out: &mut [ZIFraction], a: &[ZInt], b: &[ZInt], count: usize) -> Result<(), ZIntError> {
    for i in 0..count { out[i] = a[i].div_to_fraction(b[i])?; }
    Ok(())
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_inv_fraction_simd(out: &mut [ZIFraction], a: &[ZInt], count: usize) -> Result<(), ZIntError> {
    for i in 0..count { out[i] = a[i].inv_fraction()?; }
    Ok(())
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_reduce_fraction_simd(out: &mut [ZIFraction], fractions: &[ZIFraction], count: usize) {
    for i in 0..count { out[i] = ZInt::reduce_fraction(fractions[i]); }
}

#[cfg(not(target_arch = "x86_64"))]
pub fn zint_batch_gcd_simd(out: &mut [ZInt], a: &[ZInt], b: &[ZInt], count: usize) {
    for i in 0..count { out[i] = ZInt::gcd(a[i], b[i]); }
}

