//! getauxval

use super::cpuinfo;

pub struct Auxv {
    hwcap_16: usize,
    hwcap_26: usize,
}


/*
// If linked against a libc that provides getauxval, use that:
if libc::getauxval as fn(usize) -> usize != 0 {
return detect_features_impl(libc::Auxval::new());
    }
    // Otherwise try to read /proc/auxv
    if let Ok(v) = proc::Auxval::new() {
        return detect_features_impl(v);
    }
    */


pub mod hwcap {
    pub const NEON: usize = 1 << 12;
    pub const AT: usize = 16;
}
pub mod hwcap2 {
    pub const PMULL: usize = 1 << 2;
    pub const AT: usize = 26;
}

/// Emulate getauxval using /proc/cpuinfo
mod auxv_cpuinfo {

    use super::cpuinfo::CpuInfo;
    use super::{hwcap, hwcap2};


    /// Emulates `getauxval` using `/proc/cpuinfo`
    pub fn getauxval(t: usize) -> Result<usize, ::std::io::Error> {
        let cpuinfo = CpuInfo::new()?;

        let has_neon: bool = (cpuinfo.field("CPU architecture") == "8"
            || cpuinfo.field("Features").has("neon"))
            && !has_broken_neon(&cpuinfo);

        match t {
            hwcap::AT => if has_neon {
                Ok(hwcap::NEON)
            } else {
                Ok(0)
            },
            hwcap2::AT => {
                let mut v: usize = 0;
                if cpuinfo.field("Features").has("pmull") {
                    v |= hwcap2::PMULL;
                }
                Ok(v)
            }
            _ => unreachable!(),
        }
    }

}
/*
/// Read auxval from /proc/self/auxval
mod proc {
    #[repr(C)]
    struct Entry {
        tag: usize, value: usize
    }

    fn getauxval(t: usize) -> Result<usize, ::std::io::Error> {
        let mut file = File::open("/proc/self/auxv")?;
        let mut entry = Entry{tag: 0, value: 0};
        let entry_s: &mut [u8] = ::std::slice::from_raw_parts(
            (&mut entry as mut*const T) as mut*const u8,
            ::std::mem::size_of::<Entry>(),
        );
        loop {
            file.read_exact(&mut entry_s)?;
            if entry.tag == 0 && entry.value == 0 {
                return Err();
            }
            if entry.tag == t {
                return Ok(entry.value);
            }
        }
    }
}

/// Obtain auxval from libc's getauxval
mod libc {
    extern "C" {
        #[linkage = "extern_weak"]
        fn getauxval(usize) -> usize;
    }
}

fn getauxval(t: usize) -> usize {
    if libc::getauxval as fn(usize) -> usize != 0 {
        return libc::getauxval(t);
    }
    if let Ok(v) = proc::getauxval(t) {
        return t;
    }
    if let Ok(v) = auxv_cpuinfo::getauxval(t) {
        return t;
    }
    0
}
 */

pub fn getauxval(t: usize) -> usize {
    if let Ok(v) = auxv_cpuinfo::getauxval(t) {
        return v;
    }
    0
}
