use super::{BuildSystem, LanguageStd, PackageManager, TestFrameworkEnum};
use std::fs;
use std::path::PathBuf;

pub struct ForgeConfig {
    name: String,
    directory: PathBuf,
    language: LanguageStd,
    compiler_path: String,
    build_system: BuildSystem,
    package_manager: PackageManager,
    test_framework: TestFrameworkEnum,
    intellisense_mode: String,
}

impl ForgeConfig {
    pub fn new(
        name: String,
        directory: PathBuf,
        language: LanguageStd,
        compiler_path: String,
        build_system: BuildSystem,
        package_manager: PackageManager,
        test_framework: TestFrameworkEnum,
        intellisense_mode: String,
    ) -> ForgeConfig {
        ForgeConfig {
            name,
            directory,
            language,
            compiler_path,
            build_system,
            package_manager,
            test_framework,
            intellisense_mode,
        }
    }

    pub fn from_file() -> ForgeConfig {
        let contents = fs::read_to_string(file_path);
    }
}
