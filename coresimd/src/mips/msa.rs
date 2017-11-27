//! MIPS SIMD Architecture intrinsics
//!
//! The reference is [MIPS Architecture for Programmers Volume IV-j: The MIPS32 SIMD
//! Architecture Module Revision 1.12][msa_ref].
//!
//! [msa_ref]: http://cdn2.imgtec.com/documentation/MD00866-2B-MSA32-AFP-01.12.pdf

use simd::i8x16;

#[cfg(test)]
use stdsimd_test::assert_instr;


#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.mips.add.a.b"]
    fn msa_add_a_b(a: i8x16, b: i8x16) -> i8x16;
}

/// Vector Add Absolute Values.
///
/// Adds the absolute values of the elements in `a` and `b` into the result vector.
#[target_feature = "+msa"]
#[cfg_attr(test, assert_instr(add))]
pub unsafe fn __msa_add_a_b(a: i8x16, b: i8x16) -> i8x16 {
    msa_add_a_b(a, b)
}


#[cfg(test)]
mod tests {
    use mips::msa;

    #[test]
    fn __msa_add_a_b() {
        let a = i8x16::new(1, 2, 3, 4,
                           1, 2, 3, 4,
                           1, 2, 3, 4,
                           1, 2, 3, 4,
        );
        let b = i8x16::new(-4, -3, -2, -1,
                           -4, -3, -2, -1,
                           -4, -3, -2, -1,
                           -4, -3, -2, -1,
        );
        let r = i8x16::splat(5);

        assert_eq!(r, msa::__msa_add_a_b(a, b));
    }
}
