# 🔨 Forge

**Forge** is a tool that scaffolds modern C and C++ projects.  
It helps you set up a project by selecting a:

- Compiler
- Build system
- Package manager
- Test framework

Then it wires everything together into a clean, ready-to-use structure.

Forge isn't a build tool or a package manager—it's a setup assistant  
that gets you from zero to ready-to-code in seconds.

## ✨ Features

- Interactive CLI setup
- Support for multiple toolchain configurations
- CMake + vcpkg + GoogleTest defaults
- Git project initialization

## 🔧 Install

- To install or update run the below command in your terminal.

```bash
curl -sSfL https://raw.githubusercontent.com/anthonyb8/forge/main/scripts/install.sh | bash
```

## :notebook: Commands

#### Create Project

** Creates directory & set-up **

```bash
forge new <name>
```

** Create set-up in current directory **

```bash

forge init <name>
```

#### Build

```bash
forge build [ --release | --verbose ]
```

#### Run Executable

```bash
forge run
```

#### Run Tests

```sh
forge test [ --verbose | --superverbose ]
```

#### Clean Build Artifacts

```bash
forge clean
```

#### Help

```sh
forge [ help | --help | -h ]
```

## 📦 Status

Early development — contributions welcome!
