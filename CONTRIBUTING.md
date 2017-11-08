# Contributing to stdsimd

The `stdsimd` crate is more than willing to accept contributions! First you'll
probably want to check out the repository and make sure that tests pass for you:

```
$ git clone https://github.com/rust-lang-nursery/stdsimd
$ cd stdsimd
$ cargo +nightly test
```

To run codegen tests, run in release mode:

```
$ cargo +nightly test --release
```

Remember that this repository requires the nightly channel of Rust! If any of
the above steps don't work, [please let us know][new]!

Next up you can [find an issue][issues] to help out on, we've selected a few
with the [`help wanted`][help] and [`impl-period`][impl] tags which could
particularly use some help. You may be most interested in [#40][vendor],
implementing all vendor intrinsics on x86. That issue's got some good pointers
about where to get started!

If you've got general questions feel free to [join us on gitter][gitter] and ask
around! Feel free to ping either @BurntSushi or @alexcrichton with questions.

[gitter]: https://gitter.im/rust-impl-period/WG-libs-simd

# How to add an intrinsics

Suppose we want to add the `_mm256_adds_epu8` intrinsic to the `stdsimd` crate.

The first step is figuring out in which module does this intrinsic belong within
the `stdsimd` library. The module structure of the `stdsimd` crate is
`src/{architecture}/{target_feature}.rs`, where:

- the `x86` architecture module contains intrinsics for `x86` and `x86_64`
- the `arm` module contains intrinsics for 32-bit arm architectures
- the `aarch64` module contains intrinsics for 64-bit arm architectures

Rust's `x86` `target_feature`s correspond to the `CPUID` flag of a particular
intrinsic. The [Intel Intrinsics Guide][_mm256_adds_epu8] tells us that the
`CPUID` flag for the intrinsic is `AVX2`. That is, the correct module for adding
a new `avx2` intrinsic is `src/x86/avx2.rs`.

The next step is to add the intrinsic to the `avx2` module. The [Intel
Intrinsics Guide][_mm256_adds_epu8] gives us `_mm256_adds_epu8`'s:

- assembly instruction `vpaddusb`
- `C` signature: `__m256i _mm256_adds_epu8 (__m256i a, __m256i b);`
- description: "Add packed unsigned 8-bit integers in a and b using
  saturation, and store the results in dst."

While C vectors are _untyped, Rust SIMD vector types are typed. Here the
description tells us that the intrinsic operates on 256-bit wide vectors of
8-bit unsigned integers which allows us to choose the appropriate Rust SIMD
vector type: `u8x32`. Here `u8` stands for "8-bit unsigned integer", and `x32`
stands for 32 vector lanes. Since `8 bits * 32 = 256 bits`, the size of this
vector equals the size of the C SIMD vector type `_m256i`.


That is, the appropriate Rust declaration for this intrinsic is:

```rust
/// Add packed unsigned 8-bit integers in `a` and `b` using
/// saturation.
#[inline(always)]
#[target_feature = "+avx2"]
pub unsafe fn _mm256_adds_epu8(a: u8x32, b: u8x32) -> u8x32;
```

where I have preserved the arugment names of the Intel guide and slightly
adapted the guide's description.

Next we need to implement this intrinsic by calling the appropriate LLVM
function, and for that we need to figure out its name and signature. A good
place to start is to search for the intrinsic or the assembly instruction in the
[LLVM's Language Reference Manual][llvm_ref], however, this won't help us much
in this case. The next best place to look for is Clang's (LLVM C++ front-end)
implementation of the intrinsic; typically googling for "clang github
`_mm256_adds_epu8`" is enough. In this case, it leads us to Clang's
[`avx2intrin.h`](https://github.com/llvm-mirror/clang/blob/master/lib/Headers/avx2intrin.h)
header file, where the intrinsic is implemented as:

```c++
static __inline__ __m256i __DEFAULT_FN_ATTRS
_mm256_adds_epu8(__m256i __a, __m256i __b)
{
  return (__m256i)__builtin_ia32_paddusb256((__v32qi)__a, (__v32qi)__b);
}
```

Ok, it seems that we are not there yet. Those builtin's are defined in LLVM, so
we go to [LLVM's github repo](https://github.com/llvm-mirror/llvm) and search
for "`__builtin_ia32_paddusb256`", which gives us a single
[hit](https://github.com/llvm-mirror/llvm/blob/0aacd96bb744ae619e9d4e8763802d2db59e677a/include/llvm/IR/IntrinsicsX86.td#L1623):


```
  def int_x86_avx2_paddus_b : GCCBuiltin<"__builtin_ia32_paddusb256">,
              Intrinsic<[llvm_v32i8_ty], [llvm_v32i8_ty,
                         llvm_v32i8_ty], [IntrNoMem, Commutative]>;
```

It's our lucky day:

-  `int_x86_avx2_paddus_b`: this is the intrinsic we need to call. We must map
it to the real LLVM name, which is `llvm.x86.avx2.paddus.b`, by replacing `int`
with `llvm` and the `_` with `.`.

- `Intrinsic<[llvm_v32i8_ty], [llvm_v32i8_ty, llvm_v32i8_ty], [IntrNoMem,
  Commutative]>`: this gives us the return type (`[llvm_v32i8_ty]`), the
  arguments (`[llvm_v32i8_ty, llvm_v32i8_ty]`), and some meta-data that we can
  ignore. We need to map these to real LLVM type names by removing `llvm_` and
  `_ty` resulting in `v32i8` for our 8-bit integer vector type with 32 lanes.

Now that we have the intrinsic name we can import it into Rust and call it as a normal
function:

```rust
#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.x86.avx2.paddus.b"]
    fn paddusb(a: u8x32, b: u8x32) -> u8x32;
}

/// Add packed unsigned 8-bit integers in `a` and `b` using
/// saturation.
#[inline(always)]
#[target_feature = "+avx2"]
pub unsafe fn _mm256_adds_epu8(a: u8x32, b: u8x32) -> u8x32 {
    paddusb(a, b)
}
```

That's it! You have implemented your very first intrinsic! The final step is to
make sure that the intrinsic works correctly. The first thing we need to do is
check that it emits the proper assembly instruction. For this we use the
`assert_intr` procedural macro:

```rust
/// Add packed unsigned 8-bit integers in `a` and `b` using
/// saturation.
#[target_feature = "+avx2"]
#[cfg_attr(test, assert_instr(vpaddusb))]
pub unsafe fn _mm256_adds_epu8(a: u8x32, b: u8x32) -> u8x32 {
    paddusb(a, b)
}
```

This macro will create a test that calls the intrinsic but doesn't execute that.
Instead, the test disassembles itself and looks in the disassembly for the
`vpaddusb` assembly instruction. Chances are that if the assembly instruction
that we wanted to generate is being generated correctly, our intrinsic is going
to work. 

To make sure that this is the case we add a `simd_test`:

```rust
#[cfg(test)]
mod tests {
    #[simd_test = "avx2"]
    unsafe fn _mm256_adds_epu8() {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let a = u8x32::new(
            0, 1, 2, 3, 4, 5, 6, 7,
            8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        );
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let b = u8x32::new(
            32, 33, 34, 35, 36, 37, 38, 39,
            40, 41, 42, 43, 44, 45, 46, 47,
            48, 49, 50, 51, 52, 53, 54, 55,
            56, 57, 58, 59, 60, 61, 62, 63,
        );
        let r = avx2::_mm256_adds_epu8(a, b);
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let e = u8x32::new(
            32, 34, 36, 38, 40, 42, 44, 46,
            48, 50, 52, 54, 56, 58, 60, 62,
            64, 66, 68, 70, 72, 74, 76, 78,
            80, 82, 84, 86, 88, 90, 92, 94,
        );
        assert_eq!(r, e);
    }
}
```

The main difference between `#[test]` and `#[simd_test = "feature"]` is that we
cannot run AVX2 tests on CPUs that do not support AVX2, otherwise they would
just crash. The `simd_test` procedural macro wraps the test with run-time
feature detection to make sure that they only run on CPUs that support them.

And that's about it! Once your code compiles and test pass locally you are a
`cargo fmt --all` and `cargo clippy --all -- -D clippy-pedantic` away from
sending your first pull-request!

# How to write examples for stdsimd intrinsics

There are a few features that must be enabled for the given intrinsic to work
properly and the example must only be run by `cargo test --doc` when the feature
is supported by the CPU. As a result, the default `fn main` that is generated by
`rustdoc` will not work (in most cases). Consider using the following as a guide
to ensure your example works as expected.

```rust
/// # // We need cfg_target_feature to ensure the example is only
/// # // run by `cargo test --doc` when the CPU supports the feature
/// # #![feature(cfg_target_feature)]
/// # // We need target_feature for the intrinsic to work
/// # #![feature(target_feature)]
/// #
/// # // rustdoc by default uses `extern crate stdsimd`, but we need the
/// # // `#[macro_use]`
/// # #[macro_use] extern crate stdsimd;
/// #
/// # // The real main function
/// # fn main() {
/// #     // Only run this if `<target feature>` is supported
/// #     if cfg_feature_enabled!("<target feature>") {
/// #         // Create a `worker` function that will only be run if the target feature
/// #         // is supported and ensure that `target_feature` is enabled for your worker
/// #         // function
/// #         #[target_feature = "+<target feature>"]
/// #         fn worker() {
///
/// // Write your example here. Feature specific intrinsics will work here! Go wild!
///
/// #         }
/// #         worker();
/// #     }
/// # }
```

If some of the above syntax does not look familiar, the [Documentation as tests] section
of the [Rust Book] describes the `rustdoc` syntax quite well. As always, feel free
to [join us on gitter][gitter] and ask us if you hit any snags, and thank you for helping
to improve the documentation of `stdsimd`!

[new]: https://github.com/rust-lang-nursery/stdsimd/issues/new
[issues]: https://github.com/rust-lang-nursery/stdsimd/issues
[help]: https://github.com/rust-lang-nursery/stdsimd/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22
[impl]: https://github.com/rust-lang-nursery/stdsimd/issues?q=is%3Aissue+is%3Aopen+label%3Aimpl-period
[vendor]: https://github.com/rust-lang-nursery/stdsimd/issues/40
[Documentation as tests]: https://doc.rust-lang.org/book/first-edition/documentation.html#documentation-as-tests
[Rust Book]: https://doc.rust-lang.org/book/first-edition
[_mm256_adds_epu8]: https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm256_adds_epu8&expand=5658,5601,197
[llvm_ref]: https://llvm.org/docs/LangRef.html
