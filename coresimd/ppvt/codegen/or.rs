//! Code generation for the or reduction.
use ::coresimd::simd::*;

/// LLVM intrinsics used in the or reduction
#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.experimental.vector.reduce.or.i8.i8x2"]
    fn reduce_or_i8x2(x: i8x2) -> i8;
    #[link_name = "llvm.experimental.vector.reduce.or.u8.u8x2"]
    fn reduce_or_u8x2(x: u8x2) -> u8;
    #[link_name = "llvm.experimental.vector.reduce.or.i16.i16x2"]
    fn reduce_or_i16x2(x: i16x2) -> i16;
    #[link_name = "llvm.experimental.vector.reduce.or.u16.u16x2"]
    fn reduce_or_u16x2(x: u16x2) -> u16;
    #[link_name = "llvm.experimental.vector.reduce.or.i32.i32x2"]
    fn reduce_or_i32x2(x: i32x2) -> i32;
    #[link_name = "llvm.experimental.vector.reduce.or.u32.u32x2"]
    fn reduce_or_u32x2(x: u32x2) -> u32;
    #[link_name = "llvm.experimental.vector.reduce.or.i64.i64x2"]
    fn reduce_or_i64x2(x: i64x2) -> i64;
    #[link_name = "llvm.experimental.vector.reduce.or.u64.u64x2"]
    fn reduce_or_u64x2(x: u64x2) -> u64;
    #[link_name = "llvm.experimental.vector.reduce.or.i8.i8x4"]
    fn reduce_or_i8x4(x: i8x4) -> i8;
    #[link_name = "llvm.experimental.vector.reduce.or.u8.u8x4"]
    fn reduce_or_u8x4(x: u8x4) -> u8;
    #[link_name = "llvm.experimental.vector.reduce.or.i16.i16x4"]
    fn reduce_or_i16x4(x: i16x4) -> i16;
    #[link_name = "llvm.experimental.vector.reduce.or.u16.u16x4"]
    fn reduce_or_u16x4(x: u16x4) -> u16;
    #[link_name = "llvm.experimental.vector.reduce.or.i32.i32x4"]
    fn reduce_or_i32x4(x: i32x4) -> i32;
    #[link_name = "llvm.experimental.vector.reduce.or.u32.u32x4"]
    fn reduce_or_u32x4(x: u32x4) -> u32;
    #[link_name = "llvm.experimental.vector.reduce.or.i64.i64x4"]
    fn reduce_or_i64x4(x: i64x4) -> i64;
    #[link_name = "llvm.experimental.vector.reduce.or.u64.u64x4"]
    fn reduce_or_u64x4(x: u64x4) -> u64;
    #[link_name = "llvm.experimental.vector.reduce.or.i8.i8x8"]
    fn reduce_or_i8x8(x: i8x8) -> i8;
    #[link_name = "llvm.experimental.vector.reduce.or.u8.u8x8"]
    fn reduce_or_u8x8(x: u8x8) -> u8;
    #[link_name = "llvm.experimental.vector.reduce.or.i16.i16x8"]
    fn reduce_or_i16x8(x: i16x8) -> i16;
    #[link_name = "llvm.experimental.vector.reduce.or.u16.u16x8"]
    fn reduce_or_u16x8(x: u16x8) -> u16;
    #[link_name = "llvm.experimental.vector.reduce.or.i32.i32x8"]
    fn reduce_or_i32x8(x: i32x8) -> i32;
    #[link_name = "llvm.experimental.vector.reduce.or.u32.u32x8"]
    fn reduce_or_u32x8(x: u32x8) -> u32;
    #[link_name = "llvm.experimental.vector.reduce.or.i64.i64x8"]
    fn reduce_or_i64x8(x: i64x8) -> i64;
    #[link_name = "llvm.experimental.vector.reduce.or.u64.u64x8"]
    fn reduce_or_u64x8(x: u64x8) -> u64;
    #[link_name = "llvm.experimental.vector.reduce.or.i8.i8x16"]
    fn reduce_or_i8x16(x: i8x16) -> i8;
    #[link_name = "llvm.experimental.vector.reduce.or.u8.u8x16"]
    fn reduce_or_u8x16(x: u8x16) -> u8;
    #[link_name = "llvm.experimental.vector.reduce.or.i16.i16x16"]
    fn reduce_or_i16x16(x: i16x16) -> i16;
    #[link_name = "llvm.experimental.vector.reduce.or.u16.u16x16"]
    fn reduce_or_u16x16(x: u16x16) -> u16;
    #[link_name = "llvm.experimental.vector.reduce.or.i32.i32x16"]
    fn reduce_or_i32x16(x: i32x16) -> i32;
    #[link_name = "llvm.experimental.vector.reduce.or.u32.u32x16"]
    fn reduce_or_u32x16(x: u32x16) -> u32;
    #[link_name = "llvm.experimental.vector.reduce.or.i8.i8x32"]
    fn reduce_or_i8x32(x: i8x32) -> i8;
    #[link_name = "llvm.experimental.vector.reduce.or.u8.u8x32"]
    fn reduce_or_u8x32(x: u8x32) -> u8;
    #[link_name = "llvm.experimental.vector.reduce.or.i16.i16x32"]
    fn reduce_or_i16x32(x: i16x32) -> i16;
    #[link_name = "llvm.experimental.vector.reduce.or.u16.u16x32"]
    fn reduce_or_u16x32(x: u16x32) -> u16;
    #[link_name = "llvm.experimental.vector.reduce.or.i8.i8x64"]
    fn reduce_or_i8x64(x: i8x64) -> i8;
    #[link_name = "llvm.experimental.vector.reduce.or.u8.u8x64"]
    fn reduce_or_u8x64(x: u8x64) -> u8;
}

pub trait ReduceOr {
    type Acc;
    fn reduce_or(self) -> Self::Acc;
}

macro_rules! red_or {
    ($id:ident, $elem_ty:ident, $llvm_intr:ident) => {
        impl ReduceOr for $id {
            type Acc = $elem_ty;
            #[inline(always)]
            fn reduce_or(self) -> Self::Acc {
                unsafe { $llvm_intr(self.into_bits()) }
            }
        }
    };
}
red_or!(i8x2, i8, reduce_or_i8x2);
red_or!(u8x2, u8, reduce_or_u8x2);
red_or!(i16x2, i16, reduce_or_i16x2);
red_or!(u16x2, u16, reduce_or_u16x2);
red_or!(i32x2, i32, reduce_or_i32x2);
red_or!(u32x2, u32, reduce_or_u32x2);
red_or!(i64x2, i64, reduce_or_i64x2);
red_or!(u64x2, u64, reduce_or_u64x2);
red_or!(i8x4, i8, reduce_or_i8x4);
red_or!(u8x4, u8, reduce_or_u8x4);
red_or!(i16x4, i16, reduce_or_i16x4);
red_or!(u16x4, u16, reduce_or_u16x4);
red_or!(i32x4, i32, reduce_or_i32x4);
red_or!(u32x4, u32, reduce_or_u32x4);
red_or!(i64x4, i64, reduce_or_i64x4);
red_or!(u64x4, u64, reduce_or_u64x4);
red_or!(i8x8, i8, reduce_or_i8x8);
red_or!(u8x8, u8, reduce_or_u8x8);
red_or!(i16x8, i16, reduce_or_i16x8);
red_or!(u16x8, u16, reduce_or_u16x8);
red_or!(i32x8, i32, reduce_or_i32x8);
red_or!(u32x8, u32, reduce_or_u32x8);
red_or!(i64x8, i64, reduce_or_i64x8);
red_or!(u64x8, u64, reduce_or_u64x8);
red_or!(i8x16, i8, reduce_or_i8x16);
red_or!(u8x16, u8, reduce_or_u8x16);
red_or!(i16x16, i16, reduce_or_i16x16);
red_or!(u16x16, u16, reduce_or_u16x16);
red_or!(i32x16, i32, reduce_or_i32x16);
red_or!(u32x16, u32, reduce_or_u32x16);
red_or!(i8x32, i8, reduce_or_i8x32);
red_or!(u8x32, u8, reduce_or_u8x32);
red_or!(i16x32, i16, reduce_or_i16x32);
red_or!(u16x32, u16, reduce_or_u16x32);
red_or!(i8x64, i8, reduce_or_i8x64);
red_or!(u8x64, u8, reduce_or_u8x64);

red_or!(b8x2, i8, reduce_or_i8x2);
red_or!(b16x2, i16, reduce_or_i16x2);
red_or!(b32x2, i32, reduce_or_i32x2);
red_or!(b64x2, i64, reduce_or_i64x2);
red_or!(b8x4, i8, reduce_or_i8x4);
red_or!(b16x4, i16, reduce_or_i16x4);
red_or!(b32x4, i32, reduce_or_i32x4);
red_or!(b64x4, i64, reduce_or_i64x4);
red_or!(b8x8, i8, reduce_or_i8x8);
red_or!(b16x8, i16, reduce_or_i16x8);
red_or!(b32x8, i32, reduce_or_i32x8);
red_or!(b64x8, i64, reduce_or_i64x8);
red_or!(b8x16, i8, reduce_or_i8x16);
red_or!(b16x16, i16, reduce_or_i16x16);
red_or!(b32x16, i32, reduce_or_i32x16);
red_or!(b8x32, i8, reduce_or_i8x32);
red_or!(b16x32, i16, reduce_or_i16x32);
red_or!(b8x64, i8, reduce_or_i8x64);

#[cfg(test)]
mod tests {
    use super::ReduceOr;
    use ::coresimd::simd::*;

    // note: these are tested in the portable vector API tests

    #[test]
    fn reduce_or_i32x4() {
        let v = i32x4::splat(1);
        assert_eq!(v.reduce_or(), 1_i32);
        let v = i32x4::new(1, 1, 0, 1);
        assert_eq!(v.reduce_or(), 1_i32);
    }
}
