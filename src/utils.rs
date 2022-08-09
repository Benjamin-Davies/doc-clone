#[cfg(test)]
pub mod tests {
    pub fn example_lines() -> Vec<String> {
        include_str!("../examples/example.txt")
            .lines()
            .map(String::from)
            .collect::<Vec<_>>()
    }
}
