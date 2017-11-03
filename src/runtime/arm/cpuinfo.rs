//! Reads /proc/cpuinfo on Linux systems

/// cpuinfo
pub struct CpuInfo {
    raw: String
}

/// Field of cpuinfo
pub struct CpuInfoField(Option<&str>);

impl PartialEq<&str> for CpuInfoField {
    fn eq(&self, other: &str) -> bool {
        match self.0 {
            None => other.len() == 0,
            Some(f) => f == other.trim(),
        }
    }
}
impl Eq<&str> for CpuInfoFied {}

impl CpuInfoField {
    fn new(v: Option<&str>) -> Self {
        match v {
            None => Self(v),
            Some(f) => Self(f.trim()),
        }
    }
    /// Does the field exist?
    fn exists(&self) -> bool {
        self.0.is_some()
    }
    /// Does the field contain `other`?
    fn has(&self, other: &str) -> bool {
        match self.0 {
            None => other.len() == 0,
            Some(f) => {
                let other = other.trim(); 
                for v in f.split(" ") {
                    if f == other {
                        return true;
                    }
                }
                false
            }
        }
        false
    }
}

impl CpuInfo {
    /// Reads /proc/cpuinfo into CpuInfo.
    pub fn new() -> Result<CpuInfo, ::std::io::Error> {
        use ::std::io::Read;
        let mut file = ::std::fs::File::open("/proc/cpuinfo")?;
        let mut cpui = CpuInfo{ raw: String::new() };
        file.read_to_string(&mut cpui.raw)?;
        Ok(cpui)
    }
    /// Returns the value of the cpuinfo `field`.
    pub fn field(&self, field: &str) -> CpuInfoField {
        for l in self.raw.lines() {
            if l.starts_with(field) {
                return CpuInfoField(l.split(": ").skip(1).next());
            }
        }
        CpuInfoField(None)
    }

    /// Returns the `raw` contents of `/proc/cpuinfo`
    fn raw(&self) -> &String {
        self.raw
    }

    fn from_str(other: &str) -> Result<CpuInfo, std::io::Error> {
        Ok(CpuInfo{raw: String::from(other) })
    }
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod tests {
    use super::*;

    const CORE_DUO_T6500: &str = r"processor       : 0
vendor_id       : GenuineIntel
cpu family      : 6
model           : 23
model name      : Intel(R) Core(TM)2 Duo CPU     T6500  @ 2.10GHz
stepping        : 10
microcode       : 0xa0b
cpu MHz         : 1600.000
cache size      : 2048 KB
physical id     : 0
siblings        : 2
core id         : 0
cpu cores       : 2
apicid          : 0
initial apicid  : 0
fdiv_bug        : no
hlt_bug         : no
f00f_bug        : no
coma_bug        : no
fpu             : yes
fpu_exception   : yes
cpuid level     : 13
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe nx lm constant_tsc arch_perfmon pebs bts aperfmperf pni dtes64 monitor ds_cpl est tm2 ssse3 cx16 xtpr pdcm sse4_1 xsave lahf_lm dtherm
bogomips        : 4190.43
clflush size    : 64
cache_alignment : 64
address sizes   : 36 bits physical, 48 bits virtual
power management:

processor       : 1
vendor_id       : GenuineIntel
cpu family      : 6
model           : 23
model name      : Intel(R) Core(TM)2 Duo CPU     T6500  @ 2.10GHz
stepping        : 10
microcode       : 0xa0b
cpu MHz         : 1200.000
cache size      : 2048 KB
physical id     : 0
siblings        : 2
core id         : 1
cpu cores       : 2
apicid          : 1
initial apicid  : 1
fdiv_bug        : no
hlt_bug         : no
f00f_bug        : no
coma_bug        : no
fpu             : yes
fpu_exception   : yes
cpuid level     : 13
wp              : yes
flags           : fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe nx lm constant_tsc arch_perfmon pebs bts aperfmperf pni dtes64 monitor ds_cpl est tm2 ssse3 cx16 xtpr pdcm sse4_1 xsave lahf_lm dtherm
bogomips        : 4190.43
clflush size    : 64
cache_alignment : 64
address sizes   : 36 bits physical, 48 bits virtual
power management:
";

    const XEON_5460: &str = r"processor	: 0
vendor_id	: GenuineIntel
cpu family	: 6
model		: 23
model name	: Intel(R) Xeon(R) CPU           X5460  @ 3.16GHz
stepping	: 6
microcode	: 0x60f
cpu MHz		: 3158.785
cache size	: 6144 KB
physical id	: 0
siblings	: 4
core id		: 0
cpu cores	: 4
apicid		: 0
initial apicid	: 0
fpu		: yes
fpu_exception	: yes
cpuid level	: 10
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx lm constant_tsc arch_perfmon pebs bts rep_good nopl aperfmperf pni dtes64 monitor ds_cpl vmx est tm2 ssse3 cx16 xtpr pdcm dca sse4_1 lahf_lm dtherm tpr_shadow vnmi flexpriority
bogomips	: 6317.57
clflush size	: 64
cache_alignment	: 64
address sizes	: 38 bits physical, 48 bits virtual
power management:

processor	: 1
vendor_id	: GenuineIntel
cpu family	: 6
model		: 23
model name	: Intel(R) Xeon(R) CPU           X5460  @ 3.16GHz
stepping	: 6
microcode	: 0x60f
cpu MHz		: 3158.785
cache size	: 6144 KB
physical id	: 1
siblings	: 4
core id		: 0
cpu cores	: 4
apicid		: 4
initial apicid	: 4
fpu		: yes
fpu_exception	: yes
cpuid level	: 10
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx lm constant_tsc arch_perfmon pebs bts rep_good nopl aperfmperf pni dtes64 monitor ds_cpl vmx est tm2 ssse3 cx16 xtpr pdcm dca sse4_1 lahf_lm dtherm tpr_shadow vnmi flexpriority
bogomips	: 6317.56
clflush size	: 64
cache_alignment	: 64
address sizes	: 38 bits physical, 48 bits virtual
power management:
";

    #[test]
    fn test_cpuinfo_linux() {
        let cpuinfo = CpuInfo::new().unwrap();
        match cpuinfo.field("vendor_id") {
            "GenuineIntel" => {
                assert!(cpuinfo.field("flags").exists());
                assert!(!cpuinfo.field("vendor33_id").exists());
                assert!(cpuinfo.field("flags").has("sse"));
                assert!(!cpuinfo.field("flags").has("avx314"));
            }
            &_ => {}
        }
        println!("{}", cpuinfo.raw());
    }

    #[test]
    fn test_cpuinfo_linux_core_duo() {
        let cpuinfo = CpuInfo::from_str(CORE_DUO_T6500);
        assert!(cpuinfo.field("vendor_id") == "GenuineIntel");
        assert!(cpuinfo.field("family") == "6");
        assert!(cpuinfo.field("model") == "23");
        assert!(cpuinfo.field("model name") ==
                "Intel(R) Core(TM)2 Duo CPU     T6500  @ 2.10GHz");
        assert!(cpuinfo.field("flags") ==
                "fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe nx lm constant_tsc arch_perfmon pebs bts aperfmperf pni dtes64 monitor ds_cpl est tm2 ssse3 cx16 xtpr pdcm sse4_1 xsave lahf_lm dtherm");
        assert!(cpuinfo.field("flags").has("fpu"));
        assert!(cpuinfo.field("flags").has("dtherm"));
        assert!(cpuinfo.field("flags").has("sse2"));
        assert!(!cpuinfo.field("flags").has("avx"));
    }
}
