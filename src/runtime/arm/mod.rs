//! Run-time feature detection on arm-like architectures.
use super::bit;

#[macro_export]
#[doc(hidden)]
macro_rules! __unstable_detect_feature {
    ("neon") => {
        $crate::vendor::__unstable_detect_feature($crate::vendor::__Feature::neon{})
    };
    ("asimd") => {
        $crate::vendor::__unstable_detect_feature($crate::vendor::__Feature::asimd{})
    };
    ("pmull") => {
        $crate::vendor::__unstable_detect_feature($crate::vendor::__Feature::pmull{})
    };
    ($t:tt) => { compile_error!(concat!("unknown arm target feature: ", $t)) };
}

/// ARM CPU Feature enum. Each variant denotes a position in a bitset for a
/// particular feature.
///
/// PLEASE: do not use this, it is an implementation detail subject to change.
#[doc(hidden)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum __Feature {
    /// ARM Advanced SIMD (NEON) - Aarch32
    neon,
    /// ARM Advanced SIMD (ASIMD) - Aarch64
    asimd,
    /// Polynomial Multiply
    pmull,
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::detect_features;
