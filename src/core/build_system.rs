use super::{language::Language, test_framework::TestFramework};
use crate::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::{path::PathBuf, process::Command};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BuildSystems {
    CMake,
    Meson,
    Make,
}

impl BuildSystems {
    pub fn variants() -> Vec<&'static str> {
        vec!["CMake", "Meson", "Make"]
    }

    pub fn from_str(s: &str) -> BuildSystems {
        match s {
            "CMake" => BuildSystems::CMake,
            "Meson" => BuildSystems::Meson,
            "Make" => BuildSystems::Make,
            _ => BuildSystems::CMake,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildSystem {
    name: String,
    variant: BuildSystems,
    directory: PathBuf,
    test_framework: TestFramework,
    language: Language,
}

impl BuildSystem {
    pub fn new(
        name: String,
        variant: BuildSystems,
        directory: PathBuf,
        test_framework: TestFramework,
        language: Language,
    ) -> BuildSystem {
        BuildSystem {
            name,
            variant,
            directory,
            test_framework,
            language,
        }
    }

    pub fn init(&self) -> Result<()> {
        match self.variant {
            BuildSystems::CMake => CMakeBuilder::init(
                &self.name,
                &self.directory,
                &self.language,
                &self.test_framework,
            ),
            BuildSystems::Meson => MesonBuilder::init(),
            BuildSystems::Make => MakeBuilder::init(),
        }
    }

    pub fn configure(&self, compile_commands: bool, flags: &Vec<String>) -> Result<()> {
        match self.variant {
            BuildSystems::CMake => {
                CMakeBuilder::configure(&self.directory, compile_commands, flags)
            }
            BuildSystems::Meson => MesonBuilder::configure(),
            BuildSystems::Make => MakeBuilder::configure(),
        }
    }

    pub fn build(&self, flags: Option<&Vec<String>>) -> Result<()> {
        match self.variant {
            BuildSystems::CMake => CMakeBuilder::build(&self.directory, flags),
            BuildSystems::Meson => MesonBuilder::build(),
            BuildSystems::Make => MakeBuilder::build(),
        }
    }
}

pub struct CMakeBuilder {}

impl CMakeBuilder {
    fn init(
        name: &String,
        path: &PathBuf,
        language: &Language,
        test_framework: &TestFramework,
    ) -> Result<()> {
        let lang = language.cmake_identifier();
        let standard = language.version();
        let src_suffix = language.src_suffix();
        let test_pkg = test_framework.as_str();
        let test_targets = test_framework.cmake_target();

        let mut contents = vec![];

        contents.push("# General");
        contents.push("cmake_minimum_required(VERSION 3.14)");
        let project = format!("project({} VERSION 1.0 LANGUAGES {})\n", name, lang);
        contents.push(&project);
        let set = format!("set(CMAKE_{}_STANDARD {})", lang, standard);
        contents.push(&set);
        let required = format!("set(CMAKE_{}_STANDARD_REQUIRED ON)\n", lang);
        contents.push(&required);

        contents.push("# Library");
        let lib = format!("add_library({}Lib src/lib.{})", name, src_suffix);
        let dir = format!(
            "target_include_directories({}Lib PUBLIC include/ libs/)\n",
            name
        );
        contents.push(&lib);
        contents.push(&dir);

        contents.push("# Binary");
        let exec = format!("add_executable({} src/main.{})", name, src_suffix);
        let exc_link = format!("target_link_libraries({} PRIVATE {}Lib)\n", name, name);
        contents.push(&exec);
        contents.push(&exc_link);

        contents.push("# Testing");
        contents.push("enable_testing()");
        let find = format!("find_package({} REQUIRED)\n", test_pkg);
        contents.push(&find);
        let test_exec = format!(
            "add_executable({}Tests test/test_lib.{} test/test_main.{} )",
            name, src_suffix, src_suffix
        );
        contents.push(&test_exec);
        let link_test = format!(
            "target_link_libraries({}Tests PRIVATE
                {}
                {}Lib
            )",
            name, test_targets, name
        );
        contents.push(&link_test);
        let test_dir = format!(
            "target_include_directories({}Tests PRIVATE 
                include/ 
                src/
            )",
            name
        );
        contents.push(&test_dir);
        let add_test = format!("add_test(NAME {}Tests COMMAND {}Tests)\n", name, name);
        contents.push(&add_test);

        contents.push("# Compiled output file");
        let target = format!("set_target_properties({} PROPERTIES", name);
        contents.push(&target);
        contents.push("    RUNTIME_OUTPUT_DIRECTORY \"${CMAKE_BINARY_DIR}/bin\"");
        contents.push(")");

        let path = path.join("CMakeLists.txt");
        match fs::write(path, contents.join("\n")) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn configure(path: &PathBuf, compile_cmds: bool, flags: &Vec<String>) -> Result<()> {
        let mut args: Vec<String> = vec!["-S".into(), ".".into(), "-B".into(), "build".into()];

        if compile_cmds {
            args.push("-DCMAKE_EXPORT_COMPILE_COMMANDS=ON".into());
            args.append(&mut flags.clone());
        } else {
            args.append(&mut flags.clone());
        }

        let config_cmd = Command::new("cmake")
            .args(args)
            .current_dir(path)
            .status()?;

        if config_cmd.success() && compile_cmds {
            CMakeBuilder::link_command_compiler(path)?;
        }

        Ok(())
    }

    /// Utility function to be used by default to system link the compile_commands.json to project
    /// root
    fn link_command_compiler(path: &PathBuf) -> Result<()> {
        let ln_status = Command::new("ln")
            .args([
                "-sf",
                "build/compile_commands.json",
                "compile_commands.json",
            ])
            .current_dir(path)
            .status()?;
        if !ln_status.success() {
            eprintln!("Failed to create symlink for the compile_commands.json");
        }
        Ok(())
    }

    fn build(path: &PathBuf, flags: Option<&Vec<String>>) -> Result<()> {
        let mut args = vec!["--build".to_string(), "build".to_string()];

        if let Some(f) = flags {
            args.append(&mut f.clone());
        }

        let build_status = Command::new("cmake")
            .args(args)
            .current_dir(path)
            .status()?;

        if !build_status.success() {
            eprintln!("Compilation failed.");
        }
        Ok(())
    }
}

pub struct MesonBuilder {}

impl MesonBuilder {
    fn init() -> Result<()> {
        Ok(())
    }

    fn configure() -> Result<()> {
        Ok(())
    }

    fn build() -> Result<()> {
        Ok(())
    }
}

pub struct MakeBuilder {}

impl MakeBuilder {
    fn init() -> Result<()> {
        Ok(())
    }

    fn configure() -> Result<()> {
        Ok(())
    }

    fn build() -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{language::CStandard, test_framework::TestFrameworks};
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
    fn test_build_system_from_str() {
        let s = "CMake";
        let v = BuildSystems::from_str(s);
        assert_eq!(v, BuildSystems::CMake);

        let s = "Meson";
        let v = BuildSystems::from_str(s);
        assert_eq!(v, BuildSystems::Meson);

        let s = "Make";
        let v = BuildSystems::from_str(s);
        assert_eq!(v, BuildSystems::Make);
    }

    // CMake
    #[test]
    #[serial]
    // #[ignore]
    fn test_cmake_builder_init() -> anyhow::Result<()> {
        let cwd = env::current_dir()?;
        let name = "dummy".to_string();
        let path = cwd.join(&name);
        let language = Language::C(CStandard::C89);
        let test_variant = TestFrameworks::CMocka;
        let test_framework = TestFramework::new(test_variant, path.clone());
        let variant = BuildSystems::CMake;

        // Set-up
        create_dummy_project(&path)?;

        // Test
        let build_system = BuildSystem::new(name, variant, path.clone(), test_framework, language);
        build_system.init()?;

        // Validate
        let file_check = check_file_exits(&path.join("CMakeLists.txt"));
        assert!(file_check);

        // Clean-up
        delete_dummy_project(&path)?;

        Ok(())
    }

    #[test]
    #[serial]
    // #[ignore]
    fn test_cmake_builder_configure_default() -> anyhow::Result<()> {
        let cwd = env::current_dir()?;
        let name = "dummy".to_string();
        let path = cwd.join(&name);
        let language = Language::C(CStandard::C89);
        let test_variant = TestFrameworks::CMocka;
        let test_framework = TestFramework::new(test_variant, path.clone());
        let variant = BuildSystems::CMake;

        // Set-up
        create_dummy_project(&path)?;
        let build_system = BuildSystem::new(name, variant, path.clone(), test_framework, language);
        build_system.init()?;

        // Test
        let flags = vec!["".to_string()];
        build_system.configure(true, &flags)?;

        // Validate
        let file_check = check_file_exits(&path.join("CMakeLists.txt"));
        assert!(file_check);

        // Clean-up
        delete_dummy_project(&path)?;

        Ok(())
    }

    #[test]
    #[serial]
    // #[ignore]
    fn test_cmake_builder_configure_no_compile_cmds() -> anyhow::Result<()> {
        let cwd = env::current_dir()?;
        let name = "dummy".to_string();
        let path = cwd.join(&name);
        let language = Language::C(CStandard::C89);
        let test_variant = TestFrameworks::CMocka;
        let test_framework = TestFramework::new(test_variant, path.clone());
        let variant = BuildSystems::CMake;

        // Set-up
        create_dummy_project(&path)?;
        let build_system = BuildSystem::new(name, variant, path.clone(), test_framework, language);
        build_system.init()?;

        // Test
        let flags = vec!["".to_string()];
        build_system.configure(false, &flags)?;

        // Validate
        let file_check = check_file_exits(&path.join("CMakeLists.txt"));
        assert!(file_check);

        // Clean-up
        delete_dummy_project(&path)?;

        Ok(())
    }

    #[test]
    #[serial]
    // #[ignore]
    fn test_cmake_builder_build() -> anyhow::Result<()> {
        let cwd = env::current_dir()?;
        let name = "dummy".to_string();
        let path = cwd.join(&name);
        let language = Language::C(CStandard::C89);
        let test_variant = TestFrameworks::CMocka;
        let test_framework = TestFramework::new(test_variant, path.clone());
        let variant = BuildSystems::CMake;

        // Set-up
        create_dummy_project(&path)?;

        let build_system = BuildSystem::new(name, variant, path.clone(), test_framework, language);
        build_system.init()?;

        let flags = vec!["".to_string()];
        build_system.configure(true, &flags)?;

        // Test
        // let build_flags = vec!["".to_string()];
        build_system.build(None)?;

        // Validate
        let file_check = check_file_exits(&path.join("CMakeLists.txt"));
        assert!(file_check);

        // Clean-up
        delete_dummy_project(&path)?;

        Ok(())
    }
    // Make
    // Meson
}
