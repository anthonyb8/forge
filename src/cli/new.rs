use crate::{core::ForgeConfig, Result};
use clap::Args;
use std::env;

use super::prompter::{get_prompter, Prompter};

#[derive(Debug, Args)]
pub struct NewArgs {
    /// Name of new project directory.
    pub name: String,
}

impl NewArgs {
    pub fn process_command(&self) -> Result<()> {
        let name = self.name.clone();
        let cwd = env::current_dir()?.join(&self.name);

        let prompter = get_prompter();
        let language = prompter.select_language()?;
        let compiler = prompter.select_compiler()?;
        let test_framework = prompter.select_test_framework()?;
        let build_system = prompter.select_build_system()?;
        let package_manager = prompter.select_package_manager()?;

        let config = ForgeConfig::new(
            name.to_string(),
            cwd,
            language,
            compiler.to_string(),
            build_system,
            package_manager,
            test_framework,
            "intellisenseMode ".to_string(),
        );

        config.init()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use serial_test::serial;
    use std::{fs, path::PathBuf};

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
    fn test_process_command() -> anyhow::Result<()> {
        let name = "dummy";
        let cwd = env::current_dir()?;
        let path = cwd.join(&name);

        // Test
        let args = NewArgs {
            name: name.to_string(),
        };
        args.process_command()?;

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
