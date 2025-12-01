use std::path::PathBuf;

use anyhow::Result;
use controller::{HypervisorConfig, VmConfig, VmManager};

fn main() -> Result<()> {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");

    let h_cfg = HypervisorConfig {
        binary_path: project_root.join("bin/cloud-hypervisor"),
        kernel_path: project_root.join("images").join("CLOUDHV.fd"),
        api_socket: PathBuf::from("/tmp/ch-socket"),
        log_file: PathBuf::from("/tmp/ch.log"),
    };

    let vm_cfg = VmConfig::noble_default();

    let manager = VmManager::new(h_cfg);
    manager.create_and_boot_single(&vm_cfg)?;

    Ok(())
}
