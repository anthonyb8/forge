use crate::{core::ForgeConfig, Result};
use clap::Args;

#[derive(Debug, Args)]
pub struct CleanArgs {}

impl CleanArgs {
    pub fn process_command(&self) -> Result<()> {
        let config = ForgeConfig::from_file()?;
        config.clean()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{config::ConfigArgs, init::InitArgs};
    use serial_test::serial;
    use std::{env, fs, path::PathBuf};

    // Utility functions
    fn create_dummy_project(path: &PathBuf) -> anyhow::Result<()> {
        fs::create_dir_all(path)?;
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
    fn test_process_command() -> anyhow::Result<()> {
        let name = "dummy";
        let cwd = env::current_dir()?;
        let path = cwd.join(&name);

        // Test
        create_dummy_project(&path)?;

        env::set_current_dir(&path)?;
        let args = InitArgs {};
        args.process_command()?;

        let config_args = ConfigArgs {
            compile_commands: None,
            extra: vec!["".to_string()],
        };
        config_args.process_command()?;

        // Check files exists
        assert_eq!(
            check_file_exits(&path.join("build").join("compile_commands.json")),
            true
        );

        let clean_args = CleanArgs {};
        clean_args.process_command()?;
        env::set_current_dir(&cwd)?;

        // Validate
        assert_eq!(
            check_file_exits(&path.join("build").join("compile_commands.json")),
            false
        );

        // Clean-up
        delete_dummy_project(&path)?;

        Ok(())
    }
}
