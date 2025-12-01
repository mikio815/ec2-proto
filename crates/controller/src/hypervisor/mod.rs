use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{bail, Context, Result};
use serde_json::json;

use crate::config::VmConfig;

#[derive(Debug, Clone)]
pub struct HypervisorConfig {
    pub binary_path: PathBuf,
    pub api_socket: PathBuf,
    pub log_file: PathBuf,
    pub kernel_path: PathBuf,
}

pub struct HypervisorClient {
    cfg: HypervisorConfig,
}

impl HypervisorClient {
    pub fn new(cfg: HypervisorConfig) -> Self {
        Self { cfg }
    }

    pub fn spawn(&self) -> Result<Child> {
        if self.cfg.api_socket.exists() {
            std::fs::remove_file(&self.cfg.api_socket)
                .context("failed to remove old api socket")?;
        }

        let mut cmd = Command::new(&self.cfg.binary_path);
        cmd.arg("--api-socket")
            .arg(&self.cfg.api_socket)
            .arg("--log-file")
            .arg(&self.cfg.log_file)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        let child = cmd.spawn().context("failed to spawn hypervisor")?;

        self.wait_for_socket(Duration::from_secs(5))?;

        Ok(child)
    }

    pub fn create_vm(&self, vm: &VmConfig) -> Result<()> {
        let payload = json!({
            "cpus": { "boot_vcpus": vm.vcpus, "max_vcpus": vm.vcpus },
            "memory": { "size": vm.memory_mib * 1024 * 1024 },
            "payload": {
                "kernel": self.cfg.kernel_path,
                "cmdline": "console=ttyS0 console=hvc0 root=/dev/vda1 rw" 
            }, 
            "console": { "mode": "Tty" },
            "serial": { "mode": "Tty" }, 
            "disks": [{ "path": vm.disk_path }]
        });

        self.call_api("PUT", "vm.create", &payload.to_string())
    }

    pub fn boot_vm(&self) -> Result<()> {
        self.call_api("PUT", "vm.boot", "")
    }

    fn wait_for_socket(&self, timeout: Duration) -> Result<()> {
        let start = Instant::now();
        while start.elapsed() < timeout {
            if self.cfg.api_socket.exists() {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(50));
        }
        bail!("timeout waiting for api socket")
    }

    fn call_api(&self, method: &str, endpoint: &str, data: &str) -> Result<()> {
        let url = format!("http://localhost/api/v1/{}", endpoint);
        let mut cmd = Command::new("curl");

        eprintln!(
            "[hypervisor] {} {} payload: {}",
            method,
            endpoint,
            if data.is_empty() { "<empty>" } else { data }
        );

        cmd.arg("--unix-socket")
            .arg(&self.cfg.api_socket)
            .arg("--http1.1")
            .arg("-i")
            .arg("-X")
            .arg(method);

        if !data.is_empty() {
            cmd.arg("-H").arg("Accept: application/json")
                .arg("-H").arg("Content-Type: application/json")
                .arg("--data")
                .arg(data);
        }

        cmd.arg(&url);

        let status = cmd
            .status()
            .with_context(|| format!("failed to execute curl for {}", endpoint))?;

        if !status.success() {
            bail!("api call {} failed with status: {:?}", endpoint, status);
        }

        Ok(())
    }

}
