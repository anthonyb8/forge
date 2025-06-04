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
        let directories = ["include", "src", "build", "tests", ".vscode"];
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
        content.push("static void test_add(void **state) {");
        content.push("  (void)state;");
        content.push("  assert_int_equal(add(2, 3), 5);");
        content.push("  assert_int_equal(add(-1, 1), 0);");
        content.push("}\n");
        content.push("int main(void) {");
        content.push("  const struct CMUnitTest tests[] = {");
        content.push("      cmocka_unit_test(test_add),");
        content.push("  };\n");
        content.push("  return cmocka_run_group_tests(tests, NULL, NULL);");
        content.push("}");

        let path = self.path.join("tests").join("test_lib.c");
        match fs::write(path, content.join("\n")) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
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

    fn tests(&self) -> Result<()> {
        // this should be test specific
        let mut content = vec![];
        content.push("#include <gtest/gtest.h>");
        content.push("#include \"lib.hpp\"\n");
        content.push("TEST(GreetingTest, BasicTest) {");
        content.push("  EXPECT_EQ(get_greeting(\"Test\"), \"Hello, Test!\");");
        content.push("}\n");
        content.push("int main(int argc, char** argv) {");
        content.push("  ::testing::InitGoogleTest(&argc, argv);\n");
        content.push("  return RUN_ALL_TESTS();");
        content.push("}");

        let path = self.path.join("tests").join("test_lib.cpp");
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaffold() {
        let test_name = "dummy";
        let test_project = std::env::current_dir()
            .unwrap()
            .join("tests")
            .join(test_name);

        let scaffolder = Scaffolder::new(
            test_name.to_string(),
            test_project,
            // Language::Cpp(crate::core::language::CppStandard::Cpp11),
            Language::C(crate::core::language::CStandard::C89),
        );

        scaffolder.create_dir().expect("Error on create_dir");
        scaffolder
            .create_structure()
            .expect("Error on create_structure");
        scaffolder.git_init().expect("Error on git init");
        scaffolder.vscode_init().expect("Error on vscode init");
        scaffolder
            .create_hello_world()
            .expect("error on hello world");
    }
}
