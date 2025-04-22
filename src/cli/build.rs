use super::execute_cmd;
use crate::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct BuildArgs {
    /// Build in release mode.
    #[arg(long)]
    pub release: bool,

    /// Use verbose output.
    #[arg(long)]
    pub verbose: bool,
}

impl BuildArgs {
    pub fn process_command(&self) -> Result<()> {
        let mut build = Build::new();

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

pub struct Build {
    release: bool,
    verbose: bool,
}

impl Build {
    pub fn new() -> Self {
        Build {
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
