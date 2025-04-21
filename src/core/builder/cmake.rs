use super::{BuildSystem, TestFrameworkEnum};
use crate::{execute_cmd, Result};
use std::path::PathBuf;

pub struct CMakeBuilder {
    pub name: String,
    pub path: PathBuf,
    pub version: String,
    pub language: String,
    pub standard: String,
    pub test_framework: TestFrameworkEnum,
}

impl CMakeBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            path: PathBuf::from(format!("{}/CMakeLists.txt", name)),
            version: "1.0".to_string(),
            language: "CXX".to_string(),
            standard: "20".to_string(),
            test_framework: TestFrameworkEnum::None,
        }
    }

    pub fn language(mut self, lang: &str) -> Self {
        self.language = lang.to_string();
        self
    }

    pub fn standard(mut self, std: &str) -> Self {
        self.standard = std.to_string();
        self
    }

    pub fn framework(mut self, framework: &TestFrameworkEnum) -> Self {
        self.test_framework = *framework;
        self
    }
}

impl BuildSystem for CMakeBuilder {
    fn google_test(&self) -> String {
        let mut content = vec![];

        content.push("enable_testing()".into());
        content.push("find_package(GTest REQUIRED)\n".into());
        content.push(format!(
            "add_executable({}Tests tests/test_lib.cpp)",
            self.name
        ));
        content.push(format!(
            "target_link_libraries({}Tests PRIVATE
                GTest::gtest
                GTest::gtest_main
                {}Lib
            )",
            self.name, self.name
        ));
        content.push(format!(
            "target_include_directories({}Tests PRIVATE 
                include/ 
                src/
            )",
            self.name
        ));
        content.push(format!(
            "add_test(NAME {}Tests COMMAND {}Tests)",
            self.name, self.name
        ));

        content.join("\n")
    }

    fn boost_test(&self) -> String {
        let mut content: Vec<String> = vec![];

        content.push("enable_testing()".into());
        content.push("find_package(GTest REQUIRED)\n".into());

        content.join("\n")
    }

    fn unit_test(&self) -> String {
        let mut content: Vec<String> = vec![];

        content.push("enable_testing()".into());
        content.push("find_package(GTest REQUIRED)\n".into());

        content.join("\n")
    }

    fn init(&self) -> Result<()> {
        let mut cmake = vec![];

        cmake.push("# General".into());
        cmake.push("cmake_minimum_required(VERSION 3.14)".into());
        cmake.push(format!(
            "project({} VERSION {} LANGUAGES {})",
            self.name, self.version, self.language
        ));

        cmake.push(format!(
            "set(CMAKE_{}_STANDARD {})",
            self.language, self.standard
        ));
        cmake.push(format!(
            "set(CMAKE_{}_STANDARD_REQUIRED ON)\n",
            self.language
        ));

        cmake.push("# Library".into());
        cmake.push(format!("add_library({}Lib src/lib.cpp)", self.name));
        cmake.push(format!(
            "target_include_directories({}Lib PUBLIC include/ lib/)\n",
            self.name
        ));

        cmake.push("# Binary".into());
        cmake.push(format!("add_executable({} src/main.cpp)", self.name));

        cmake.push(format!(
            "target_link_libraries({} PRIVATE {}Lib)\n",
            self.name, self.name
        ));

        cmake.push("# Testing".into());
        cmake.push(self.test_init(&self.test_framework));

        let content = cmake.join("\n");
        std::fs::write(&self.path, content)?;

        Ok(())
    }

    fn build(&self) -> Result<()> {
        let cmds = [
            format!(
                "cd {} && cmake -B build -DCMAKE_EXPORT_COMPILE_COMMANDS=ON",
                self.name
            ),
            format!(
                "cd {} && ln -sf build/compile_commands.json compile_commands.json",
                self.name
            ),
        ];
        for cmd in cmds {
            match execute_cmd(&cmd) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}
