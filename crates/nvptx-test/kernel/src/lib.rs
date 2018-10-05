#![feature(stdsimd, abi_ptx)]
#![no_std]

#[cfg(not(target_os = "cuda"))]
compile_error!("This is a CUDA crate. Building it with host toolchain is not possible.");

extern crate coresimd;

use coresimd::arch::nvptx;

//#[panic_handler]
//pub unsafe fn foo(_: &core::panic::PanicInfo) -> ! { core::hint::unreachable_unchecked() }

/// Add two "vectors" of length `n`. `c <- a + b`
#[no_mangle]
pub unsafe extern "ptx-kernel" fn add(a: *const f32,
                                      b: *const f32,
                                      c: *mut f32,
                                      n: usize) {
    let i = nvptx::block_dim_x()
        .wrapping_mul(nvptx::block_idx_x())
        .wrapping_add(nvptx::thread_idx_x()) as isize;

    if (i as usize) < n {
        *c.offset(i) = *a.offset(i) + *b.offset(i);
    }
}
