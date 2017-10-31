pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Vec<String> {
        self.source
            .split_whitespace()
            .map(|s| String::from(s))
            .collect()
    }
}
