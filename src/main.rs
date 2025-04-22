use clap::CommandFactory;
use clap_builder::builder::styling::{AnsiColor, Styles};
use forge_lib::cli::Cli;

fn main() {
    let styles = Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default());

    let mut cmd = Cli::command();

    cmd = cmd.styles(styles);

    let _matches = cmd.get_matches();
}
