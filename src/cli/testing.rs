use crate::{core::ForgeConfig, Result};
use ansi_term::Colour::{Cyan, Green};
use clap::Args;

fn ctest_help() -> Result<()> {
    let output = std::process::Command::new("ctest").arg("--help").output()?;
    let binding = String::from_utf8_lossy(&output.stdout);
    let text = binding.as_ref();
    let mut lines = text.lines().peekable();
    let mut in_options = false;

    println!(
        "{} {}\n",
        Green.paint("Usage:"),
        Cyan.paint("forge test -- [OPTIONS]")
    );

    let mut options = Vec::new();
    let mut descriptions = Vec::new();

    while let Some(line) = lines.next() {
        let trimmed = line.trim();

        if trimmed == "Options" {
            in_options = true;
            continue;
        }

        if in_options {
            if trimmed.is_empty() {
                continue;
            }

            // Handle new option line
            if trimmed.starts_with('-') {
                if let Some((left, right)) = trimmed.rsplit_once("= ") {
                    options.push(left.trim_end());
                    descriptions.push(right.trim_start().to_string());
                } else {
                    options.push(trimmed.trim_end());
                    descriptions.push("".to_string());
                };

                // Now look ahead for additional description lines
                while let Some(next_line) = lines.peek() {
                    let next_trimmed = next_line.trim();
                    if next_trimmed.starts_with('-') || next_trimmed.is_empty() {
                        break;
                    }

                    let cleaned = next_trimmed
                        .trim_start_matches('=')
                        .trim_start()
                        .to_string();

                    options.push("");
                    descriptions.push(cleaned);
                    lines.next();
                }
            }
        }
    }

    let desc_indent = 32;

    println!("{}", Green.bold().paint("Options:"));
    for i in 0..options.len() {
        let indent = if desc_indent >= options[i].len() {
            desc_indent - options[i].len()
        } else {
            1
        };

        print!("  {}", Cyan.paint(options[i]));
        print!("{}", " ".repeat(indent));
        println!("{}", descriptions[i]);
    }
    Ok(())
}

#[derive(Debug, Args)]
#[command(disable_help_flag = true)]
pub struct TestArgs {
    /// Help
    #[arg(short, long)]
    pub help: bool,
    /// CTest flags
    #[arg(last = true)]
    pub options: Option<Vec<String>>,
}

impl TestArgs {
    pub fn process_command(&self) -> anyhow::Result<()> {
        if self.help {
            return Ok(ctest_help()?);
        }

        let config = ForgeConfig::from_file()?;
        config.test(self.options.as_ref())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{build::BuildArgs, config::ConfigArgs, init::InitArgs};
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
    // #[ignore]
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

        let build_args = BuildArgs { options: None };
        build_args.process_command()?;

        let test_args = TestArgs {
            help: false,
            options: None,
        };
        test_args.process_command()?;
        env::set_current_dir(&cwd)?;

        // Validate
        assert!(check_file_exits(&path.join("build").join("dummyTests")));

        // Clean-up
        delete_dummy_project(&path)?;

        Ok(())
    }

    #[test]
    #[serial]
    #[ignore]
    fn test_ctest_display() -> anyhow::Result<()> {
        ctest_help()?;

        Ok(())
    }
}
