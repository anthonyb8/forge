use super::execute_cmd;
use crate::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct TestArgs {
    #[arg(long)]
    pub verbose: bool,

    #[arg(long)]
    pub superverbose: bool,
}

impl TestArgs {
    pub fn process_command(&self) -> anyhow::Result<()> {
        let mut tester = Tester::new();

        if self.verbose {
            tester = tester.verbose();
        }

        if self.superverbose {
            tester = tester.superverbose();
        }

        tester.build()?;

        Ok(())
    }
}

pub struct Tester {
    verbose: bool,
    superverbose: bool,
}

impl Tester {
    pub fn new() -> Self {
        Tester {
            verbose: false,
            superverbose: false,
        }
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }

    pub fn superverbose(mut self) -> Self {
        self.superverbose = true;
        self
    }

    pub fn build(&self) -> Result<()> {
        let build_cmd = "cmake --build ./build";
        let mut test_cmd = String::from("cd build && ctest");

        if self.verbose {
            test_cmd.push_str(" --verbose");
        }

        if self.superverbose {
            test_cmd.push_str(" -VV");
        }

        execute_cmd(&build_cmd)?;
        execute_cmd(&test_cmd)?;

        Ok(())
    }
}
