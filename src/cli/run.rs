use super::execute_cmd;
use crate::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct RunArgs {
    #[arg(long)]
    pub release: bool,

    #[arg(long)]
    pub verbose: bool,
}

impl RunArgs {
    pub fn process_command(&self) -> Result<()> {
        let mut build = Run::new();

        if self.verbose {
            build = build.verbose();
        }

        if self.release {
            build = build.release();
        }

        build.build()?;

        Ok(())
    }
}

pub struct Run {
    release: bool,
    verbose: bool,
}

impl Run {
    pub fn new() -> Self {
        Run {
            release: false,
            verbose: false,
        }
    }

    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
    pub fn release(mut self) -> Self {
        self.release = true;
        self
    }

    pub fn build(&self) -> Result<()> {
        let build_type = if self.release { "Release" } else { "Debug" };

        let configure_cmd = format!(
            "cmake -DCMAKE_BUILD_TYPE={} -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -B build",
            build_type
        );

        let mut build_cmd = String::from("cmake --build ./build");
        if self.release {
            build_cmd.push_str(" --verbose");
        }

        execute_cmd(&configure_cmd)?;
        execute_cmd(&build_cmd)?;

        Ok(())
    }
}
