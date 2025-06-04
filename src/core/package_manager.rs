use super::{language::Language, test_framework::TestFramework};
use crate::{error, Error, Result};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};
use which::which;

#[derive(Debug, Serialize, Deserialize)]
pub enum PackageManagers {
    Vcpkg,
    Conan,
}

impl PackageManagers {
    pub fn variants() -> Vec<&'static str> {
        vec!["Vcpkg", "Conan"]
    }
    pub fn from_str(s: &str) -> PackageManagers {
        match s {
            "Vcpkg" => PackageManagers::Vcpkg,
            "Conan" => PackageManagers::Conan,
            _ => PackageManagers::Vcpkg,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageManager {
    variant: PackageManagers,
    directory: PathBuf,
    test_framework: TestFramework,
    language: Language,
}

impl PackageManager {
    pub fn new(
        variant: PackageManagers,
        directory: PathBuf,
        test_framework: TestFramework,
        language: Language,
    ) -> PackageManager {
        PackageManager {
            variant,
            directory,
            test_framework,
            language,
        }
    }

    pub fn init(&self) -> Result<()> {
        match self.variant {
            PackageManagers::Vcpkg => VcpkgManager::init(&self.directory)?,
            PackageManagers::Conan => ConanManager::init(&self.directory)?,
        }
        Ok(())
    }

    pub fn config(&self) -> Result<()> {
        match self.variant {
            PackageManagers::Vcpkg => VcpkgManager::config(&self.directory, &self.test_framework)?,
            PackageManagers::Conan => ConanManager::config(&self.directory, &self.test_framework)?,
        }
        Ok(())
    }
}

struct VcpkgManager;

impl VcpkgManager {
    pub fn init(project_dir: &PathBuf) -> Result<()> {
        match which("vcpkg") {
            Ok(_) => Command::new("vcpkg")
                .args(["new", "--application"])
                .current_dir(project_dir)
                .status()
                .map_err(|e| error!(CustomError, "vcpkg failed: {}", e))?
                .success()
                .then_some(())
                .ok_or_else(|| error!(CustomError, "vcpkg command failed")),
            Err(e) => Err(error!(CustomError, "{}", e)),
        }
    }

    pub fn config(dir: &PathBuf, test_framework: &TestFramework) -> Result<()> {
        match test_framework.vcpkg_setup() {
            Ok(_) => {
                let _status = Command::new("vcpkg")
                    .arg("install")
                    .current_dir(dir)
                    .status()?;
                Ok(())
            }
            Err(e) => Err(error!(CustomError, "{}", e)),
        }
    }
}

struct ConanManager;

impl ConanManager {
    pub fn init(project_dir: &PathBuf) -> Result<()> {
        match which("conan") {
            Ok(_) => Command::new("conan")
                .args(["new", "--application"])
                .current_dir(project_dir)
                .status()
                .map_err(|e| error!(CustomError, "conan failed: {}", e))?
                .success()
                .then_some(())
                .ok_or_else(|| error!(CustomError, "conan command failed")),
            Err(e) => Err(error!(CustomError, "{}", e)),
        }
    }
    pub fn config(dir: &PathBuf, test_framework: &TestFramework) -> Result<()> {
        match test_framework.conan_setup() {
            Ok(_) => {
                let _status = Command::new("vcpkg")
                    .arg("install")
                    .current_dir(dir)
                    .status()?;
                Ok(())
            }
            Err(e) => Err(error!(CustomError, "{}", e)),
        }
    }
}
