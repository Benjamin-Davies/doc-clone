use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, File},
    io::{self, Read},
    path::{Path, PathBuf},
};

pub fn scan(paths: &[PathBuf]) -> io::Result<HashMap<String, Vec<String>>> {
    let mut sources = HashMap::new();

    for path in paths {
        traverse(&path, &mut sources)?;
    }

    Ok(sources)
}

// TODO: should this be using recursion?
fn traverse(path: &Path, sources: &mut HashMap<String, Vec<String>>) -> io::Result<()> {
    let metadata = path.metadata()?;
    if metadata.is_file() {
        if path.extension() == Some(OsStr::new("rs")) {
            scan_file(path, sources)?;
        }
    } else if metadata.is_dir() && !is_cache_dir(path)? {
        for child in fs::read_dir(path)? {
            traverse(&child?.path(), sources)?;
        }
    }
    Ok(())
}

fn is_cache_dir(path: &Path) -> io::Result<bool> {
    const SIGNATURE: &'static [u8] = b"Signature: 8a477f597d28d172789f06886806bc55";
    let tag_path = path.join("CACHEDIR.TAG");
    if tag_path.is_file() {
        let mut file = File::open(tag_path)?;
        let mut buf = [0; SIGNATURE.len()];
        file.read(&mut buf)?;
        if buf == SIGNATURE {
            return Ok(true);
        }
    }
    Ok(false)
}

fn scan_file(path: &Path, sources: &mut HashMap<String, Vec<String>>) -> io::Result<()> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;

    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        const DOC_CLONE_SOURCE_ATTR: &'static str = "@doc-clone-source:";
        // TODO: extract into parse_clone_source_attr
        if let Some(line) = parse_doc_comment(line) {
            if let Some(index) = line.find(DOC_CLONE_SOURCE_ATTR) {
                if let Some(key) = line[index + DOC_CLONE_SOURCE_ATTR.len()..]
                    .split_ascii_whitespace()
                    .next()
                {
                    scan_comment(key, &mut lines, sources);
                }
            }
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

/// @doc-clone-source:foo
/// Some
/// More
/// Lines
fn parse_doc_comment(line: &str) -> Option<&str> {
    const PREFIX: &'static str = "///";
    let line = line.trim_start();
    if line.starts_with(PREFIX) {
        Some(line[PREFIX.len()..].trim())
    } else {
        None
    }
}
