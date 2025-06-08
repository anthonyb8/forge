use crate::{error, Error, Result};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};
use which::which;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TestFrameworks {
    GTest,
    CMocka,
    Boost,
    Unit,
}

impl TestFrameworks {
    pub fn variants() -> Vec<&'static str> {
        vec!["GTest", "CMocka", "Boost", "Unit"]
    }

    pub fn from_str(s: &str) -> TestFrameworks {
        match s {
            "GTest" => TestFrameworks::GTest,
            "CMocka" => TestFrameworks::CMocka,
            "Boost" => TestFrameworks::Boost,
            "Unit" => TestFrameworks::Unit,
            _ => TestFrameworks::GTest,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TestFramework {
    variant: TestFrameworks,
    dir: PathBuf,
}

impl TestFramework {
    pub fn new(variant: TestFrameworks, dir: PathBuf) -> TestFramework {
        TestFramework { variant, dir }
    }
    pub fn as_str(&self) -> &'static str {
        return match self.variant {
            TestFrameworks::GTest => "GTest",
            TestFrameworks::Boost => "Boost",
            TestFrameworks::CMocka => "CMocka",
            TestFrameworks::Unit => "Unit",
        };
    }

    pub fn cmake_target(&self) -> &'static str {
        return match self.variant {
            TestFrameworks::GTest => GTest::cmake_target(),
            TestFrameworks::Boost => Boost::cmake_target(),
            TestFrameworks::CMocka => CMocka::cmake_target(),
            TestFrameworks::Unit => Unit::cmake_target(),
        };
    }

    pub fn vcpkg_setup(&self) -> Result<()> {
        match self.variant {
            TestFrameworks::GTest => GTest::vcpkg_setup(&self.dir)?,
            TestFrameworks::Boost => Boost::vcpkg_setup(&self.dir)?,
            TestFrameworks::CMocka => CMocka::vcpkg_setup(&self.dir)?,
            TestFrameworks::Unit => Unit::vcpkg_setup(&self.dir)?,
        };
        Ok(())
    }

    pub fn conan_setup(&self) -> Result<()> {
        match self.variant {
            TestFrameworks::GTest => GTest::conan_setup()?,
            TestFrameworks::Boost => Boost::conan_setup()?,
            TestFrameworks::CMocka => CMocka::conan_setup()?,
            TestFrameworks::Unit => Unit::conan_setup()?,
        };
        Ok(())
    }
}

pub struct GTest {}

impl GTest {
    // Package Managers
    pub fn vcpkg_setup(dir: &PathBuf) -> Result<()> {
        match which("vcpkg") {
            Ok(_) => {
                let status = Command::new("vcpkg")
                    .args(["add", "port", "gtest"])
                    .current_dir(dir)
                    .status()?;

                if !status.success() {
                    eprintln!("Failed to retrieve GTest for vcpkg.")
                }
                Ok(())
            }
            Err(e) => Err(error!(CustomError, "{}", e)),
        }
    }

    pub fn conan_setup() -> Result<()> {
        Ok(())
    }

    // Build Systems
    pub fn cmake_target() -> &'static str {
        "GTest::gtest GTest::gtest_main"
    }
}

pub struct CMocka {}

impl CMocka {
    pub fn vcpkg_setup(dir: &PathBuf) -> Result<()> {
        match which("vcpkg") {
            Ok(_) => {
                let status = Command::new("vcpkg")
                    .args(["add", "port", "cmocka"])
                    .current_dir(dir)
                    .status()?;

                if !status.success() {
                    eprintln!("Failed to retrieve CMocka for vcpkg.")
                }
                Ok(())
            }
            Err(e) => Err(error!(CustomError, "{}", e)),
        }
    }

    pub fn conan_setup() -> Result<()> {
        Ok(())
    }

    // Build Systems
    pub fn cmake_target() -> &'static str {
        "cmocka::cmocka"
    }
}

pub struct Boost {}

impl Boost {
    pub fn vcpkg_setup(dir: &PathBuf) -> Result<()> {
        match which("vcpkg") {
            Ok(_) => {
                let status = Command::new("vcpkg")
                    .args(["add", "port", "Boost"])
                    .current_dir(dir)
                    .status()?;

                if !status.success() {
                    eprintln!("Failed to retrieve GTest for vcpkg.")
                }
                Ok(())
            }
            Err(e) => Err(error!(CustomError, "{}", e)),
        }
    }

    pub fn conan_setup() -> Result<()> {
        Ok(())
    }

    // Build Systems
    pub fn cmake_target() -> &'static str {
        "Boost::unit_test_framework\n"
    }
}

pub struct Unit {}

impl Unit {
    pub fn vcpkg_setup(dir: &PathBuf) -> Result<()> {
        match which("vcpkg") {
            Ok(_) => {
                let status = Command::new("vcpkg")
                    .args(["add", "port", "Unit"])
                    .current_dir(dir)
                    .status()?;

                if !status.success() {
                    eprintln!("Failed to retrieve CMocka for vcpkg.")
                }
                Ok(())
            }
            Err(e) => Err(error!(CustomError, "{}", e)),
        }
    }

    pub fn conan_setup() -> Result<()> {
        Ok(())
    }

    // Build Systems
    pub fn cmake_target() -> &'static str {
        "Unit"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        language::Language,
        language::{CStandard, CppStandard},
        package_manager::{PackageManager, PackageManagers},
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

    fn delete_dummy_project(path: &PathBuf) -> anyhow::Result<()> {
        fs::remove_dir_all(path)?;
        Ok(())
    }

    #[test]
    fn test_test_framework_from_str() {
        let s = "GTest";
        let v = TestFrameworks::from_str(s);
        assert_eq!(v, TestFrameworks::GTest);

        let s = "CMocka";
        let v = TestFrameworks::from_str(s);
        assert_eq!(v, TestFrameworks::CMocka);

        let s = "Boost";
        let v = TestFrameworks::from_str(s);
        assert_eq!(v, TestFrameworks::Boost);

        let s = "Unit";
        let v = TestFrameworks::from_str(s);
        assert_eq!(v, TestFrameworks::Unit);
    }

    #[test]
    #[serial]
    // #[ignore]
    fn test_gtest_vcpkg_setup() -> anyhow::Result<()> {
        let name = "dummy";
        let cwd = env::current_dir()?;
        let path = cwd.join(name);
        create_dummy_project(&path)?;

        let variant = PackageManagers::Vcpkg;
        let language = Language::Cpp(CppStandard::Cpp11);
        let test_variant = TestFrameworks::GTest;
        let test_framework = TestFramework::new(test_variant, path.clone());

        // Set-up
        let pkg_manager =
            PackageManager::new(variant, path.clone(), test_framework.clone(), language);
        pkg_manager.init()?;

        // Test
        test_framework.vcpkg_setup()?;

        // Validate
        let expected = json!({
          "dependencies": [
            "gtest"
          ]
        });
        let actual: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(path.join("vcpkg.json"))?)?;

        assert_eq!(actual, expected);

        // Clean-up
        delete_dummy_project(&path)?;

        Ok(())
    }

    #[test]
    #[serial]
    fn test_cmocka_vcpkg_setup() -> anyhow::Result<()> {
        let name = "dummy";
        let cwd = env::current_dir()?;
        let path = cwd.join(name);
        create_dummy_project(&path)?;

        let variant = PackageManagers::Vcpkg;
        let language = Language::C(CStandard::C89);
        let test_variant = TestFrameworks::CMocka;
        let test_framework = TestFramework::new(test_variant, path.clone());

        // Set-up
        let pkg_manager =
            PackageManager::new(variant, path.clone(), test_framework.clone(), language);
        pkg_manager.init()?;

        // Test
        test_framework.vcpkg_setup()?;

        // Validate
        let expected = json!({
          "dependencies": [
            "cmocka"
          ]
        });
        let actual: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(path.join("vcpkg.json"))?)?;

        assert_eq!(actual, expected);

        // Clean-up
        delete_dummy_project(&path)?;

        Ok(())
    }
}
