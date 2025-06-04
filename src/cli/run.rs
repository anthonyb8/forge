use crate::core::build_system::{BuildOptions, BuildType};
use crate::{core::ForgeConfig, Result};
use clap::Args;

#[derive(Debug, Args)]
pub struct RunArgs {
    /// Build in release mode.
    #[arg(long)]
    pub release: bool,

    /// Use verbose output.
    #[arg(long)]
    pub verbose: bool,
}

impl RunArgs {
    pub fn process_command(&self) -> Result<()> {
        let config = ForgeConfig::from_file()?;
        let mut build_type = BuildType::Debug;

        if self.release {
            build_type = BuildType::Release;
        }

        let options = BuildOptions {
            build_type,
            verbose: self.verbose,
        };

        config.build(&options)?;
        config.run()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::init::InitArgs;
    use std::env;

    #[test]
    fn test_build_debug() -> anyhow::Result<()> {
        let cwd = env::current_dir()?;
        let test_dir = env::set_current_dir(cwd.join("tests").join("dummy"))?;
        // let args = InitArgs {};
        // args.process_command()?;

        let clean_args = RunArgs {
            release: false,
            verbose: false,
        };
        clean_args.process_command()?;

        Ok(())
    }
}
