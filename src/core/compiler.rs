use std::collections::HashMap;
use which::which;

pub fn detect_compilers() -> HashMap<String, String> {
    let options = ["clang", "clang++", "gcc", "g++"];

    options
        .iter()
        .filter_map(|&o| {
            which(o)
                .ok()
                .map(|p| (o.to_string(), p.to_string_lossy().into_owned()))
        })
        .collect()
}
