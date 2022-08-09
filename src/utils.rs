use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub fn read_to_string(path: impl AsRef<Path>) -> io::Result<String> {
    let mut input = String::new();
    File::open(&path)?.read_to_string(&mut input)?;
    Ok(input)
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
