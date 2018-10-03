//! NVPTX intrinsics (experimental)
//!
//! These intrinsics form the foundation of the CUDA
//! programming model.
//!
//! The reference is the [CUDA C Programming Guide][cuda_c]. Relevant is also
//! the [LLVM NVPTX Backend documentation][llvm_docs].
//!
//! [cuda_c]:
//! http://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html
//! [llvm_docs]:
//! https://llvm.org/docs/NVPTXUsage.html

mod llvm {
    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = "llvm.cuda.syncthreads"]
        pub fn syncthreads() -> ();
        #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.x"]
        pub fn block_dim_x() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.y"]
        pub fn block_dim_y() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.ntid.z"]
        pub fn block_dim_z() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.x"]
        pub fn block_idx_x() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.y"]
        pub fn block_idx_y() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.ctaid.z"]
        pub fn block_idx_z() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.x"]
        pub fn grid_dim_x() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.y"]
        pub fn grid_dim_y() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.nctaid.z"]
        pub fn grid_dim_z() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.tid.x"]
        pub fn thread_idx_x() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.tid.y"]
        pub fn thread_idx_y() -> i32;
        #[link_name = "llvm.nvvm.read.ptx.sreg.tid.z"]
        pub fn thread_idx_z() -> i32;
    }
}

/// Synchronizes all threads in the block.
#[inline]
pub unsafe fn syncthreads() -> () {
    llvm::syncthreads()
}

/// x-th thread-block dimension.
#[inline]
pub unsafe fn block_dim_x() -> i32 {
    llvm::block_dim_x()
}

/// y-th thread-block dimension.
#[inline]
pub unsafe fn block_dim_y() -> i32 {
    llvm::block_dim_y()
}

/// z-th thread-block dimension.
#[inline]
pub unsafe fn block_dim_z() -> i32 {
    llvm::block_dim_z()
}

/// x-th thread-block index.
#[inline]
pub unsafe fn block_idx_x() -> i32 {
    llvm::block_idx_x()
}

/// y-th thread-block index.
#[inline]
pub unsafe fn block_idx_y() -> i32 {
    llvm::block_idx_y()
}

/// z-th thread-block index.
#[inline]
pub unsafe fn block_idx_z() -> i32 {
    llvm::block_idx_z()
}

/// x-th block-grid dimension.
#[inline]
pub unsafe fn grid_dim_x() -> i32 {
    llvm::grid_dim_x()
}

/// y-th block-grid dimension.
#[inline]
pub unsafe fn grid_dim_y() -> i32 {
    llvm::grid_dim_y()
}

/// z-th block-grid dimension.
#[inline]
pub unsafe fn grid_dim_z() -> i32 {
    llvm::grid_dim_z()
}

/// x-th thread index.
#[inline]
pub unsafe fn thread_idx_x() -> i32 {
    llvm::thread_idx_x()
}

/// y-th thread index.
#[inline]
pub unsafe fn thread_idx_y() -> i32 {
    llvm::thread_idx_y()
}

/// z-th thread index.
#[inline]
pub unsafe fn thread_idx_z() -> i32 {
    llvm::thread_idx_z()
}
