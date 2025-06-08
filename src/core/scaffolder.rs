use super::language::Language;
use crate::Result;
use serde_json::json;
use std::process::Command;
use std::{fs, path::PathBuf};

pub struct Scaffolder {
    name: String,
    project_dir: PathBuf,
    language: Language,
}

impl Scaffolder {
    pub fn new(name: String, project_dir: PathBuf, language: Language) -> Scaffolder {
        Scaffolder {
            name,
            project_dir,
            language,
        }
    }

    pub fn create_dir(&self) -> Result<()> {
        fs::create_dir_all(&self.project_dir)?;
        Ok(())
    }

    pub fn create_structure(&self) -> Result<()> {
        let directories = ["include", "src", "build", "test", "libs", ".vscode"];
        for d in directories {
            let path = self.project_dir.join(d);
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    pub fn git_init(&self) -> Result<()> {
        let _status = Command::new("git")
            .arg("init")
            .current_dir(&self.project_dir)
            .status()?;

        let mut content = vec![];
        content.push("# Ignore build output");
        content.push("/build/");
        content.push("/bin/\n");
        content.push("# Ignore CMake files");
        content.push("/CMakeFiles/");
        content.push("/CMakeCache.txt");
        content.push("/cmake_install.cmake\n");
        content.push("# Ignore vcpkg installation files");
        content.push("/vcpkg_installed/");
        content.push("/vcpkg/\n");
        content.push("# Ignore system files");
        content.push("*.DS_Store");
        content.push("*.swp");
        content.push(".cache\n");
        content.push("# Vcpkg Commands");
        content.push("compile_commands.json");
        content.push("vcpkg-configuration.json");
        content.push("vcpkg.json\n");

        let path = self.project_dir.join(".gitignore");
        fs::write(path, content.join("\n"))?;
        Ok(())
    }

    pub fn vscode_init(&self) -> Result<()> {
        let content = json!(
        {
          "configurations": [
            {
              "name": self.name,
              "includePath": [
                "${workspaceFolder}/lib",
                "${workspace}/vcpkg_installed/x64-osx/include"
              ],
              "defines": [],
              "macFrameworkPath": [],
              // "compilerPath": self.compiler_path,
              "cStandard": "c11",
              "cppStandard": "c++20",
              // "intelliSenseMode": self.intellisense_mode
            }
          ],
          "version": 4
        });

        let path = self
            .project_dir
            .join(".vscode")
            .join("c_cpp_properties.json");
        fs::write(path, serde_json::to_string_pretty(&content)?)?;
        Ok(())
    }

    pub fn create_hello_world(&self) -> Result<()> {
        match self.language {
            Language::C(_) => {
                let x = CHelloWorld {
                    // name: self.name.clone(),
                    path: self.project_dir.clone(),
                };
                x.build()?;
            }
            Language::Cpp(_) => {
                let x = CppHelloWorld {
                    // name: self.name.clone(),
                    path: self.project_dir.clone(),
                };
                x.build()?;
            }
        }
        Ok(())
    }

    pub fn build(&self) -> Result<()> {
        self.create_dir()?;
        self.create_structure()?;
        self.git_init()?;
        self.vscode_init()?;
        self.create_hello_world()?;
        Ok(())
    }
}

struct CHelloWorld {
    // name: String,
    path: PathBuf,
}

impl CHelloWorld {
    fn build(&self) -> Result<()> {
        self.header()?;
        self.lib()?;
        self.bin()?;
        self.tests()?;
        Ok(())
    }

    fn bin(&self) -> Result<()> {
        let mut content = vec![];
        content.push("#include <stdio.h>");
        content.push("#include \"lib.h\"\n");
        content.push("int main() {");
        content.push("  char str[11] = \"Hello World\";");
        content.push("  get_greeting(str);\n");
        content.push("  return 0;");
        content.push("}");

        let path = self.path.join("src").join("main.c");
        match fs::write(path, content.join("\n")) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn lib(&self) -> Result<()> {
        let mut content = vec![];
        content.push("#include \"lib.h\"\n");
        content.push("void get_greeting(const char* name) {");
        content.push("  while (*name != '\\0') {");
        content.push("    printf(\"%c\", *name);");
        content.push("    name++;");
        content.push("  }");
        content.push("}\n");
        content.push("int add(int a, int b) { return a + b; }");

        let path = self.path.join("src").join("lib.c");
        match fs::write(path, content.join("\n")) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn header(&self) -> Result<()> {
        let mut content = vec![];
        content.push("#pragma once\n");
        content.push("#include <stdio.h>\n");
        content.push("void get_greeting(const char* name);\n");
        content.push("int add(int a, int b);\n");

        let path = self.path.join("include").join("lib.h");
        match fs::write(path, content.join("\n")) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn tests(&self) -> Result<()> {
        let mut content = vec![];
        content.push("#include <setjmp.h>");
        content.push("#include <stdarg.h>");
        content.push("#include <stddef.h>");
        content.push("#include <stdint.h>");
        content.push("// Third party");
        content.push("#include <cmocka.h>");
        content.push("#include \"lib.h\"\n");

        // test_main.c
        let mut main_content = content.clone();
        main_content.push("extern const struct CMUnitTest libTests[];");
        main_content.push("extern const size_t libTestsSize;\n");
        main_content.push("int main(void) {");
        main_content.push("  int failures = 0;");
        main_content.push("  failures += _cmocka_run_group_tests(\"lib_tests\", libTests, libTestsSize, NULL, NULL);");
        main_content.push("  return failures;");
        main_content.push("}");

        // test_lib.c
        content.push("static void test_add(void **state) {");
        content.push("  (void)state;");
        content.push("  assert_int_equal(add(2, 3), 5);");
        content.push("  assert_int_equal(add(-1, 1), 0);");
        content.push("}\n");
        content.push("const struct CMUnitTest libTests[] = {cmocka_unit_test(test_add)};");
        content.push("const size_t libTestsSize = sizeof(libTests) / sizeof(libTests[0]);");

        let path_main = self.path.join("test").join("test_main.c");
        fs::write(path_main, main_content.join("\n"))?;

        let path_sub = self.path.join("test").join("test_lib.c");
        fs::write(path_sub, content.join("\n"))?;

        Ok(())
    }
}

struct CppHelloWorld {
    // name: String,
    path: PathBuf,
}

impl CppHelloWorld {
    fn build(&self) -> Result<()> {
        self.header()?;
        self.lib()?;
        self.bin()?;
        self.tests()?;
        Ok(())
    }

    fn bin(&self) -> Result<()> {
        let mut content = vec![];
        content.push("#include <iostream>");
        content.push("#include \"lib.hpp\"\n");
        content.push("int main(){");
        content.push("  std::string name = \"World\";");
        content.push("  std::cout << get_greeting(name) << std::endl;\n");
        content.push("  return 0;");
        content.push("}");

        let path = self.path.join("src").join("main.cpp");
        match fs::write(path, content.join("\n")) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn lib(&self) -> Result<()> {
        let mut content = vec![];
        content.push("#include \"lib.hpp\"\n");
        content.push("std::string get_greeting(const std::string& name) {");
        content.push("  return \"Hello, \" + name + \"!\";");
        content.push("}");

        let path = self.path.join("src").join("lib.cpp");
        match fs::write(path, content.join("\n")) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn header(&self) -> Result<()> {
        let mut content = vec![];
        content.push("#pragma once\n");
        content.push("#include <string>\n");
        content.push("std::string get_greeting(const std::string& name);");

        let path = self.path.join("include").join("lib.hpp");
        match fs::write(path, content.join("\n")) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    fn tests(&self) -> Result<()> {
        // this should be test specific
        let mut content = vec![];
        content.push("#include <gtest/gtest.h>\n");

        // test_main.cpp
        let mut main_content = content.clone();
        main_content.push("int main(int argc, char** argv) {");
        main_content.push("  ::testing::InitGoogleTest(&argc, argv);\n");
        main_content.push("  return RUN_ALL_TESTS();");
        main_content.push("}");

        // test_lib.cpp
        content.push("#include \"lib.hpp\"\n");
        content.push("TEST(GreetingTest, BasicTest) {");
        content.push("  EXPECT_EQ(get_greeting(\"Test\"), \"Hello, Test!\");");
        content.push("}\n");

        let path_main = self.path.join("test").join("test_main.cpp");
        fs::write(path_main, main_content.join("\n"))?;

        let path = self.path.join("test").join("test_lib.cpp");
        fs::write(path, content.join("\n"))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::language::{CStandard, CppStandard};
    use serial_test::serial;
    use std::{
        env,
        fs::{self},
    };
    use std::{thread, time::Duration};

    // Utility functions
    fn create_dummy_project(path: &PathBuf) -> anyhow::Result<()> {
        fs::create_dir_all(path)?;
        thread::sleep(Duration::from_millis(10));
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
    fn test_scaffold_c() -> anyhow::Result<()> {
        // Set-up
        let name = "dummy";
        let cwd = env::current_dir()?;
        let path = cwd.join(&name);
        create_dummy_project(&path)?;
        let language = Language::C(CStandard::C89);

        // Test
        let scaffolder = Scaffolder::new(name.to_string(), path.clone(), language);
        scaffolder.build()?;

        // Validate
        assert!(check_file_exits(&path.join(".gitignore")));
        assert!(check_file_exits(&path.join(".git")));
        assert!(check_file_exits(&path.join("include").join("lib.h")));
        assert!(check_file_exits(&path.join("src").join("lib.c")));
        assert!(check_file_exits(&path.join("src").join("main.c")));
        assert!(check_file_exits(&path.join("test").join("test_lib.c")));
        assert!(check_file_exits(&path.join("test").join("test_main.c")));
        assert!(check_file_exits(
            &path.join(".vscode").join("c_cpp_properties.json")
        ));

        // Clean-un
        delete_dummy_project(&path)?;

        Ok(())
    }

    #[test]
    #[serial]
    // #[ignore]
    fn test_scaffold_cpp() -> anyhow::Result<()> {
        // Set-up
        let name = "dummy";
        let cwd = env::current_dir()?;
        let path = cwd.join(&name);
        create_dummy_project(&path)?;
        let language = Language::Cpp(CppStandard::Cpp14);

        // Test
        let scaffolder = Scaffolder::new(name.to_string(), path.clone(), language);
        scaffolder.build()?;

        // Validate
        assert!(check_file_exits(&path.join(".gitignore")));
        assert!(check_file_exits(&path.join(".git")));
        assert!(check_file_exits(&path.join("include").join("lib.hpp")));
        assert!(check_file_exits(&path.join("src").join("lib.cpp")));
        assert!(check_file_exits(&path.join("src").join("main.cpp")));
        assert!(check_file_exits(&path.join("test").join("test_lib.cpp")));
        assert!(check_file_exits(&path.join("test").join("test_main.cpp")));
        assert!(check_file_exits(
            &path.join(".vscode").join("c_cpp_properties.json")
        ));

        // Clean-up
        delete_dummy_project(&path)?;

        Ok(())
    }
}
