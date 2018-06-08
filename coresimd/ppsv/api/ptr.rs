//! Minimal pointer vector API

/// Minimal interface: all packed SIMD pointer vector types implement this.
macro_rules! impl_minimal_ptr {
    ($id:ident, $elem_count:expr, $isize_ty:ident, $usize_ty:ident, $mask_ty:ident,
     $($elem_name:ident),+) => {
        impl<T: super::api::Pointer> $id<T> {
            /// Creates a new instance with each vector elements initialized
            /// with the provided values.
            pub const fn new($($elem_name: T),*) -> Self {
                $id($($elem_name),*)
            }

            /// Returns the number of vector lanes.
            #[inline]
            pub const fn lanes() -> usize {
                $elem_count
            }

            /// Constructs a new instance with each element initialized to
            /// `value`.
            #[inline]
            pub const fn splat(value: T) -> Self {
                $id($({
                    #[allow(non_camel_case_types, dead_code)]
                    struct $elem_name;
                    value
                }),*)
            }

            /// Extracts the value at `index`.
            ///
            /// # Panics
            ///
            /// If `index >= Self::lanes()`.
            #[inline]
            pub fn extract(self, index: usize) -> T {
                assert!(index < $elem_count);
                unsafe { self.extract_unchecked(index) }
            }

            /// Extracts the value at `index`.
            ///
            /// # Precondition
            ///
            /// If `index >= Self::lanes()` the behavior is undefined.
            #[inline]
            pub unsafe fn extract_unchecked(self, index: usize) -> T {
                use coresimd::simd_llvm::simd_extract;
                simd_extract(self, index as u32)
            }

            /// Returns a new vector where the value at `index` is replaced by `new_value`.
            ///
            /// # Panics
            ///
            /// If `index >= Self::lanes()`.
            #[inline]
            #[must_use = "replace does not modify the original value - it returns a new vector with the value at `index` replaced by `new_value`d"]
            pub fn replace(self, index: usize, new_value: T) -> Self {
                assert!(index < $elem_count);
                unsafe { self.replace_unchecked(index, new_value) }
            }

            /// Returns a new vector where the value at `index` is replaced by `new_value`.
            ///
            /// # Precondition
            ///
            /// If `index >= Self::lanes()` the behavior is undefined.
            #[inline]
            #[must_use = "replace_unchecked does not modify the original value - it returns a new vector with the value at `index` replaced by `new_value`d"]
            pub unsafe fn replace_unchecked(
                self,
                index: usize,
                new_value: T,
            ) -> Self {
                use coresimd::simd_llvm::simd_insert;
                simd_insert(self, index as u32, new_value)
            }

            /*
            pub fn wrapping_add(self, count: $usize_ty) -> Self {
                unsafe {
                    #[cfg(target_pointer_width = "64")]
                    let sz = ::mem::size_of::<T>() as u64;
                    #[cfg(target_pointer_width = "32")]
                    let sz = ::mem::size_of::<T>() as u32;

                    let ptr: $usize_ty =  ::mem::transmute(self);


                    let ptr = ptr + $usize_ty::splat(sz) * count;
                    ::mem::transmute(ptr)
                }
            }

            pub fn wrapping_sub(self, count: $usize_ty) -> Self {
                unsafe {
                    union U {
                        x: $id<T>,
                        y: $usize_ty,
                    }
                    let ptr: $usize_ty =  U { x: self }.y;
                    #[cfg(target_pointer_width = "64")]
                    let sz = ::mem::size_of::<T>() as u64;
                    #[cfg(target_pointer_width = "32")]
                    let sz = ::mem::size_of::<T>() as u32;


                    let ptr = ptr - $usize_ty::splat(sz) * count;
                    U { y: ptr }.x
                }
            }

            pub fn wrapping_offset(self, count: $isize_ty) -> Self {
                unsafe {
                    union U {
                        x: $id<T>,
                        y: $isize_ty,
                    }

                    let ptr: $isize_ty =  U { x: self }.y;

                    #[cfg(target_pointer_width = "64")]
                    let sz = ::mem::size_of::<T>() as i64;
                    #[cfg(target_pointer_width = "32")]
                    let sz = ::mem::size_of::<T>() as i32;

                    let ptr = ptr + $isize_ty::splat(sz) * count;
                    U { y: ptr }.x
                }
            }
            */

            //pub fn is_null(self) -> $mask_ty {
            //    self.eq(Self::splat(T::null()))
            //}

            pub unsafe fn read<Value>(self) -> Value
                where Value: super::api::Lanes<[T; $elem_count]> + ::default::Default,
                      Self: super::api::Lanes<[*const T; $elem_count]>,
            {
                $mask_ty::splat(true).read(self, ::default::Default::default())
            }

            pub unsafe fn write<Value>(self, value: Value) -> ()
                where Value: super::api::Lanes<[T; $elem_count]>,
                      Self: super::api::Lanes<[*mut T; $elem_count]>,
            {
                $mask_ty::splat(true).write(self, value)
            }
        }
    }
}
