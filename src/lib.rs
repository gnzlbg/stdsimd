//! SIMD and vendor intrinsics support library.
//!
//! This documentation is only for one particular architecture, you can find
//! others at:
//!
//! * [i686](https://rust-lang-nursery.github.io/stdsimd/i686/stdsimd/)
//! * [`x86_64`](https://rust-lang-nursery.github.io/stdsimd/x86_64/stdsimd/)
//! * [arm](https://rust-lang-nursery.github.io/stdsimd/arm/stdsimd/)
//! * [aarch64](https://rust-lang-nursery.github.io/stdsimd/aarch64/stdsimd/)
//! * [powerpc](https://rust-lang-nursery.github.io/stdsimd/powerpc/stdsimd/)
//! * [powerpc64](https://rust-lang-nursery.github.io/stdsimd/powerpc64/stdsimd/)
//!
//! # Overview
//!
//! The `simd` module exposes *portable vector types*. These types work on all
//! platforms, but their run-time performance may vary depending on hardware
//! support.
//!
//! The `vendor` module exposes vendor-specific intrinsics that typically
//! correspond to a single machine instruction. In general, these intrinsics
//! are not portable: their availability is architecture-dependent, and not all
//! machines of that architecture might provide the intrinsic.
//!
//! Two macros make it possible to write portable code:
//!
//! * `cfg!(target_feature = "feature")`: returns `true` if the `feature` is
//! enabled in all CPUs that the binary will run on (at compile-time)
//! * `cfg_feature_enabled!("feature")`: returns `true` if the `feature` is
//! enabled in the CPU in which the binary is currently running on (at
//! run-time, unless the result is known at compile time)
//!
//! # Example
//!
//! ```rust
//! #![feature(cfg_target_feature, target_feature)]
//!
//! #[macro_use]
//! extern crate stdsimd;
//! use stdsimd::vendor;
//! use stdsimd::simd::i32x4;
//!
//! fn main() {
//!     let a = i32x4::new(1, 2, 3, 4);
//!     let b = i32x4::splat(10);
//!     assert_eq!(b, i32x4::new(10, 10, 10, 10));
//!     let c = a + b;
//!     assert_eq!(c, i32x4::new(11, 12, 13, 14));
//!     assert_eq!(sum_portable(b), 40);
//!     assert_eq!(sum_ct(b), 40);
//!     assert_eq!(sum_rt(b), 40);
//! }
//!
//! // Sums the elements of the vector.
//! fn sum_portable(x: i32x4) -> i32 {
//!     let mut r = 0;
//!     for i in 0..4 {
//!         r += x.extract(i);
//!     }
//!     r
//! }
//!
//! // Sums the elements of the vector using SSE2 instructions.
//! // This function is only safe to call if the CPU where the
//! // binary runs supports SSE2.
//! #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
//! #[target_feature(enable = "sse2")]
//! unsafe fn sum_sse2(x: i32x4) -> i32 {
//!     use std::mem;
//!     let x: vendor::__m128i = mem::transmute(x);
//!     let x = vendor::_mm_add_epi32(x, vendor::_mm_srli_si128(x, 8));
//!     let x = vendor::_mm_add_epi32(x, vendor::_mm_srli_si128(x, 4));
//!     let ret = vendor::_mm_cvtsi128_si32(x);
//!     mem::transmute(ret)
//! }
//!
//! // Uses the SSE2 version if SSE2 is enabled for all target
//! // CPUs at compile-time (does not perform any run-time
//! // feature detection).
//! fn sum_ct(x: i32x4) -> i32 {
//!     #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"),
//!               target_feature = "sse2"))]
//!     {
//!         // This function is only available for x86/x86_64 targets,
//!         // and is only safe to call it if the target supports SSE2
//!         unsafe { sum_sse2(x) }
//!     }
//!     #[cfg(not(all(any(target_arch = "x86_64", target_arch = "x86"),
//!               target_feature = "sse2")))]
//!     {
//!         sum_portable(x)
//!     }
//! }
//!
//! // Detects SSE2 at run-time, and uses a SIMD intrinsic if enabled.
//! fn sum_rt(x: i32x4) -> i32 {
//!     #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
//!     {
//!         // If SSE2 is not enabled at compile-time, this
//!         // detects whether SSE2 is available at run-time:
//!         if cfg_feature_enabled!("sse2") {
//!             return unsafe { sum_sse2(x) };
//!         }
//!     }
//!     sum_portable(x)
//! }
//! ```
//!
//! # Status
//!
//! This crate is intended for eventual inclusion into the standard library,
//! but some work and experimentation is needed to get there! First and
//! foremost you can help out by kicking the tires on this crate and seeing if
//! it works for your use case! Next up you can help us fill out the [vendor
//! intrinsics][vendor] to ensure that we've got all the SIMD support
//! necessary.
//!
//! The language support and status of SIMD is also still a little up in the
//! air right now, you may be interested in a few issues along these lines:
//!
//! * [Overal tracking issue for SIMD support][simd_tracking_issue]
//! * [`cfg_target_feature` tracking issue][cfg_target_feature_issue]
//! * [SIMD types currently not sound][simd_soundness_bug]
//! * [`#[target_feature]` improvements][target_feature_impr]
//!
//! [vendor]: https://github.com/rust-lang-nursery/stdsimd/issues/40
//! [simd_tracking_issue]: https://github.com/rust-lang/rust/issues/27731
//! [cfg_target_feature_issue]: https://github.com/rust-lang/rust/issues/29717
//! [simd_soundness_bug]: https://github.com/rust-lang/rust/issues/44367
//! [target_feature_impr]: https://github.com/rust-lang/rust/issues/44839

#![feature(const_fn, const_size_of, use_extern_macros, cfg_target_feature)]
#![cfg_attr(target_os = "linux", feature(linkage))]
extern crate coresimd;

/// Re-export run-time feature detection macros.
#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "arm",
          target_arch = "aarch64", target_arch = "powerpc64"))]
pub use coresimd::__unstable_detect_feature;

/// Platform dependent vendor intrinsics.
pub mod vendor {
    #[doc(inline)]
    pub use coresimd::vendor::*;
}

/// Run-time feature detection.
#[doc(hidden)]
pub mod __vendor_runtime {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",
              all(target_os = "linux",
                  any(target_arch = "arm", target_arch = "aarch64",
                      target_arch = "powerpc64"))))]
    pub use runtime::std::*;
}

/// Platform independent SIMD vector types and operations.
pub mod simd {
    #[doc(inline)]
    pub use coresimd::simd::*;
}

/// The `stdsimd` run-time.
#[macro_use]
#[cfg(any(target_arch = "x86", target_arch = "x86_64",
          all(target_os = "linux",
              any(target_arch = "arm", target_arch = "aarch64",
                  target_arch = "powerpc64"))))]
mod runtime;

/// Error gracefully in architectures without run-time detection support.
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64",
              all(target_os = "linux",
                  any(target_arch = "arm", target_arch = "aarch64",
                      target_arch = "powerpc64")))))]
#[doc(hidden)]
#[macro_export]
macro_rules! cfg_feature_enabled {
    ($name:tt) => (
        {
            compile_error!("cfg_target_feature! is not supported in this architecture")
        }
    )
}
