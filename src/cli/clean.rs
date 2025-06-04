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
    use crate::cli::init::InitArgs;
    use std::env;

    #[test]
    fn test_build_debug() -> anyhow::Result<()> {
        let cwd = env::current_dir()?;
        let test_dir = env::set_current_dir(cwd.join("tests").join("dummy"))?;
        // let args = InitArgs {};
        // args.process_command()?;

        let clean_args = CleanArgs {};
        clean_args.process_command()?;

        Ok(())
    }
}
