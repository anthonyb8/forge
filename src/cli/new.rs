use crate::core::build_system::{BuildSystem, BuildSystems};
use crate::core::compiler::detect_compilers;
use crate::core::package_manager::{PackageManager, PackageManagers};
use crate::core::test_framework::{TestFramework, TestFrameworks};
use crate::{
    core::{language::Language, ForgeConfig},
    Result,
};
use clap::Args;
use inquire::Select;
use std::env;

#[derive(Debug, Args)]
pub struct NewArgs {
    /// Name of new project directory.
    pub name: String,
}

impl NewArgs {
    pub fn process_command(&self) -> Result<()> {
        // Project
        let name = self.name.clone();
        let cwd = env::current_dir()?.join(&self.name);
        println!("{:?}", cwd);

        // Lanaguage
        let choice = Select::new("Langauge:", Language::variants()).prompt()?;
        let language = Language::from_str(choice);

        // Compiler
        let compiler_map = detect_compilers();
        let compiler = Select::new("Compiler:", compiler_map.keys().collect()).prompt()?;

        // Test Framework
        let choice = Select::new("Test Framework", TestFrameworks::variants()).prompt()?;
        let test_enum = TestFrameworks::from_str(&choice);
        let test_framework = TestFramework::new(test_enum, cwd.clone());

        // Build System
        let choice = Select::new("Build System:", BuildSystems::variants()).prompt()?;
        let build_enum = BuildSystems::from_str(&choice);
        let build_system = BuildSystem::new(
            name.clone().to_string(),
            build_enum,
            cwd.clone(),
            test_framework.clone(),
            language.clone(),
        );

        // Package Manager
        let choice = Select::new("Package Manager:", PackageManagers::variants()).prompt()?;
        let pkg_enum = PackageManagers::from_str(&choice);
        let package_manager = PackageManager::new(
            pkg_enum,
            cwd.clone(),
            test_framework.clone(),
            language.clone(),
        );

        let config = ForgeConfig::new(
            name.to_string(),
            cwd,
            language,
            compiler.to_string(),
            build_system,
            package_manager,
            test_framework,
            "intellisenseMOde ".to_string(),
        );

        config.init()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_process_command() -> anyhow::Result<()> {
        let cwd = env::current_dir()?;
        env::set_current_dir(cwd.join("tests"))?;

        // Test
        let args = NewArgs {
            name: "dummy_new".to_string(),
        };
        args.process_command()?;

        // Validate

        //Cleanup
        if cwd.join("dummy_new").exists() {
            println!("path exists");
            std::fs::remove_dir_all("build")?;
        }
        Ok(())
    }
}
