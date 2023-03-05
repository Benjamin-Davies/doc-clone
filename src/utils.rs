use std::path::PathBuf;

pub struct Warning {
    pub path: PathBuf,
    pub line: usize,
    pub content: String,
}

impl Warning {
    pub fn print(&self, error: bool) {
        println!(
            "::{} file={},line={}::{}",
            if error { "error" } else { "warning" },
            self.path.display(),
            self.line,
            self.content
        );
    }
}

#[cfg(test)]
pub mod tests {
    pub fn example_lines() -> Vec<String> {
        include_str!("../examples/example.txt")
            .lines()
            .map(String::from)
            .collect::<Vec<_>>()
    }
}
