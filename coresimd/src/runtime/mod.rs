//! Run-time feature detection
pub mod cache;
pub mod bit;

#[macro_use]
pub mod macros;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[macro_use]
pub mod x86;

#[cfg(target_arch = "arm")]
#[macro_use]
pub mod arm;

#[cfg(target_arch = "aarch64")]
#[macro_use]
pub mod aarch64;

#[cfg(target_arch = "powerpc64")]
#[macro_use]
pub mod powerpc64;

#[cfg(all(target_os = "linux",
          any(target_arch = "arm", target_arch = "aarch64",
              target_arch = "powerpc64")))]
pub mod linux;

pub mod arch {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub use super::x86::{detect_features, __Feature};

    #[cfg(target_arch = "arm")]
    pub use runtime::arm::{detect_features, __Feature};

    #[cfg(target_arch = "aarch64")]
    pub use runtime::aarch64::{detect_features, __Feature};

    #[cfg(target_arch = "powerpc64")]
    pub use runtime::powerpc64::{detect_features, __Feature};

    /// Interface for querying whether a feature is enabled
    pub trait HasFeature {
        fn has_feature(&mut self, x: &__Feature) -> bool;
    }
}

/// Run-time feature detection exposed by `coresimd`.
pub mod core {
    pub use super::arch::__Feature;

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub use super::arch::detect_features;

    #[cfg(all(target_os = "linux",
              any(target_arch = "arm", target_arch = "aarch64",
                  target_arch = "powerpc64")))]
    pub use super::linux::detect_features;

    /// Performs run-time feature detection.
    #[doc(hidden)]
    pub fn __unstable_detect_feature(x: __Feature) -> bool {
        super::cache::test(x as u32, detect_features)
    }

}
