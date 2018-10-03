`std::arch` - Rust's std library implementation of architecture-specific APIs
=======

[![Travis-CI Status]][travis] [![Appveyor Status]][appveyor] [![Latest Version]][crates.io] [![docs]][docs.rs]

# Usage

The `arch` module is shipped with `libcore` and `libstd`. To use it, just import
it: `use std::arch` or `use core::arch`.

It is not recommended to use `stdsimd`'s master branch, but if you need to do
so, you can add `stdsimd` to your `Cargo.toml` as follows:

```toml
#[dependencies]
stdsimd = { git = "https://github.com/rust-lang-nursery/stdsimd.git" }
```

# Architecture support

| Architecture: | Rust version  | Docs              | Builds | Runs | Verified |
|---------------|---------------|-------------------|--------|------|----------|
| `x86`         | stable 1.27.0 | [docs][i686]      | ✓      | ✓    | ✓        |
| `x86_64`      | stable 1.27.0 | [docs][x86_64]    | ✓      | ✓    | ✓        |
| `arm`         | nightly       | [docs][arm]       | ✓      | ✓    | ✗        |
| `aarch64`     | nightly       | [docs][aarch64]   | ✓      | ✓    | ✗        |
| `powerpc`     | nightly       | [docs][powerpc]   | ✓      | ✓    | ✗        |
| `powerpc64`   | nightly       | [docs][powerpc64] | ✓      | ✓    | ✗        |
| `powerpc64le` | nightly       | [docs][powerpc64] | ✓      | ✓    | ✗        |
| `mips`        | nightly       | [docs][mips]      | ✓      | ✗    | ✗        |
| `mipsel`      | nightly       | [docs][mips]      | ✓      | ✗    | ✗        |
| `mips64`      | nightly       | [docs][mips64]    | ✓      | ✗    | ✗        |
| `mips64el`    | nightly       | [docs][mips64]    | ✓      | ✗    | ✗        |
| `thumbv6m`    | nightly       | -                 | ✓      | ✗    | ✗        |
| `thumbv7m`    | nightly       | -                 | ✓      | ✗    | ✗        |
| `thumbv7em`   | nightly       | -                 | ✓      | ✗    | ✗        |
| `wasm`        | nightly       | [docs][wasm]      | ✓      | ✓    | ✗        |
| `nvptx`       | nightly       | [docs][nvptx]     | ✓      | ✓    | ✗        |


[i686]: https://rust-lang-nursery.github.io/stdsimd/i686/stdsimd/
[x86_64]: https://rust-lang-nursery.github.io/stdsimd/x86_64/stdsimd/
[arm]: https://rust-lang-nursery.github.io/stdsimd/arm/stdsimd/
[aarch64]: https://rust-lang-nursery.github.io/stdsimd/aarch64/stdsimd/
[powerpc]: https://rust-lang-nursery.github.io/stdsimd/powerpc/stdsimd/
[powerpc64]: https://rust-lang-nursery.github.io/stdsimd/powerpc64/stdsimd/
[mips]: https://rust-lang-nursery.github.io/stdsimd/mips/stdsimd/
[mips64]: https://rust-lang-nursery.github.io/stdsimd/mips64/stdsimd/
[wasm]: https://rust-lang-nursery.github.io/stdsimd/wasm/stdsimd/
[nvptx]: https://rust-lang-nursery.github.io/stdsimd/nvptx/stdsimd/

# Contribute

* [How to get started][contrib]
* [How to help implement intrinsics][help-implement]

[contrib]: https://github.com/rust-lang-nursery/stdsimd/blob/master/CONTRIBUTING.md
[help-implement]: https://github.com/rust-lang-nursery/stdsimd/issues/40

# License

`stdsimd` is primarily distributed under the terms of both the MIT license and
the Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.


[travis]: https://travis-ci.org/rust-lang-nursery/stdsimd
[Travis-CI Status]: https://travis-ci.org/rust-lang-nursery/stdsimd.svg?branch=master
[appveyor]: https://ci.appveyor.com/project/rust-lang-libs/stdsimd/branch/master
[Appveyor Status]: https://ci.appveyor.com/api/projects/status/ix74qhmilpibn00x/branch/master?svg=true
[Latest Version]: https://img.shields.io/crates/v/stdsimd.svg
[crates.io]: https://crates.io/crates/stdsimd
[docs]: https://docs.rs/stdsimd/badge.svg
[docs.rs]: https://docs.rs/stdsimd/
