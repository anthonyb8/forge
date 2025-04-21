use std::collections::HashMap;

use strum_macros::EnumString;
use which::which;

#[derive(Debug, EnumString, strum_macros::VariantNames)]
pub enum LanguageStd {
    Cpp11,
    Cpp14,
    Cpp17,
    Cpp20,
    Cpp23,
    C89,
    C99,
    C11,
}

impl LanguageStd {
    pub fn language(&self) -> &str {
        match self {
            LanguageStd::Cpp11 => "CXX",
            LanguageStd::Cpp14 => "CXX",
            LanguageStd::Cpp17 => "CXX",
            LanguageStd::Cpp20 => "CXX",
            LanguageStd::Cpp23 => "CXX",
            LanguageStd::C89 => "C",
            LanguageStd::C99 => "C",
            LanguageStd::C11 => "C",
        }
    }

    pub fn version(&self) -> &str {
        match self {
            LanguageStd::Cpp11 => "11",
            LanguageStd::Cpp14 => "14",
            LanguageStd::Cpp17 => "17",
            LanguageStd::Cpp20 => "20",
            LanguageStd::Cpp23 => "23",
            LanguageStd::C89 => "89",
            LanguageStd::C99 => "99",
            LanguageStd::C11 => "11",
        }
    }
}

pub fn detect_compilers() -> HashMap<String, String> {
    let options = ["clangd", "clang", "clang++", "g++", "gcc"];

    options
        .iter()
        .filter_map(|&o| {
            which(o)
                .ok()
                .map(|p| (o.to_string(), p.to_string_lossy().into_owned()))
        })
        .collect()
}
