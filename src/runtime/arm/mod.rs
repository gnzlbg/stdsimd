//! Run-time feature detection on arm-like architectures.
mod getauxval;
mod cpuinfo;

#[cfg(target_arch = "arm")]
#[macro_export]
#[doc(hidden)]
macro_rules! __unstable_detect_feature {
    ("neon") => {
        $crate::vendor::__unstable_detect_feature($crate::vendor::__Feature::neon{})
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
    /// Advanced SIMD Extension (NEON)
    neon,
    /// Polynomial Multiply
    pmull,
}

pub fn detect_features() -> usize {
    let value: usize = 0;
    let hwcap = getauxval(hwcap::AT);
    let hwcap2 = getauxval(hwcap2::AT);

    if bit::test(hwcap, hwcap::NEON) {
        bit::set(value, __Feature::neon as u32);
    }
    if bit::test(hwcap, hwcap::PMULL) {
        bit::set(value, __Feature::pmull as u32);
    }
    value
}
