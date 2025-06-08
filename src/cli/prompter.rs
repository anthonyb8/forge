use crate::core::build_system::BuildSystems;
use crate::core::compiler::detect_compilers;
use crate::core::package_manager::PackageManagers;
use crate::core::test_framework::TestFrameworks;
use crate::{core::language::Language, Result};
use inquire::Select;

#[cfg(not(test))]
pub fn get_prompter() -> impl Prompter {
    RealPrompter {}
}

#[cfg(test)]
pub fn get_prompter() -> impl Prompter {
    MockPrompter {}
}

pub trait Prompter {
    fn select_language(&self) -> Result<Language>;
    fn select_compiler(&self) -> Result<String>;
    fn select_test_framework(&self) -> Result<TestFrameworks>;
    fn select_build_system(&self) -> Result<BuildSystems>;
    fn select_package_manager(&self) -> Result<PackageManagers>;
}

pub struct RealPrompter {}

impl Prompter for RealPrompter {
    fn select_language(&self) -> Result<Language> {
        let choice = Select::new("Langauge:", Language::variants()).prompt()?;
        Ok(Language::from_str(choice))
    }
    fn select_compiler(&self) -> Result<String> {
        let compiler_map = detect_compilers();
        let compiler = Select::new("Compiler:", compiler_map.keys().collect()).prompt()?;
        Ok(compiler.to_string())
    }
    fn select_test_framework(&self) -> Result<TestFrameworks> {
        let choice = Select::new("Test Framework", TestFrameworks::variants()).prompt()?;
        let test_framework = TestFrameworks::from_str(&choice);
        Ok(test_framework)
    }

    fn select_build_system(&self) -> Result<BuildSystems> {
        let choice = Select::new("Build System:", BuildSystems::variants()).prompt()?;
        let build_system = BuildSystems::from_str(&choice);
        Ok(build_system)
    }
    fn select_package_manager(&self) -> Result<PackageManagers> {
        let choice = Select::new("Package Manager:", PackageManagers::variants()).prompt()?;
        let package_manager = PackageManagers::from_str(&choice);
        Ok(package_manager)
    }
}

pub struct MockPrompter {}

impl Prompter for MockPrompter {
    fn select_language(&self) -> Result<Language> {
        Ok(Language::C(crate::core::language::CStandard::C99))
    }
    fn select_compiler(&self) -> Result<String> {
        Ok("clang".to_string())
    }
    fn select_test_framework(&self) -> Result<TestFrameworks> {
        Ok(TestFrameworks::CMocka)
    }

    fn select_build_system(&self) -> Result<BuildSystems> {
        Ok(BuildSystems::CMake)
    }
    fn select_package_manager(&self) -> Result<PackageManagers> {
        Ok(PackageManagers::Vcpkg)
    }
}
