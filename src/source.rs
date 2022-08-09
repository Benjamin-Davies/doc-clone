use std::{
    collections::HashMap,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

use crate::{
    constants::{DOC_CLONE_SOURCE_ATTR, RUSTDOC_COMMENT_PREFIX},
    helpers::is_cache_dir,
};

pub fn scan(paths: &[PathBuf]) -> io::Result<HashMap<String, Vec<String>>> {
    let mut sources = HashMap::new();

    for path in paths {
        traverse(&path, &mut sources)?;
    }

    Ok(sources)
}

fn traverse(path: &Path, sources: &mut HashMap<String, Vec<String>>) -> io::Result<()> {
    let mut stack = vec![path.to_owned()];

    while let Some(path) = stack.pop() {
        if is_hidden_file(&path) {
            continue;
        }

        let metadata = path.metadata()?;
        if metadata.is_file() {
            if path.extension() == Some(OsStr::new("rs")) {
                scan_file(&path, sources)?;
            }
        } else if metadata.is_dir() && !is_cache_dir(&path)? {
            for child in fs::read_dir(path)? {
                stack.push(child?.path());
            }
        }
    }

    Ok(())
}

fn scan_file(path: &Path, sources: &mut HashMap<String, Vec<String>>) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;

    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        if let Some(key) = parse_source_attr(line) {
            scan_comment(key, &mut lines, sources);
        }
    }

    Ok(())
}

fn scan_comment<'a>(
    key: &str,
    mut lines: impl Iterator<Item = &'a str>,
    sources: &mut HashMap<String, Vec<String>>,
) {
    let mut docs = Vec::new();
    while let Some(line) = lines.next().and_then(parse_doc_comment) {
        docs.push(line.to_owned());
    }
    sources.insert(key.to_owned(), docs);
}

fn parse_source_attr(line: &str) -> Option<&str> {
    parse_doc_comment(line).and_then(|line| {
        line.find(DOC_CLONE_SOURCE_ATTR).and_then(|index| {
            line[index + DOC_CLONE_SOURCE_ATTR.len()..]
                .split_ascii_whitespace()
                .next()
        })
    })
}

fn parse_doc_comment(line: &str) -> Option<&str> {
    let line = line.trim_start();
    if line.starts_with(RUSTDOC_COMMENT_PREFIX) {
        Some(line[RUSTDOC_COMMENT_PREFIX.len()..].trim())
    } else {
        None
    }
}

fn is_hidden_file(path: &PathBuf) -> bool {
    if let Some(s) = path.file_name() {
        s.to_string_lossy().starts_with(".")
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tests::example_lines;

    use super::scan;

    #[test]
    fn scan_example() {
        let sources = scan(&["examples".into()]).unwrap();

        assert_eq!(sources.get("foo"), Some(&example_lines()));
    }

    #[test]
    fn scan_parent_dir() {
        let sources = scan(&["examples/dir/..".into()]).unwrap();

        assert_eq!(sources.get("foo"), Some(&example_lines()));
    }
}
