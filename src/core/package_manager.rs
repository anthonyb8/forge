use super::{language::Language, test_framework::TestFramework};
use crate::{error, Error, Result};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};
use which::which;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        language::{CStandard, CppStandard},
        test_framework::TestFrameworks,
    };
    use serde_json::json;
    use serial_test::serial;
    use std::{
        env,
        fs::{self},
    };
    use std::{thread, time::Duration};

    // Utility functions
    fn create_dummy_project(path: &PathBuf) -> anyhow::Result<()> {
        fs::create_dir_all(path)?;
        thread::sleep(Duration::from_millis(10));
        Ok(())
    }

    fn check_file_exits(path: &PathBuf) -> bool {
        return match fs::exists(path) {
            Ok(s) => s,
            Err(_) => false,
        };
    }

    fn delete_dummy_project(path: &PathBuf) -> anyhow::Result<()> {
        fs::remove_dir_all(path)?;
        Ok(())
    }

    #[test]
    fn test_pkg_manager_from_str() {
        let s = "Vcpkg";
        let v = PackageManagers::from_str(s);
        assert_eq!(v, PackageManagers::Vcpkg);

        let s = "Conan";
        let v = PackageManagers::from_str(s);
        assert_eq!(v, PackageManagers::Conan);
    }

    #[test]
    #[serial]
    // #[ignore]
    fn test_vcpkg_init() -> anyhow::Result<()> {
        let variant = PackageManagers::Vcpkg;
        let name = "dummy";
        let cwd = env::current_dir()?;
        let path = cwd.join(name);
        let language = Language::C(CStandard::C89);
        let test_variant = TestFrameworks::CMocka;
        let test_framework = TestFramework::new(test_variant, path.clone());

        // Set-up
        create_dummy_project(&path)?;

        // Test
        let pkg_manager = PackageManager::new(variant, path.clone(), test_framework, language);
        pkg_manager.init()?;

        // Validate
        assert!(check_file_exits(&path.join("vcpkg.json")));
        assert!(check_file_exits(&path.join("vcpkg-configuration.json")));

        // Clean-up
        delete_dummy_project(&path)?;

        Ok(())
    }

    #[test]
    #[serial]
    fn test_vcpkg_cmocka_config() -> anyhow::Result<()> {
        let variant = PackageManagers::Vcpkg;
        let name = "dummy";
        let cwd = env::current_dir()?;
        let path = cwd.join(name);
        let language = Language::C(CStandard::C89);
        let test_variant = TestFrameworks::CMocka;
        let test_framework = TestFramework::new(test_variant, path.clone());

        // Set-up
        create_dummy_project(&path)?;

        let pkg_manager = PackageManager::new(variant, path.clone(), test_framework, language);
        pkg_manager.init()?;

        // Test
        pkg_manager.config()?;

        // Validate
        let expected = json!({
          "dependencies": [
            "cmocka"
          ]
        });
        let actual: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(path.join("vcpkg.json"))?)?;

        assert_eq!(actual, expected);
        assert!(check_file_exits(
            &path.join("vcpkg_installed").join("vcpkg")
        ));

        // Clean-up
        delete_dummy_project(&path)?;

        Ok(())
    }

    #[test]
    #[serial]
    fn test_vcpkg_gtest_config() -> anyhow::Result<()> {
        let variant = PackageManagers::Vcpkg;
        let name = "dummy";
        let cwd = env::current_dir()?;
        let path = cwd.join(name);
        let language = Language::Cpp(CppStandard::Cpp14);
        let test_variant = TestFrameworks::GTest;
        let test_framework = TestFramework::new(test_variant, path.clone());

        // Set-up
        create_dummy_project(&path)?;

        let pkg_manager = PackageManager::new(variant, path.clone(), test_framework, language);
        pkg_manager.init()?;

        // Test
        pkg_manager.config()?;

        // Validate
        let expected = json!({
          "dependencies": [
            "gtest"
          ]
        });
        let actual: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(path.join("vcpkg.json"))?)?;

        assert_eq!(actual, expected);
        assert!(check_file_exits(
            &path.join("vcpkg_installed").join("vcpkg")
        ));

        // Clean-up
        delete_dummy_project(&path)?;

        Ok(())
    }
}
