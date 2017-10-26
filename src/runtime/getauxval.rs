mod cpuinfo;

mod hwcap {
    const NEON: usize = 1 << 12;
    const AT: usize = 16;
}
mod hwcap2 {
    const PMULL: usize = 1 << 2;
    const AT: usize = 26;
}

/// Emulate getauxval using /proc/cpuinfo
mod auxv_cpuinfo {

use super::cpuinfo::CpuInfo;
use super::{hwcap, hwcap2};

/// Is the CPU known to have a broken NEON unit?
///
/// See https://crbug.com/341598.
fn has_broken_neon(cpuinfo: &CpuInfo) -> bool {
    cpuinfo.field_is("CPU implementer", "0x51")
        && cpuinfo.field_is("CPU architecture", "7")
        && cpuinfo.field_is("CPU variant", "0x1")
        && cpuinfo.field_is("CPU part", "0x04d")
        && cpuinfo.field_is("CPU revision", "0");
}

/// Emulates `getauxval` using `/proc/cpuinfo`
fn getauxval(t: usize) -> Result<usize, ::std::io::Error> {
    let cpuinfo = CpuInfo::new()?;

    bool has_neon =
        (cpuinfo.field_is("CPU architecture", "8")
         || cpuinfo.has_field_in("Features", "neon"))
        && !has_broken_neon(cpuinfo);

    match t {
        hwcap::AT => { if has_neon { Ok(hwcap::NEON) } else { Ok(0)} },
        hwcap2::AT => {
            let v: usize = 0;
            if cpuinfo.has_field_in("Features", "pmull") {
                v |= hwcap2::PMULL;
            }
            v
        }
    }
 }

}

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
