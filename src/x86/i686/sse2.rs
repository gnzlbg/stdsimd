//! i686's Streaming SIMD Extensions 2 (SSE2)

#[cfg(test)]
use stdsimd_test::assert_instr;

use v128::*;

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.x86.sse2.cvtsd2si64"]
    fn cvtsd2si64(a: f64x2) -> i64;
    #[link_name = "llvm.x86.sse2.cvttsd2si64"]
    fn cvttsd2si64(a: f64x2) -> i64;
}

/// Return `a` with its lower element replaced by `b` after converting it to
/// an `f64`.
#[inline(always)]
#[target_feature = "+sse2"]
#[cfg_attr(test, assert_instr(cvtsi2sd))]
pub unsafe fn _mm_cvtsi64_sd(a: f64x2, b: i64) -> f64x2 {
    a.replace(0, b as f64)
}

/// Return `a` with its lower element replaced by `b` after converting it to
/// an `f64`.
#[inline(always)]
#[target_feature = "+sse2"]
#[cfg_attr(test, assert_instr(cvtsi2sd))]
pub unsafe fn _mm_cvtsi64x_sd(a: f64x2, b: i64) -> f64x2 {
    _mm_cvtsi64_sd(a, b)
}

/// Return a vector whose lowest element is `a` and all higher elements are
/// `0`.
#[inline(always)]
#[target_feature = "+sse2"]
// no particular instruction to test
pub unsafe fn _mm_cvtsi64_si128(a: i64) -> i64x2 {
    i64x2::new(a, 0)
}

/// Return a vector whose lowest element is `a` and all higher elements are
/// `0`.
#[inline(always)]
#[target_feature = "+sse2"]
// no particular instruction to test
pub unsafe fn _mm_cvtsi64x_si128(a: i64) -> i64x2 {
    _mm_cvtsi64_si128(a)
}

/// Return the lowest element of `a`.
#[inline(always)]
#[target_feature = "+sse2"]
// no particular instruction to test
pub unsafe fn _mm_cvtsi128_si64(a: i64x2) -> i64 {
    a.extract(0)
}

/// Return the lowest element of `a`.
#[inline(always)]
#[target_feature = "+sse2"]
// no particular instruction to test
pub unsafe fn _mm_cvtsi128_si64x(a: i64x2) -> i64 {
    _mm_cvtsi128_si64(a)
}

/// Convert the lower double-precision (64-bit) floating-point element in a to
/// a 64-bit integer.
#[inline(always)]
#[target_feature = "+sse2"]
#[cfg_attr(test, assert_instr(cvtsd2si))]
pub unsafe fn _mm_cvtsd_si64(a: f64x2) -> i64 {
    cvtsd2si64(a)
}

/// Alias for [`_mm_cvtsd_si64`](fn._mm_cvtsd_si64_ss.html).
#[inline(always)]
#[target_feature = "+sse2"]
#[cfg_attr(test, assert_instr(cvtsd2si))]
pub unsafe fn _mm_cvtsd_si64x(a: f64x2) -> i64 {
    _mm_cvtsd_si64(a)
}

/// Convert the lower double-precision (64-bit) floating-point element in `a`
/// to a 64-bit integer with truncation.
#[inline(always)]
#[target_feature = "+sse2"]
#[cfg_attr(test, assert_instr(cvttsd2si))]
pub unsafe fn _mm_cvttsd_si64(a: f64x2) -> i64 {
    cvttsd2si64(a)
}

/// Alias for [`_mm_cvttsd_si64`](fn._mm_cvttsd_si64_ss.html).
#[inline(always)]
#[target_feature = "+sse2"]
#[cfg_attr(test, assert_instr(cvttsd2si))]
pub unsafe fn _mm_cvttsd_si64x(a: f64x2) -> i64 {
    _mm_cvttsd_si64(a)
}

#[cfg(test)]
mod tests {
    use stdsimd_test::simd_test;

    use v128::*;
    use x86::i686::sse2;

    #[simd_test = "sse2"]
    unsafe fn _mm_cvtsi64_sd() {
        let a = f64x2::splat(3.5);
        let r = sse2::_mm_cvtsi64_sd(a, 5);
        assert_eq!(r, f64x2::new(5.0, 3.5));
    }

    #[simd_test = "sse2"]
    unsafe fn _mm_cvtsi64_si128() {
        let r = sse2::_mm_cvtsi64_si128(5);
        assert_eq!(r, i64x2::new(5, 0));
    }

    #[simd_test = "sse2"]
    unsafe fn _mm_cvtsi128_si64() {
        let r = sse2::_mm_cvtsi128_si64(i64x2::new(5, 0));
        assert_eq!(r, 5);
    }

    #[simd_test = "sse2"]
    unsafe fn _mm_cvtsd_si64() {
        use std::{f64, i64};

        let r = sse2::_mm_cvtsd_si64(f64x2::new(-2.0, 5.0));
        assert_eq!(r, -2_i64);

        let r = sse2::_mm_cvtsd_si64(f64x2::new(f64::MAX, f64::MIN));
        assert_eq!(r, i64::MIN);
    }

    #[simd_test = "sse2"]
    unsafe fn _mm_cvtsd_si64x() {
        use std::{f64, i64};

        let r = sse2::_mm_cvtsd_si64x(f64x2::new(f64::NAN, f64::NAN));
        assert_eq!(r, i64::MIN);
    }

    #[simd_test = "sse2"]
    unsafe fn _mm_cvttsd_si64() {
        let a = f64x2::new(-1.1, 2.2);
        let r = sse2::_mm_cvttsd_si64(a);
        assert_eq!(r, -1_i64);
    }

    #[simd_test = "sse2"]
    unsafe fn _mm_cvttsd_si64x() {
        use std::{f64, i64};

        let a = f64x2::new(f64::NEG_INFINITY, f64::NAN);
        let r = sse2::_mm_cvttsd_si64x(a);
        assert_eq!(r, i64::MIN);
    }
}
