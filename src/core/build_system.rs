use super::{language::Language, test_framework::TestFramework};
use crate::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::{path::PathBuf, process::Command};

pub enum BuildType {
    Debug,
    Release,
}
impl BuildType {
    pub fn as_str(&self) -> &'static str {
        return match self {
            BuildType::Debug => "debug",
            BuildType::Release => "release",
        };
    }
}

pub struct BuildOptions {
    pub build_type: BuildType,
    pub verbose: bool,
}

impl BuildOptions {
    pub fn cmake_args(&self) -> Vec<&'static str> {
        let mut args = vec![];

        match self.verbose {
            true => args.push("--verbose"),
            false => (),
        };

        args.push("--config");
        args.push(self.build_type.as_str());
        args
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn config(&self) -> Result<()> {
        match self.variant {
            BuildSystems::CMake => CMakeBuilder::config(&self.directory),
            BuildSystems::Meson => MesonBuilder::config(),
            BuildSystems::Make => MakeBuilder::config(),
        }
    }

    pub fn build(&self, options: &BuildOptions) -> Result<()> {
        match self.variant {
            BuildSystems::CMake => CMakeBuilder::build(&self.directory, options),
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
            "target_include_directories({}Lib PUBLIC include/ lib/)\n",
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
            "add_executable({}Tests tests/test_lib.{})",
            name, src_suffix
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
        let add_test = format!("add_test(NAME {}Tests COMMAND {}Tests)", name, name);
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

    fn config(path: &PathBuf) -> Result<()> {
        let config_cmd = Command::new("cmake")
            .args(["-B", "build", "-DCMAKE_EXPORT_COMPILE_COMMANDS=ON"])
            .current_dir(path)
            .status()?;

        if config_cmd.success() {
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
        }
        Ok(())
    }

    fn build(path: &PathBuf, options: &BuildOptions) -> Result<()> {
        let mut args = vec!["--build", "build"];
        args.append(&mut options.cmake_args());
        println!("{:?}", args);

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

    fn build() -> Result<()> {
        Ok(())
    }
    fn config() -> Result<()> {
        Ok(())
    }
}

pub struct MakeBuilder {}

impl MakeBuilder {
    fn init() -> Result<()> {
        Ok(())
    }

    fn build() -> Result<()> {
        Ok(())
    }
    fn config() -> Result<()> {
        Ok(())
    }
}
