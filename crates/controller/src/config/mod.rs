use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct VmConfig {
    pub vcpus: u8,
    pub memory_mib: u64,
    pub disk_path: PathBuf,
}

impl VmConfig {
    pub fn noble_default() -> Self {
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");

        Self {
            vcpus: 1,
            memory_mib: 1024,
            disk_path: project_root.join("images").join("noble.raw"),
        }
    }
}
