use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Language {
    C(CStandard),
    Cpp(CppStandard),
}

impl Language {
    pub fn variants() -> Vec<&'static str> {
        [CStandard::variants(), CppStandard::variants()].concat()
    }
    pub fn from_str(s: &str) -> Language {
        if CppStandard::variants().contains(&s) {
            Language::Cpp(CppStandard::from_str(s))
        } else {
            Language::C(CStandard::from_str(s))
        }
    }

    pub fn version(&self) -> &'static str {
        match self {
            Language::C(std) => std.version(),
            Language::Cpp(std) => std.version(),
        }
    }

    pub fn src_suffix(&self) -> &str {
        match self {
            Language::C(_) => "c",
            Language::Cpp(_) => "cpp",
        }
    }

    pub fn header_suffix(&self) -> &str {
        match self {
            Language::C(_) => "h",
            Language::Cpp(_) => "hpp",
        }
    }

    pub fn cmake_identifier(&self) -> &str {
        match self {
            Language::C(_) => "C",
            Language::Cpp(_) => "CXX",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CStandard {
    C89,
    C99,
    C11,
}

impl CStandard {
    pub fn variants() -> Vec<&'static str> {
        vec!["C89", "C99", "C11"]
    }

    pub fn version(&self) -> &'static str {
        match self {
            CStandard::C89 => "89",
            CStandard::C99 => "99",
            CStandard::C11 => "11",
        }
    }

    pub fn from_str(s: &str) -> CStandard {
        match s {
            "C89" => CStandard::C89,
            "C99" => CStandard::C99,
            "C11" => CStandard::C11,
            _ => CStandard::C89,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CppStandard {
    Cpp11,
    Cpp14,
    Cpp17,
    Cpp20,
    Cpp23,
}

impl CppStandard {
    pub fn variants() -> Vec<&'static str> {
        vec!["Cpp11", "Cpp14", "Cpp17", "Cpp20", "Cpp23"]
    }

    pub fn version(&self) -> &'static str {
        match self {
            CppStandard::Cpp11 => "11",
            CppStandard::Cpp14 => "14",
            CppStandard::Cpp17 => "17",
            CppStandard::Cpp20 => "20",
            CppStandard::Cpp23 => "23",
        }
    }
    pub fn from_str(s: &str) -> CppStandard {
        match s {
            "Cpp11" => CppStandard::Cpp11,
            "Cpp14" => CppStandard::Cpp14,
            "Cpp17" => CppStandard::Cpp17,
            "Cpp20" => CppStandard::Cpp20,
            "Cpp23" => CppStandard::Cpp23,
            _ => CppStandard::Cpp11,
        }
    }
}
