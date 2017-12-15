//! AltiVec
//!
//! The reference is the [Power ISA v.2.03][altivec_ref].
//!
//! [altivec_ref]: http://www.power.org/wp-content/uploads/2012/06/PowerISA_203_Final_Public.pdf

#[cfg(test)]
use stdsimd_test::assert_instr;

use v128::i8x16;

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.ppc.altivec.vavgsb"]
    fn vavgsb_(a: i8x16, b: i8x16) -> i8x16;
}

#[inline(always)]
#[target_feature = "altivec"]
#[cfg_attr(test, assert_instr(vavgsb))]
unsafe fn vavgsb(a: i8x16, b: i8x16) -> i8x16 {
    vavgsb_(a, b)
}

#[cfg(test)]
mod tests {
    use stdsimd_test::simd_test;
    use simd::*;
    use powerpc::altivec;

    #[simd_test = "altivec"]
    unsafe fn vavgsb() {
        let a = i8x16::splat(1);
        let b = i8x16::splat(2);
        let e = i8x16::splat(4);
        let r = altivec::vavgsb(a, b);
        assert_eq!(r, e);
    }
}
