use clap::Parser;
use forge_lib::cli::Cli;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    args.command.process_command()?;

    Ok(())
}
