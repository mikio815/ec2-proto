use anyhow::Result;

use crate::config::VmConfig;
use crate::hypervisor::{HypervisorClient, HypervisorConfig};

pub struct VmManager {
    hypervisor: HypervisorClient,
}

impl VmManager {
    pub fn new(h_cfg: HypervisorConfig) -> Self {
        let hypervisor = HypervisorClient::new(h_cfg);
        Self { hypervisor }
    }

    /// テストなので
    pub fn create_and_boot_single(&self, vm_cfg: &VmConfig) -> Result<()> {
        let mut child = self.hypervisor.spawn()?;

        self.hypervisor.create_vm(vm_cfg)?;
        self.hypervisor.boot_vm()?;

        child.wait()?;

        Ok(())
    }
}
