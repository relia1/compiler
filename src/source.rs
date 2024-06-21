// source
use std::fs;

pub struct Source {
    pub file_contents: String,
}

impl Source {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_contents: fs::read_to_string(file_path).unwrap(),
        }
    }
}

