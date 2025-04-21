use crate::{error, execute_cmd, Error, Result};
use strum_macros::EnumString;
use which::which;

#[derive(Debug, EnumString, strum_macros::VariantNames)]
pub enum PackageManagerEnum {
    Vcpkg,
    Conan,
}

pub trait PackageManager {
    fn init(&self) -> Result<()>;
}

pub struct VcpkgManager {
    project_name: String,
}

impl VcpkgManager {
    pub fn new(project_name: &str) -> Result<Self> {
        match which("vcpkg") {
            Ok(_) => Ok(VcpkgManager {
                project_name: project_name.to_string(),
            }),
            Err(e) => Err(error!(CustomError, "{}", e)),
        }
    }
}

impl PackageManager for VcpkgManager {
    fn init(&self) -> Result<()> {
        let cmd = format!(
            "cd {} && vcpkg new --application && vcpkg add port gtest && vcpkg install",
            self.project_name
        );
        Ok(execute_cmd(&cmd)?)
    }
}

pub struct ConanManager {
    project_name: String,
}

impl ConanManager {
    pub fn new(project_name: &str) -> Result<Self> {
        match which("conan") {
            Ok(_) => Ok(ConanManager {
                project_name: project_name.to_string(),
            }),
            Err(e) => Err(error!(CustomError, "{}", e)),
        }
    }
}

impl PackageManager for ConanManager {
    fn init(&self) -> Result<()> {
        let cmd = format!(
            "cd {} && vcpkg new --application && vcpkg add port gtest && vcpkg install",
            self.project_name
        );
        Ok(execute_cmd(&cmd)?)
    }
}
