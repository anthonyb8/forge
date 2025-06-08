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

#[cfg(test)]
mod tests {
    use super::*;
    use which::which;

    #[test]
    fn test_detect_compilers() {
        let options = ["clang", "clang++", "gcc", "g++"];
        let mut count: usize = 0;

        for o in options {
            match which(o) {
                Ok(_) => count += 1,
                Err(_) => (),
            }
        }

        // Test
        let compilers = detect_compilers();

        // Validate
        assert_eq!(compilers.len(), count);
    }
}
