//! Pointer vector types

simd_ptr_ty! {
    px2: 2, T, isizex2, usizex2, ptrmx2 | T, T | x0, x1 |
    /// A vector with 2 pointer lanes.
}

simd_ptr_ty! {
    px4: 4, T, isizex4, usizex4, ptrmx4 | T, T, T, T | x0, x1, x2, x3 |
    /// A vector with 4 pointer lanes.
}

simd_ptr_ty! {
    px8: 8, T, isizex8, usizex8, ptrmx8 | T, T, T, T, T, T, T, T |
    x0, x1, x2, x3, x4, x5, x6, x7 |
    /// A vector with 8 pointer lanes.
}

/*
simd_ptr_ty! {
    px16: 16, T, isizex16, usizex16, ptrmx16 |
    T, T, T, T, T, T, T, T,
    T, T, T, T, T, T, T, T |
    x0, x1, x2, x3, x4, x5, x6, x7,
    x8, x9, x10, x11, x12, x13, x14, x15 |
    /// A vector with 16 pointer lanes.
}

simd_ptr_ty! {
    px32: 32, T, isizex32, usizex32, ptrmx32 |
    T, T, T, T, T, T, T, T,
    T, T, T, T, T, T, T, T,
    T, T, T, T, T, T, T, T,
    T, T, T, T, T, T, T, T |
    x0, x1, x2, x3, x4, x5, x6, x7,
    x8, x9, x10, x11, x12, x13, x14, x15,
    x16, x17, x18, x19, x20, x21, x22, x23,
    x24, x25, x26, x27, x28, x29, x30, x31 |
    /// A vector with 32 pointer lanes.
}

simd_ptr_ty! {
    px64: 64, T, isizex64, usizex64, ptrmx64 |
    T, T, T, T, T, T, T, T,
    T, T, T, T, T, T, T, T,
    T, T, T, T, T, T, T, T,
    T, T, T, T, T, T, T, T,
    T, T, T, T, T, T, T, T,
    T, T, T, T, T, T, T, T,
    T, T, T, T, T, T, T, T,
    T, T, T, T, T, T, T, T |
    x0, x1, x2, x3, x4, x5, x6, x7,
    x8, x9, x10, x11, x12, x13, x14, x15,
    x16, x17, x18, x19, x20, x21, x22, x23,
    x24, x25, x26, x27, x28, x29, x30, x31,
    x32, x33, x34, x35, x36, x37, x38, x39,
    x40, x41, x42, x43, x44, x45, x46, x47,
    x48, x49, x50, x51, x52, x53, x54, x55,
    x56, x57, x58, x59, x60, x61, x62, x63 |
    /// A vector with 64 pointer lanes.
}
*/

#[cfg(target_pointer_width = "32")]
mod types {
    pub use ::simd::*;
    pub type usizex2 = u32x2;
    pub type usizex4 = u32x4;
    pub type usizex8 = u32x8;
    //pub type usizex16 = u32x16;
    //pub type usizex32 = u32x32;
    //pub type usizex64 = u32x64;

    pub type isizex2 = i32x2;
    pub type isizex4 = i32x4;
    pub type isizex8 = i32x8;
    //pub type isizex16 = i32x16;
    //pub type isizex32 = i32x32;
    //pub type isizex64 = i32x64;

    pub type ptrmx2 = m32x2;
    pub type ptrmx4 = m32x4;
    pub type ptrmx8 = m32x8;
    //pub type ptrmx16 = m1x16;
    //pub type ptrmx32 = m1x32;
    //pub type ptrmx64 = m1x64;
}

#[cfg(target_pointer_width = "64")]
mod types {
    pub use ::simd::*;
    pub type usizex2 = u64x2;
    pub type usizex4 = u64x4;
    pub type usizex8 = u64x8;
    //pub type usizex16 = u64x16;
    //pub type usizex32 = u64x32;
    //pub type usizex64 = u64x64;

    pub type isizex2 = i64x2;
    pub type isizex4 = i64x4;
    pub type isizex8 = i64x8;
    //pub type isizex16 = i64x16;
    //pub type isizex32 = i64x32;
    //pub type isizex64 = i64x64;

    pub type ptrmx2 = m64x2;
    pub type ptrmx4 = m64x4;
    pub type ptrmx8 = m1x8;
    //pub type ptrmx16 = m1x16;
    //pub type ptrmx32 = m1x32;
    //pub type ptrmx64 = m1x64;
}


pub use self::types::{
    usizex2, usizex4, usizex8, //usizex16, usizex32, usizex64,
    isizex2, isizex4, isizex8, //isizex16, isizex32, isizex64,
};

use self::types::{
    ptrmx2, ptrmx4, ptrmx8, //ptrmx16, ptrmx32, ptrmx64,
};
