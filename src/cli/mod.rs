mod build;
mod new;
mod run;
mod testing;

use crate::{execute_cmd, Result};
use build::BuildArgs;
use clap::{Parser, Subcommand};
use clap_builder::builder::styling::{AnsiColor, Styles};
use new::NewArgs;
use run::RunArgs;
use testing::TestArgs;

pub const CLAP_STYLING: Styles = Styles::styled()
    .header(AnsiColor::Yellow.on_default())
    .usage(AnsiColor::Green.on_default())
    .literal(AnsiColor::Green.on_default())
    .placeholder(AnsiColor::Green.on_default());

//  Cli commands
#[derive(Debug, Parser)]
#[command(styles = CLAP_STYLING)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Build an existing project.
    Build(BuildArgs),
    /// Create a new project including directory.
    New(NewArgs),
    /// Run binary build.
    Run(RunArgs),
    /// Run compiled tests.
    Test(TestArgs),
    /// Clean build artifacts.
    Clean,
}

impl Commands {
    pub fn process_command(&self) -> anyhow::Result<()> {
        match self {
            Commands::Build(args) => Ok(args.process_command()?),
            Commands::New(args) => Ok(args.process_command()?),
            Commands::Run(args) => Ok(args.process_command()?),
            Commands::Test(args) => Ok(args.process_command()?),
            Commands::Clean => Ok(clean()?),
        }
    }
}

fn clean() -> Result<()> {
    if std::path::Path::new("build").exists() {
        std::fs::remove_dir_all("build")?;
        std::fs::create_dir("build")?;
    }

    if std::path::Path::new(".cache").exists() {
        std::fs::remove_dir_all(".cache")?;
    }

    let cmds = [
        format!("cmake -B build -DCMAKE_EXPORT_COMPILE_COMMANDS=ON"),
        format!("ln -sf build/compile_commands.json compile_commands.json"),
    ];

    for cmd in cmds {
        match execute_cmd(&cmd) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    Ok(())
}
