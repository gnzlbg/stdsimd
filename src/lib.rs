//! SIMD support
//!
//! This crate provides the fundamentals of supporting SIMD in Rust. This crate
//! should compile on all platforms and provide `simd` and `vendor` modules at
//! the top-level. The `simd` module contains *portable vector types* which
//! should work across all platforms and be implemented in the most efficient
//! manner possible for the platform at hand. The `vendor` module contains
//! vendor intrinsics that operate over these SIMD types, typically
//! corresponding to a particular CPU instruction
//!
//! ```rust
//! extern crate stdsimd;
//! use stdsimd::simd::u32x4;
//!
//! fn main() {
//!     let a = u32x4::new(1, 2, 3, 4);
//!     let b = u32x4::splat(10);
//!     assert_eq!(a + b, u32x4::new(11, 12, 13, 14));
//! }
//! ```
//!
//! > **Note**: This crate is *nightly only* at the moment, and requires a
//! > nightly rust toolchain to compile.
//!
//! This documentation is only for one particular architecture, you can find
//! others at:
//!
//! * [i686](https://rust-lang-nursery.github.io/stdsimd/i686/stdsimd/)
//! * [`x86_64`](https://rust-lang-nursery.github.io/stdsimd/x86_64/stdsimd/)
//! * [arm](https://rust-lang-nursery.github.io/stdsimd/arm/stdsimd/)
//! * [aarch64](https://rust-lang-nursery.github.io/stdsimd/aarch64/stdsimd/)
//!
//! ## Portability
//!
//! The `simd` module and its types should be portable to all platforms. The
//! runtime characteristics of these types may vary per platform and per CPU
//! feature enabled, but they should always have the most optimized
//! implementation for the target at hand.
//!
//! The `vendor` module provides no portability guarantees. The `vendor` module
//! is per CPU architecture currently and provides intrinsics corresponding to
//! functions for that particular CPU architecture. Note that the functions
//! provided in this module are intended to correspond to CPU instructions and
//! have no runtime support for whether you CPU actually supports the
//! instruction.
//!
//! CPU target feature detection is done via the `cfg_feature_enabled!` macro
//! at runtime. This macro will detect at runtime whether the specified feature
//! is available or not, returning true or false depending on the current CPU.
//!
//! ```
//! #![feature(cfg_target_feature)]
//!
//! #[macro_use]
//! extern crate stdsimd;
//!
//! fn main() {
//!     if cfg_feature_enabled!("avx2") {
//!         println!("avx2 intrinsics will work");
//!     } else {
//!         println!("avx2 intrinsics will not work");
//!         // undefined behavior: may generate a `SIGILL`.
//!     }
//! }
//! ```
//!
//! After verifying that a specified feature is available, use `target_feature`
//! to enable a given feature and use the desired intrinsic.
//!
//! ```ignore
//! # #![feature(cfg_target_feature)]
//! # #![feature(target_feature)]
//! # #[macro_use]
//! # extern crate stdsimd;
//! # fn main() {
//! #     if cfg_feature_enabled!("avx2") {
//! // avx2 specific code may be used in this function
//! #[target_feature = "+avx2"]
//! fn and_256() {
//!     // avx2 feature specific intrinsics will work here!
//!     use stdsimd::vendor::{__m256i, _mm256_and_si256};
//!
//!     let a = __m256i::splat(5);
//!     let b = __m256i::splat(3);
//!
//!     let got = unsafe { _mm256_and_si256(a, b) };
//!
//!     assert_eq!(got, __m256i::splat(1));
//! }
//! #         and_256();
//! #     }
//! # }
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

#![feature(macro_reexport, const_fn, const_atomic_usize_new)]

/// We re-export run-time feature detection for those architectures that have
/// suport for it in `core`:
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[macro_reexport(cfg_feature_enabled, __unstable_detect_feature)]
extern crate coresimd;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
extern crate coresimd;

/// Platform dependent vendor intrinsics.
pub mod vendor {
    pub use coresimd::vendor::*;

    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    pub use super::runtime::{__unstable_detect_feature, __Feature};
}

/// Platform independent SIMD vector types and operations.
pub mod simd {
    pub use coresimd::simd::*;
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[macro_use]
mod runtime;
