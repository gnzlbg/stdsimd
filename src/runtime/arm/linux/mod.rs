//! Run-time feature detection for ARM on linux
mod cpuinfo;

use super::{__Feature, bit};

trait FeatureQuery {
    fn has_feature(&mut self, x: &__Feature) -> bool;
}

fn detect_features_impl<T: FeatureQuery>(mut x: T) -> usize {
    let value: usize = 0;
    {
        let mut enable_feature = | f | {
            if x.has_feature(&f) {
                bit::set(value, f as u32);
            }
        };
        enable_feature(__Feature::neon);
        enable_feature(__Feature::asimd);
        enable_feature(__Feature::pmull);
    }
    value
}

/// Detects ARM features:
pub fn detect_features() -> usize {
    // FIXME: use libc::getauxval, and if that fails /proc/auxv
    // Try to read /proc/cpuinfo
    if let Ok(v) = cpuinfo::CpuInfo::new() {
        return detect_features_impl(v);
    }
    // Otherwise all features are disabled
    0
}
