use crate::core::Config;
use crate::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct NewArgs {
    pub name: String,

    #[arg(long)]
    pub test: bool,
}

impl NewArgs {
    pub fn process_command(&self) -> Result<()> {
        let config = Config::new(&self.name)?
            .language_standard()?
            .compiler()?
            .test_framework()?
            .build_system()?
            .package_manager()?;

        config.build()?;

        Ok(())
    }
}
