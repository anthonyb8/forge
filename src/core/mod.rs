pub mod build_system;
pub mod compiler;
pub mod language;
pub mod package_manager;
pub mod scaffolder;
pub mod test_framework;

use crate::Result;
use build_system::{BuildOptions, BuildSystem};
use language::Language;
use package_manager::PackageManager;
use scaffolder::Scaffolder;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use test_framework::TestFramework;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    name: String,
    language: Language,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolsConfig {
    compiler_path: String,
    package_manager: PackageManager,
    build_system: BuildSystem,
    test_framework: TestFramework,
    intellisense_mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
        build_system: BuildSystem,
        package_manager: PackageManager,
        test_framework: TestFramework,
        intellisense_mode: String,
    ) -> ForgeConfig {
        ForgeConfig {
            directory,
            project: ProjectConfig { name, language },
            tools: ToolsConfig {
                compiler_path,
                package_manager,
                build_system,
                test_framework,
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
        self.tools.build_system.config()?;
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
            self.tools.build_system.config()?;
        }

        Ok(())
    }

    pub fn build(&self, options: &BuildOptions) -> Result<()> {
        self.tools.build_system.build(options)?;

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

    pub fn test(&self, verbose: Option<&str>) -> Result<()> {
        let mut args = vec!["--test-dir", "build"];

        if let Some(s) = verbose {
            args.push(s);
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
    use build_system::BuildSystems::CMake;
    use language::CStandard::C11;
    use language::CppStandard::Cpp11;
    use package_manager::PackageManagers::Vcpkg;
    use test_framework::TestFrameworks::{CMocka, GTest};

    fn c_config() -> ForgeConfig {
        let name = "dummy_c".to_string();
        let directory = std::env::current_dir().unwrap().join("tests").join(&name);
        let language = Language::C(C11);
        let compiler_path = "clang".to_string();
        let intellisense = "testing".to_string();
        let test_framework = TestFramework::new(CMocka, directory.clone());
        let packagemanager = PackageManager::new(
            Vcpkg,
            directory.clone(),
            test_framework.clone(),
            language.clone(),
        );

        let builsystem = BuildSystem::new(
            name.clone(),
            CMake,
            directory.clone(),
            test_framework.clone(),
            language.clone(),
        );

        ForgeConfig::new(
            name,
            directory.clone(),
            language,
            compiler_path,
            builsystem,
            packagemanager,
            test_framework,
            intellisense,
        )
    }

    fn cpp_config() -> ForgeConfig {
        let name = "dummy_cpp".to_string();
        let directory = std::env::current_dir().unwrap().join("tests").join(&name);
        let language = Language::Cpp(Cpp11);
        let compiler_path = "clang++".to_string();
        let intellisense = "testing".to_string();
        let test_framework = TestFramework::new(GTest, directory.clone());
        let packagemanager = PackageManager::new(
            Vcpkg,
            directory.clone(),
            test_framework.clone(),
            language.clone(),
        );

        let builsystem = BuildSystem::new(
            name.clone(),
            CMake,
            directory.clone(),
            test_framework.clone(),
            language.clone(),
        );

        ForgeConfig::new(
            name,
            directory.clone(),
            language,
            compiler_path,
            builsystem,
            packagemanager,
            test_framework,
            intellisense,
        )
    }

    #[test]
    #[ignore]
    fn test_from_file() -> anyhow::Result<()> {
        let directory = std::env::current_dir().unwrap().join("tests").join("dummy");
        let contents = fs::read_to_string(directory.join("create_forge.toml"))?;
        let config: ForgeConfig = toml::from_str(&contents)?;
        println!("{:?}", config);

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_to_file() -> anyhow::Result<()> {
        let name = "dummy".to_string();
        let directory = std::env::current_dir().unwrap().join("tests").join(&name);
        let language = Language::Cpp(language::CppStandard::Cpp11);
        let compiler_path = "clang++".to_string();
        let intellisense = "testing".to_string();
        let test_framework = TestFramework::new(GTest, directory.clone());
        let packagemanager = PackageManager::new(
            Vcpkg,
            directory.clone(),
            test_framework.clone(),
            language.clone(),
        );

        let builsystem = BuildSystem::new(
            name.clone(),
            CMake,
            directory.clone(),
            test_framework.clone(),
            language.clone(),
        );
        let _config = ForgeConfig::new(
            name,
            directory.clone(),
            language,
            compiler_path,
            builsystem,
            packagemanager,
            test_framework,
            intellisense,
        );

        // let toml_str = toml::to_string_pretty(&config)?;
        // fs::write(directory.join("create_forge.toml"), toml_str)?;
        // println!("{:?}", toml_str);

        Ok(())
    }

    #[test]
    #[ignore = ""]
    fn test_init_cpp() -> anyhow::Result<()> {
        let config = cpp_config();
        config.init()?;

        Ok(())
    }

    #[test]
    fn test_init_c() -> anyhow::Result<()> {
        let config = c_config();
        config.init()?;

        Ok(())
    }
}
