//! `x86` and `x86_64` intrinsics.

/// 128-bit wide signed integer vector type
#[allow(non_camel_case_types)]
pub type __m128i = ::v128::i8x16;
/// 256-bit wide signed integer vector type
#[allow(non_camel_case_types)]
pub type __m256i = ::v256::i8x32;

#[macro_use]
mod macros;

#[macro_use]
mod runtime;

pub use self::runtime::{__unstable_detect_feature, __Feature};

mod i586;
pub use self::i586::*;

// i686: either x86_64, or all x86 targets that have SSE2
#[cfg(any(
    target_arch = "x86_64",
    all(target_arch = "x86", target_feature = "sse2")
))]
mod i686;
#[cfg(any(
    target_arch = "x86_64",
    all(target_arch = "x86", target_feature = "sse2")
))]
pub use self::i686::*;
