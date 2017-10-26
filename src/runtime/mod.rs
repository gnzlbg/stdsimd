//! Run-time feature detection
mod storage;
mod bit;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[macro_use]
mod x86;
#[macro_use]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use self::x86::{__Feature, __unstable_detect_feature, detect_features};

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
#[macro_use]
mod arm;
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
#[macro_use]
pub use self::arm::{__Feature, __unstable_detect_feature, detect_features};

/// Performs run-time feature detection.
#[doc(hidden)]
fn __unstable_detect_feature(x: __Feature) -> bool {
    storage::test(x as u32, detect_features)
}
