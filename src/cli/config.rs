use crate::{core::ForgeConfig, Result};
use clap::Args;

#[derive(Debug, Args)]
pub struct ConfigArgs {
    /// Makes compile_commnds.json and syslink to root | Default: True
    pub compile_commands: Option<bool>,

    /// Builder configr flags (e.g. -Wall -DDEBUG)
    #[arg(last = true)]
    pub extra: Vec<String>,
}

impl ConfigArgs {
    pub fn process_command(&self) -> Result<()> {
        let config = ForgeConfig::from_file()?;

        let compile_cmds = match self.compile_commands {
            Some(b) => b,
            None => true,
        };

        config.configure_builder(compile_cmds, &self.extra)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::init::InitArgs;
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
        env::set_current_dir(&cwd)?;

        // Validate
        assert!(check_file_exits(
            &path.join("build").join("compile_commands.json")
        ));
        assert!(check_file_exits(&path.join("compile_commands.json")));

        // Clean-up
        delete_dummy_project(&path)?;

        Ok(())
    }
}
