pub mod builder;
pub mod compiler;
pub mod package;

use crate::{error, execute_cmd, Error, Result};
use builder::cmake::CMakeBuilder;
use builder::BuildSystem;
use builder::BuildSystemEnum;
use builder::TestFrameworkEnum;
use compiler::{detect_compilers, LanguageStd};
use inquire::Select;
use package::{ConanManager, PackageManager, PackageManagerEnum, VcpkgManager};
use serde_json::json;
use std::fs;
use std::str::FromStr;
use strum::VariantNames;

pub struct Config {
    name: String,
    package_manager: Option<Box<dyn PackageManager>>,
    build_system: Option<Box<dyn BuildSystem>>,
    test_framework: TestFrameworkEnum,
    compiler_path: String,
    language: LanguageStd,
    intellisense_mode: String,
}

impl Config {
    pub fn new(name: &str) -> Result<Config> {
        Ok(Config {
            name: name.to_string(),
            package_manager: None,
            build_system: None,
            test_framework: TestFrameworkEnum::None,
            compiler_path: "".to_string(),
            language: LanguageStd::Cpp20,
            intellisense_mode: "holder".to_string(),
        })
    }

    pub fn create(&self) -> Result<()> {
        let cmd = format!("mkdir -p {}", self.name);
        Ok(execute_cmd(&cmd)?)
    }

    pub fn git_init(&self) -> Result<()> {
        let cmd = format!("cd {} && git init && touch .gitignore", self.name);
        execute_cmd(&cmd)?;

        let content = r#"# Ignore build output
/build/
/bin/

# Ignore CMake files
/CMakeFiles/
/CMakeCache.txt
/cmake_install.cmake

# Ignore vcpkg installation files
/vcpkg_installed/
/vcpkg/

# Ignore system files
*.DS_Store
*.swp
.cache

# Vcpkg Commands
compile_commands.json
vcpkg-configuration.json
vcpkg.json
        "#;
        fs::write(format!("{}/.gitignore", self.name), content.to_string())?;

        Ok(())
    }

    pub fn language_standard(mut self) -> Result<Self> {
        let choice = Select::new("Langauge:", LanguageStd::VARIANTS.to_vec())
            .prompt()?
            .to_string();

        self.language = LanguageStd::from_str(&choice)?;
        Ok(self)
    }

    pub fn compiler(mut self) -> Result<Self> {
        let compiler_map = detect_compilers();
        let choice = Select::new("Compiler:", compiler_map.keys().collect()).prompt()?;

        self.compiler_path = compiler_map.get(choice).unwrap().to_string();
        Ok(self)
    }

    pub fn test_framework(mut self) -> Result<Self> {
        let choice = Select::new("Test Framework", TestFrameworkEnum::VARIANTS.to_vec())
            .prompt()?
            .to_string();

        self.test_framework = TestFrameworkEnum::from_str(&choice)?;
        Ok(self)
    }

    pub fn build_system(mut self) -> Result<Self> {
        let choice = Select::new("Build System:", BuildSystemEnum::VARIANTS.to_vec()).prompt()?;

        self.build_system = Some(match choice {
            "CMake" => Box::new(
                CMakeBuilder::new(&self.name)
                    .language(&self.language.language())
                    .standard(&self.language.version())
                    .framework(&self.test_framework),
            ),
            // "conan" => Box::new(ConanManager::new(&self.name)?),
            _ => return Err(error!(CustomError, "Invalid Option")),
        });
        Ok(self)
    }

    pub fn package_manager(mut self) -> Result<Self> {
        let choice =
            Select::new("Package Manager:", PackageManagerEnum::VARIANTS.to_vec()).prompt()?;

        self.package_manager = Some(match choice {
            "Vcpkg" => Box::new(VcpkgManager::new(&self.name)?),
            "Conan" => Box::new(ConanManager::new(&self.name)?),
            _ => return Err(error!(CustomError, "Invalid Option")),
        });
        Ok(self)
    }

    pub fn vscode_init(&self) -> Result<()> {
        let cmd = format!("mkdir -p {}/.vscode", self.name);

        match execute_cmd(&cmd) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        let file = format!("{}/.vscode/c_cpp_properies.json", self.name);

        let content = json!(
        {
          "configurations": [
            {
              "name": "Mac",
              "includePath": [
                "${workspaceFolder}/lib",
                "${workspace}/vcpkg_installed/x64-osx/include"
              ],
              "defines": [],
              "macFrameworkPath": [],
              "compilerPath": self.compiler_path,
              "cStandard": "c11",
              "cppStandard": "c++20",
              "intelliSenseMode": self.intellisense_mode
            }
          ],
          "version": 4
        });

        fs::write(file, serde_json::to_string_pretty(&content)?)?;
        Ok(())
    }

    pub fn hello_world(&self) -> Result<()> {
        let directories = ["include", "src", "tests", "build"];
        for dir in directories {
            let cmd = format!("mkdir -p {}/{}", self.name, dir);
            match execute_cmd(&cmd) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        self.write_header()?;
        self.write_library()?;
        self.write_binary()?;
        self.write_tests()?;

        Ok(())
    }

    fn write_tests(&self) -> std::io::Result<()> {
        let file = format!("{}/tests/test_lib.cpp", self.name);
        let mut content = vec![];

        content.push("#include <gtest/gtest.h>\n");
        content.push("#include \"lib.h\"\n");
        content.push("TEST(GreetingTest, BasicTest) {");
        content.push("  EXPECT_EQ(get_greeting(\"Test\"), \"Hello, Test!\");");
        content.push("}\n");
        content.push("int main(int argc, char** argv) {");
        content.push("  ::testing::InitGoogleTest(&argc, argv);");
        content.push("  return RUN_ALL_TESTS();");
        content.push("}");

        fs::write(file, content.join("\n"))?;
        Ok(())
    }

    fn write_header(&self) -> std::io::Result<()> {
        let file = format!("{}/include/lib.h", self.name);
        let mut content = vec![];

        content.push("#pragma once\n");
        content.push("#include <string>\n");
        content.push("std::string get_greeting(const std::string& name);");

        fs::write(file, content.join("\n"))?;
        Ok(())
    }

    fn write_library(&self) -> std::io::Result<()> {
        let file = format!("{}/src/lib.cpp", self.name);
        let mut content = vec![];

        content.push("#include \"lib.h\"\n");
        content.push("  std::string get_greeting(const std::string& name) {");
        content.push("  return \"Hello, \" + name + \"!\";");
        content.push("}");

        fs::write(file, content.join("\n"))?;
        Ok(())
    }

    fn write_binary(&self) -> std::io::Result<()> {
        let file = format!("{}/src/main.cpp", self.name);
        let mut content = vec![];

        content.push("#include \"lib.h\"\n");
        content.push("#include <iostream>");
        content.push("int main(){");
        content.push("  std::string name = \"World\";");
        content.push("  std::cout << get_greeting(name) << std::endl;");
        content.push("  return 0;");
        content.push("}");

        fs::write(file, content.join("\n"))?;
        Ok(())
    }

    pub fn build(&self) -> Result<()> {
        self.create()?;
        self.git_init()?;
        self.vscode_init()?;
        self.hello_world()?;

        match &self.package_manager {
            Some(pm) => pm.init()?,
            None => return Err(error!(CustomError, "Package manager not configured")),
        }

        match &self.build_system {
            Some(bs) => {
                bs.init()?;
                bs.build()?;
            }
            None => return Err(error!(CustomError, "Build system not configured")),
        }

        Ok(())
    }
}
