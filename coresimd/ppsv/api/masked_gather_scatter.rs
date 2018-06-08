//! Masked gather and scatter
#![allow(unused)]

/// Implements masked gather and scatter operations
macro_rules! impl_masked_gather_scatter {
    ($id:ident, $elem_ty:ident, $elem_count:expr) => {
        impl $id {
            /// Masked pointer vector read.
            ///
            /// Reads `pointer` elements whose mask is `true`, returning elements
            /// of `default` otherwise.
            ///
            /// # Safety
            ///
            /// The pointers being read from must be aligned and be non-null.
            #[inline]
            pub unsafe fn read<P, V, T>(self, pointer: P, default: V) -> V
                where
                V: super::api::Lanes<[T; $elem_count]>,
                P: super::api::Lanes<[*const T; $elem_count]>,
            {
                use coresimd::simd_llvm::simd_gather;
                simd_gather(default, pointer, self)
            }

            /// Masked pointer vector write.
            ///
            /// Writes `values` to `pointer` elements whose mask is `true`.
            ///
            /// # Safety
            ///
            /// The pointers being written to must be aligned and be non-null.
            #[inline]
            pub unsafe fn write<P, V, T>(self, pointer: P, values: V) -> ()
                where
                V: super::api::Lanes<[T; $elem_count]>,
                P: super::api::Lanes<[*mut T; $elem_count]>,

            {
                use coresimd::simd_llvm::simd_scatter;
                simd_scatter(values, pointer, self)
            }
        }
    }
}
