//! ELF Auxiliary Vector
//!
//! The auxiliary vector is a memory region in a running ELF program's stack
//! composed of (key: usize, value: usize) pairs.
//!
//! The keys used in the aux vector are platform dependent. For Linux, they are
//! defined in [linux/auxvec.h][auxvec_h]. The hardware capabilities of a given
//! CPU can be queried with the  `AT_HWCAP` and `AT_HWCAP2` keys.
//!
//! There is no perfect way of reading the auxiliary vector.
//!
//! - `coresimd`: if `getauxval` is available, `coresimd` will try to use it.
//! - `stdsimd`: if `getauxval` is not available, it will try to read
//! `/proc/self/auxv`, and if that fails it will try to read `/proc/cpuinfo`.
//!
//! For more information about when `getauxval` is available check the great
//! [`auxv` crate documentation][auxv_docs].
//!
//! [auxvec_h]: https://github.com/torvalds/linux/blob/master/include/uapi/linux/auxvec.h
//! [auxv_docs]: https://docs.rs/auxv/0.3.3/auxv/

/// Key to access the CPU Hardware capabilities bitfield.
pub const AT_HWCAP: usize = 16;
/// Key to access the CPU Hardware capabilities 2 bitfield.
pub const AT_HWCAP2: usize = 26;

/// Cache HWCAP bitfields of the ELF Auxiliary Vector.
///
/// If an entry cannot be read all the bits in the bitfield
/// are set to zero.
#[cfg(any(target_arch = "arm", target_arch = "powerpc64"))]
#[derive(Debug, Copy, Clone)]
pub struct AuxVec {
    pub hwcap: usize,
    pub hwcap2: usize,
}

/// Cache HWCAP bitfields of the ELF Auxiliary Vector.
///
/// If an entry cannot be read all the bits in the bitfield
/// are set to zero.
#[cfg(target_arch = "aarch64")]
#[derive(Debug, Copy, Clone)]
pub struct AuxVec {
    pub hwcap: usize,
}

pub mod libc {
    use super::*;

    mod ffi {
        extern "C" {
            #[linkage = "extern_weak"]
            pub fn getauxval(x: usize) -> usize;
        }
    }


    /// Returns the value of the ELF Auxiliary Vector associated with `key`.
    ///
    /// This can fail if the auxiliary vector does not contain the key,
    /// the final binary is not linked against a libc library containing
    /// `getauxval`, etc.
    ///
    /// This only returns the value if reading the key from the auxiliary
    /// vector properly succeeds.
    fn getauxval(key: usize) -> Result<usize, ()> {
        unsafe {
            let ffi_getauxval : Option<extern fn(usize) -> usize>
                = ::core::mem::transmute(&ffi::getauxval);
            if let Some(f) = ffi_getauxval {
                return Ok(f(key));
            }
            Err(())
        }
    }

    /// Computes the entries of the Auxiliary Vector cache by
    /// calling libc's `getauxval(3)`.
    pub fn auxv() -> Result<AuxVec, ()> {
        if let Ok(hwcap) = getauxval(AT_HWCAP) {
            #[cfg(target_arch = "aarch64")]
            {
                return Ok(AuxVec { hwcap });
            }
            #[cfg(any(target_arch = "arm", target_arch = "powerpc64"))]
            {
                if let Ok(hwcap2) = getauxval(AT_HWCAP2) {
                    return Ok(AuxVec { hwcap, hwcap2 });
                }
            }
        }
        Err(())
    }

    #[cfg(test)]
    mod tests {
        extern crate auxv as auxv_crate;
        use super::*;

        // Reads the Auxiliary Vector key from getauxval()
        // using the auxv crate.
        fn auxv_crate_get(key: usize) -> Option<usize> {
            use self::auxv_crate::AuxvType;
            use self::auxv_crate::getauxval::Getauxval;
            let q = auxv_crate::getauxval::NativeGetauxval {};
            match q.getauxval(key as AuxvType) {
                Ok(v) => Some(v as usize),
                Err(_) => None,
            }
        }

        #[test]
        fn auxv_dump() {
            if let Ok(auxvec) = auxv() {
                println!("{:?}", auxvec);
            } else {
                println!("reading /proc/self/auxv failed!");
            }
        }

        #[cfg(any(target_arch = "arm", target_arch = "powerpc64"))]
        #[test]
        fn auxv_crate() {
            let v = auxv();
            if let Some(hwcap) = auxv_crate_get(AT_HWCAP) {
                assert_eq!(v.unwrap().hwcap, hwcap);
            }
            if let Some(hwcap2) = auxv_crate_get(AT_HWCAP2) {
                assert_eq!(v.unwrap().hwcap2, hwcap2);
            }
        }

        #[cfg(target_arch = "aarch64")]
        #[test]
        fn auxv_crate() {
            let v = auxv();
            if let Some(hwcap) = auxv_crate_get(AT_HWCAP) {
                assert_eq!(v.unwrap().hwcap, hwcap);
            }
        }
    }
}
