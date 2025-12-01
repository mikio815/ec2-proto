pub struct VmConfig {
    pub cpu: usize,
    pub memory_mib: usize,
    pub disk_path: String,
}

pub struct VmManager;

impl VmManager {
    pub fn create(config: &VmConfig) -> anyhow::Result<()> {
        unimplemented!("後で")
    }

    pub fn boot() -> anyhow::Result<()> {
        unimplemented!("後で")
    }
}
