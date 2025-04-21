use crate::{error, Error, Result};
use std::process::{Command, Stdio};

pub fn execute_cmd(cmd: &str) -> Result<()> {
    let mut child = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?
    };

    let status = child.wait()?;

    if !status.success() {
        Err(error!(CustomError, "Command failed: {}", cmd))
    } else {
        Ok(())
    }
}
