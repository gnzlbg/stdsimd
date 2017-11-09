//! Use libc's getauxval

use super::{__Feature, FeatureQuery};

const AT_HWCAP: usize = 16;
const AT_HWCAP2: usize = 26;

#[cfg(target_arch = "arm")]
mod arm {
    // See:
    // http://elixir.free-electrons.com/linux/latest/source/arch/arm/include/uapi/asm/hwcap.h
    const HWCAP_NEON: usize = 12;
    const HWCAP2_PMULL: usize = 1;
}

#[cfg(target_arch = "aarch64")]
mod aarch64 {
    // See:
    // http://elixir.free-electrons.com/linux/latest/source/arch/arm64/include/uapi/asm/hwcap.h
    const HWCAP_ASIMD: usize = 1;
    const HWCAP_PMULL: usize = 4;
}

extern "C" {
    #[linkage = "extern_weak"]
    fn getauxval(usize) -> usize;
}

pub struct Auxv {
    hwcap: usize,
    hwcap2: usize,
}

impl Auxv {
    pub fn new() -> Option<Auxv> {
        if getauxval as fn(usize) -> usize != 0 {
            return Some(Auxv {
                hwcap: getauxval(AT_HWCAP),
                hwcap2: getauxval(AT_HWCAP2)
            });
        }
        None
    }
}

impl FeatureQuery for Auxv {
    #[cfg(target_arch = "arm")]
    fn has_feature(&mut self, x: &__Feature) -> bool {
        use __Feature::*;
        match *x {
            neon => bit::test(self.hwcap, arm::HWCAP_NEON),
            asimd => false,
            pmull => bit::test(self.hwcap2, arm::HWCAP2_PMULL),
        }
    }
    #[cfg(target_arch = "aarch64")]
    fn has_feature(&mut self, x: &__Feature) -> bool {
        use __Feature::*;
        match *x {
            neon => bit::test(self.hwcap, aarch64::HWCAP_ASIMD),
            asimd => bit::test(self.hwcap, aarch64::HWCAP_ASIMD),
            pmull => bit::test(self.hwcap, aarch64::HWCAP_PMULL),
        }
    }
}
