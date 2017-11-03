//! Run-time feature detection
mod cache;
mod bit;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[macro_use]
mod x86;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use self::x86::{__Feature};
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use self::x86::detect_features;

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
#[macro_use]
mod arm;
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub use self::arm::{__Feature};
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
use self::arm::{detect_features};

/// Performs run-time feature detection.
#[doc(hidden)]
pub fn __unstable_detect_feature(x: __Feature) -> bool {
    cache::test(x as u32, detect_features)
}
