use super::build_system::{BuildSystem, BuildSystems};
use super::language::Language;
use super::package_manager::{PackageManager, PackageManagers};
use super::scaffolder::Scaffolder;
use super::test_framework::{TestFramework, TestFrameworks};
use crate::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectConfig {
    name: String,
    language: Language,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolsConfig {
    compiler_path: String,
    package_manager: PackageManager,
    build_system: BuildSystem,
    test_framework: TestFramework,
    intellisense_mode: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ForgeConfig {
    directory: PathBuf,
    project: ProjectConfig,
    tools: ToolsConfig,
}

impl ForgeConfig {
    pub fn new(
        name: String,
        directory: PathBuf,
        language: Language,
        compiler_path: String,
        build_system: BuildSystems,
        package_manager: PackageManagers,
        test_framework: TestFrameworks,
        intellisense_mode: String,
    ) -> ForgeConfig {
        let test_framework = TestFramework::new(test_framework, directory.clone());

        ForgeConfig {
            directory: directory.clone(),
            project: ProjectConfig {
                name: name.clone(),
                language: language.clone(),
            },
            tools: ToolsConfig {
                compiler_path,
                test_framework: test_framework.clone(),
                package_manager: PackageManager::new(
                    package_manager,
                    directory.clone(),
                    test_framework.clone(),
                    language.clone(),
                ),
                build_system: BuildSystem::new(
                    name.clone(),
                    build_system,
                    directory,
                    test_framework,
                    language,
                ),
                intellisense_mode,
            },
        }
    }

    pub fn from_file() -> Result<ForgeConfig> {
        let contents = fs::read_to_string("Forge.toml")?;
        let config: ForgeConfig = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn to_file(&self) -> Result<()> {
        let toml_str = toml::to_string_pretty(self)?;
        fs::write(self.directory.join("Forge.toml"), toml_str)?;
        Ok(())
    }

    pub fn init(&self) -> Result<()> {
        let scaffolder = Scaffolder::new(
            self.project.name.clone(),
            self.directory.clone(),
            self.project.language.clone(),
        );

        scaffolder.build()?;
        self.tools.package_manager.init()?;
        self.tools.package_manager.config()?;
        self.tools.build_system.init()?;
        // self.tools.build_system.config()?;
        self.to_file()?;

        Ok(())
    }

    pub fn clean(&self) -> Result<()> {
        if std::path::Path::new(".cache").exists() {
            std::fs::remove_dir_all(".cache")?;
        }

        if std::path::Path::new("build").exists() {
            std::fs::remove_dir_all("build")?;
            std::fs::create_dir("build")?;
            // self.tools.build_system.config()?;
        }

        Ok(())
    }

    pub fn configure_builder(&self, compile_commands: bool, flags: &Vec<String>) -> Result<()> {
        self.tools.build_system.configure(compile_commands, flags)?;
        Ok(())
    }

    pub fn build(&self, flags: Option<&Vec<String>>) -> Result<()> {
        self.tools.build_system.build(flags)?;

        Ok(())
    }

    pub fn run(&self) -> Result<()> {
        let bin = format!("./build/bin/{}", self.project.name);
        let run_cmd = Command::new(&bin).current_dir(".").status()?;

        if !run_cmd.success() {
            eprintln!("Failed: {:?}", run_cmd);
        }

        Ok(())
    }

    pub fn test(&self, flags: Option<&Vec<String>>) -> Result<()> {
        let mut args: Vec<String> = vec!["--test-dir".into(), "build".into()];

        if let Some(f) = flags {
            args.extend(f.clone());
        }

        let test_cmd = Command::new("ctest").args(args).current_dir(".").status()?;

        if !test_cmd.success() {
            eprintln!("Failed: {:?}", test_cmd);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        language::{CStandard, CppStandard, Language},
        package_manager::PackageManagers,
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
    #[serial]
    // #[ignore]
    fn test_to_file() -> anyhow::Result<()> {
        let name = "dummy".to_string();
        let cwd = std::env::current_dir()?;
        let path = cwd.join(&name);
        let language = Language::Cpp(CppStandard::Cpp11);
        let compiler_path = "clang++".to_string();
        let intellisense = "testing".to_string();
        let test_framework = TestFrameworks::GTest;
        let package_manager = PackageManagers::Vcpkg;
        let build_system = BuildSystems::CMake;

        // Set-up
        create_dummy_project(&path)?;

        // Test
        let config = ForgeConfig::new(
            name,
            path.clone(),
            language,
            compiler_path,
            build_system,
            package_manager,
            test_framework,
            intellisense,
        );
        config.to_file()?;

        // Validate
        assert!(check_file_exits(&path.join("Forge.toml")));

        env::set_current_dir(&path)?;
        let new_config: ForgeConfig = ForgeConfig::from_file()?;
        assert_eq!(config, new_config);
        env::set_current_dir(&cwd)?;

        // Clean-up
        delete_dummy_project(&path)?;

        Ok(())
    }

    #[test]
    #[serial]
    // #[ignore]
    fn test_init_cpp() -> anyhow::Result<()> {
        let name = "dummy".to_string();
        let cwd = std::env::current_dir()?;
        let path = cwd.join(&name);
        let language = Language::Cpp(CppStandard::Cpp11);
        let compiler_path = "clang++".to_string();
        let intellisense = "testing".to_string();
        let test_framework = TestFrameworks::GTest;
        let package_manager = PackageManagers::Vcpkg;
        let build_system = BuildSystems::CMake;

        // Set-up
        create_dummy_project(&path)?;

        // Test
        let config = ForgeConfig::new(
            name,
            path.clone(),
            language,
            compiler_path,
            build_system,
            package_manager,
            test_framework,
            intellisense,
        );
        config.init()?;

        // Validate
        assert!(check_file_exits(&path.join("CMakeLists.txt")));
        assert!(check_file_exits(&path.join(".gitignore")));
        assert!(check_file_exits(&path.join(".git")));
        assert!(check_file_exits(&path.join("include").join("lib.hpp")));
        assert!(check_file_exits(&path.join("src").join("lib.cpp")));
        assert!(check_file_exits(&path.join("src").join("main.cpp")));
        assert!(check_file_exits(&path.join("test").join("test_lib.cpp")));
        assert!(check_file_exits(&path.join("test").join("test_main.cpp")));
        assert!(check_file_exits(
            &path.join(".vscode").join("c_cpp_properties.json")
        ));

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

    #[test]
    #[serial]
    // #[ignore]
    fn test_init_c() -> anyhow::Result<()> {
        let name = "dummy".to_string();
        let cwd = std::env::current_dir()?;
        let path = cwd.join(&name);
        let language = Language::C(CStandard::C89);
        let compiler_path = "clang".to_string();
        let intellisense = "testing".to_string();
        let test_framework = TestFrameworks::CMocka;
        let package_manager = PackageManagers::Vcpkg;
        let build_system = BuildSystems::CMake;

        // Set-up
        create_dummy_project(&path)?;

        // Test
        let config = ForgeConfig::new(
            name,
            path.clone(),
            language,
            compiler_path,
            build_system,
            package_manager,
            test_framework,
            intellisense,
        );
        config.init()?;

        // Validate
        assert!(check_file_exits(&path.join("CMakeLists.txt")));
        assert!(check_file_exits(&path.join(".gitignore")));
        assert!(check_file_exits(&path.join(".git")));
        assert!(check_file_exits(&path.join("include").join("lib.h")));
        assert!(check_file_exits(&path.join("src").join("lib.c")));
        assert!(check_file_exits(&path.join("src").join("main.c")));
        assert!(check_file_exits(&path.join("test").join("test_lib.c")));
        assert!(check_file_exits(&path.join("test").join("test_main.c")));
        assert!(check_file_exits(
            &path.join(".vscode").join("c_cpp_properties.json")
        ));

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
}
