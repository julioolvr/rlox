extern crate rlox;

pub fn execute(code: &str) -> Vec<String> {
    rlox::run_string(code.to_string())
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect()
}
