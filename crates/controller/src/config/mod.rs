use std::path::PathBuf;

pub struct VmConfig {
    pub vcpus: u8,
    pub memory_mib: u64,
    pub disk_path: PathBuf,
}

impl VmConfig {
    pub fn noble_default() -> Self {
        Self {
            vcpus: 1,
            memory_mib: 512,
            disk_path: PathBuf::from("/images/images/noble-server-cloudimg-amd64.img"),
        }
    }
}