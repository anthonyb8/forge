use crate::core::build_system::{BuildOptions, BuildType};
use crate::core::ForgeConfig;
use clap::Args;

#[derive(Debug, Args)]
pub struct TestArgs {
    /// Use verbose output.
    #[arg(long)]
    pub verbose: bool,

    /// Use superverbose output.
    #[arg(long)]
    pub superverbose: bool,
}

impl TestArgs {
    pub fn process_command(&self) -> anyhow::Result<()> {
        let config = ForgeConfig::from_file()?;

        let options = BuildOptions {
            build_type: BuildType::Debug,
            verbose: false,
        };

        config.build(&options)?;

        if self.verbose {
            config.test(Some("--verbose"))?;
        } else if self.superverbose {
            config.test(Some("-VV"))?;
        } else {
            config.test(None)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::init::InitArgs;
    use std::env;

    #[test]
    fn test_test_debug() -> anyhow::Result<()> {
        let cwd = env::current_dir()?;
        let test_dir = env::set_current_dir(cwd.join("tests").join("dummy"))?;
        // let args = InitArgs {};
        // args.process_command()?;

        let test_args = TestArgs {
            verbose: false,
            superverbose: false,
        };

        test_args.process_command()?;

        Ok(())
    }
}
