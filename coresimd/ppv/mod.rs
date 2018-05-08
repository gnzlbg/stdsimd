//! Portable Packed Vector types
#![allow(non_camel_case_types)]

/// Portable packed SIMD vector
///
///
#[repr(transparent)]
pub struct Pack<A: Packable>(A::Vector);

/// Types that can be packed
pub trait Packable: sealed::Array {}
impl<T: sealed::Array> Packable for T {}

/// 1-bit wide mask
pub struct m1(i64);

/// 8-bit wide mask
pub struct m8(i8);

/// 16-bit wide mask
pub struct m16(i16);

/// 32-bit wide mask
pub struct m32(i32);

/// 64-bit wide mask
pub struct m64(i64);

/// Sealed traits: private implementation details
mod sealed {
    /// Sealed trait
    pub trait Array {
        type Vector;
        type Api;
        type Storage;
        type ApiArray;
        type Input;
        const LANES: usize;

        fn from_storage_to_api(x: Self::Storage) -> Self::Api;
    }

    pub trait Primitive {
        /// Type used to store `Self` inside a `repr(simd)` type.
        type Storage = Self;
        /// Type used in the user-facing APIs
        type Api = Self;

        fn 
    }

    /// Signed integer types
    pub trait Int {}

    /// Unsigned integer types
    pub trait Uint {}

    /// Floating-point types
    pub trait Float {}

    /// Mask types
    pub trait Mask {}

    /// Pointer types
    pub trait Ptr {}

    /// Two-element packed vector
    #[repr(simd)]
    pub struct Pack2<T>(T, T);

    /// Four-element packed vector
    #[repr(simd)]
    pub struct Pack4<T>(T, T, T, T);

    /// Eight-element packed vector
    #[repr(simd)]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub struct Pack8<T>(T, T, T, T, T, T, T, T);

    /// Sixteen-element packed vector
    #[repr(simd)]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub struct Pack16<T>(T, T, T, T, T, T, T, T,
                         T, T, T, T, T, T, T, T);

    /// Thirty-two-element packed vector
    #[repr(simd)]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub struct Pack32<T>(T, T, T, T, T, T, T, T,
                         T, T, T, T, T, T, T, T,
                         T, T, T, T, T, T, T, T,
                         T, T, T, T, T, T, T, T);

    /// Sixty-four-element packed vector
    #[repr(simd)]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub struct Pack64<T>(T, T, T, T, T, T, T, T,
                         T, T, T, T, T, T, T, T,
                         T, T, T, T, T, T, T, T,
                         T, T, T, T, T, T, T, T,
                         T, T, T, T, T, T, T, T,
                         T, T, T, T, T, T, T, T,
                         T, T, T, T, T, T, T, T,
                         T, T, T, T, T, T, T, T);
}

macro_rules! impl_kind {
    ($kind:path, $id:ident) => {
        impl sealed::Primitive for $id {}
        impl $kind for $id {}
    };
    ($kind:path, $id:ident, $storage_ty:ident, $api_ty:ident) => {
        impl sealed::Primitive for $id {
            type Storage = $storage_ty;
            type Api = $api_ty;
        }
        impl $kind for $id {}
    };
    ($kind:path: $($id:ident),*) => { $( impl_kind!($kind, $id); )* };
    ($kind:path: $(($id:ident, $storage_ty:ident, $api_ty:ident)),*) => {
        $( impl_kind!($kind, $id, $storage_ty, $api_ty); )*
    };
}

impl_kind!(sealed::Int: i8, i16, i32, i64);
impl_kind!(sealed::Uint: u8, u16, u32, u64);
impl_kind!(sealed::Float: f32, f64);
impl_kind!(
    sealed::Mask: (m1, i64, bool),
    (m8, i8, bool),
    (m16, i16, bool),
    (m32, i32, bool),
    (m64, i64, bool)
);

impl<T> sealed::Primitive for *const T {}
impl<T> sealed::Primitive for *mut T {}
impl<T> sealed::Ptr for *const T {}
impl<T> sealed::Ptr for *mut T {}

macro_rules! impl_simd_array {
    ($N:expr: $T:ident) => {
        impl<T> sealed::Array for [T; $N]
        where
            T: sealed::Primitive,
        {
            type Vector = sealed::Pack2<T::Storage>;
            type ApiElement = T::Api;
            type StorageElement = T::Storage;
            type ApiArray = [Self::Api; $N];
            type ApiTuple = (Self::)
            type StorageTuple = (Self::, ..., Self::Element);
            type Input = T;
            const LANES: usize = $N;

            fn from_storage_to_api(x: Self::Storage) -> Self::Api {
                
            }
        }
    };
}

impl_simd_array!(2);
impl_simd_array!(4);
impl_simd_array!(6);
impl_simd_array!(8);
impl_simd_array!(16);
impl_simd_array!(32);
impl_simd_array!(64);

macro_rules! minimal_pack {
    ($id:pack, ) => {
        impl $id
    }
}

impl<A: Packable> Pack<A> {
    /// Returns the number of vector lanes.
    #[inline]
    pub const fn lanes() -> usize {
        <A as sealed::Array>::LANES
    }

    /// Extracts the value at `index`.
    ///
    /// # Panics
    ///
    /// If `index >= Self::lanes()`.
    #[inline]
    pub fn extract(self, index: usize) -> <A as sealed::Array>::ApiElement {
        assert!(index < Self::lanes());
        unsafe { self.extract_unchecked(index) }
    }

    /// Extracts the value at `index`.
    ///
    /// # Precondition
    ///
    /// If `index >= Self::lanes()` the behavior is undefined.
    #[inline]
    pub unsafe fn extract_unchecked(self, index: usize) -> <A as sealed::Array>::ApiElement {
        use coresimd::simd_llvm::simd_extract;
        Self::storage_element_to_api(simd_extract(self, index as u32))
    }

}


impl<A: Packable> Pack<A> where A::Input: sealed::Int {
}

impl<A: Packable> Pack<A> where A::Input: sealed::Float {
}



#[cfg(test)]
mod test {
    use super::Pack;
    fn foo() {


    }
}
