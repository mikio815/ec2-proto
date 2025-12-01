use std::process::{Command, Stdio};
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = "/tmp/ch-socket";
    let _ = std::fs::remove_file(socket_path);

    let ch_path = "/bin/cloud-hypervisor";
    let mut child = Command::new(ch_path)
        .args(&["--api-socket", socket_path])
        .args(&["--console", "tty", "--serial", "tty"])
        .args(&["--log-file", "/tmp/ch.log"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    thread::sleep(Duration::from_secs(1));

    println!("creating VM...");

    let disk_path = "/images/images/noble-server-cloudimg-amd64.img";

    let payload = serde_json::json!({
        "cpus": { "boot_vcpus": 1, "max_vcpus": 1 },
        "memory": { "size": 512 * 1024 * 1024 },
        "console": { "mode": "Tty" },
        "disks": [
            { "path" : disk_path, "readonly": false }
        ],
        "cmdline": { "args": "console=ttyS0 root=/dev/vda1 rw" }
    });

    let status = Command::new("curl")
        .args(&["-X", "PUT", "--unix-socket", socket_path])
        .args(&["--data", &payload.to_string()])
        .arg("http://localhost/api/v1/vm.create")
        .status()?;

    if !status.success() {
        panic!("failed to create VM!");
    }

    println!("boot VM...");

    let status = Command::new("curl")
        .args(&["-X", "PUT", "--unix-socket", socket_path])
        .arg("http://localhost/api/v1/vm.boot")
        .status()?;

    if !status.success() {
        panic!("failed to call boot api!");
    }

    child.wait()?;

    Ok(())
}
