mod build;
mod clean;
mod config;
pub mod init;
mod new;
pub mod prompter;
mod run;
mod testing;

use build::BuildArgs;
use clap::{Parser, Subcommand};
use clap_builder::builder::styling::{AnsiColor, Styles};
use clean::CleanArgs;
use config::ConfigArgs;
use init::InitArgs;
use new::NewArgs;
use run::RunArgs;
use testing::TestArgs;

pub const CLAP_STYLING: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Cyan.on_default())
    .placeholder(AnsiColor::Cyan.on_default());

//  Cli commands
#[derive(Debug, Parser)]
#[command(styles = CLAP_STYLING)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Clean build artifacts.
    Config(ConfigArgs),
    /// Build an existing project.
    Build(BuildArgs),
    /// Initialize project in current working directory.
    Init(InitArgs),
    /// Create a new project including directory.
    New(NewArgs),
    /// Run binary build.
    Run(RunArgs),
    /// Run compiled tests.
    Test(TestArgs),
    /// Clean build artifacts.
    Clean(CleanArgs),
}

impl Commands {
    pub fn process_command(&self) -> anyhow::Result<()> {
        match self {
            Commands::Config(args) => Ok(args.process_command()?),
            Commands::Build(args) => Ok(args.process_command()?),
            Commands::Init(args) => Ok(args.process_command()?),
            Commands::New(args) => Ok(args.process_command()?),
            Commands::Run(args) => Ok(args.process_command()?),
            Commands::Test(args) => Ok(args.process_command()?),
            Commands::Clean(args) => Ok(args.process_command()?),
        }
    }
}
