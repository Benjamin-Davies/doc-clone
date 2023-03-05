use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use crate::{
    constants::{DOC_CLONE_ATTR, JAVADOC_COMMENT_LINE_DELIMETER},
    utils::Warning,
};

pub fn substitute(
    path: &Path,
    sources: &HashMap<String, (PathBuf, usize, Vec<String>)>,
    used_sources: &mut HashSet<String>,
    in_place: bool,
) -> io::Result<Vec<Warning>> {
    let path = std::env::current_dir()?.join(path);
    let input = fs::read_to_string(&path)?;
    let mut line = 1;
    let mut output = String::with_capacity(input.len());
    let mut warnings = Vec::new();

    let mut cursor = 0;
    while let Some(attr_offset) = input[cursor..].find(DOC_CLONE_ATTR) {
        let attr_index = cursor + attr_offset;
        output.push_str(&input[cursor..attr_index]);
        line += input[cursor..attr_index]
            .chars()
            .filter(|&c| c == '\n')
            .count();

        if let Some(key) = input[attr_index + DOC_CLONE_ATTR.len()..]
            .split_ascii_whitespace()
            .next()
        {
            let length = DOC_CLONE_ATTR.len() + key.len();

            if let Some((_, _, source)) = sources.get(key) {
                output.push_str(&source.join(JAVADOC_COMMENT_LINE_DELIMETER));
                used_sources.insert(key.to_owned());
            } else {
                warnings.push(Warning {
                    path: path.clone(),
                    line,
                    content: format!("Undefined key: {key}"),
                })
            }

            cursor = attr_index + length;
        } else {
            break;
        }
    }
    output.push_str(&input[cursor..]);

    if in_place {
        File::create(path)?.write_all(output.as_bytes())?;
    } else {
        print!("{}", output);
    }

    Ok(warnings)
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        fs,
    };

    use crate::utils::tests::example_lines;

    use super::substitute;

    #[test]
    fn substitute_example() {
        let mut sources = HashMap::new();
        sources.insert("foo".to_owned(), ("".into(), 0, example_lines()));

        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("test.c");
        fs::copy("examples/input.c", &path).unwrap();

        substitute(&path, &sources, &mut HashSet::new(), true).unwrap();

        assert_eq!(
            fs::read_to_string(path).unwrap(),
            include_str!("../examples/expected.c")
        );
    }

    #[test]
    fn record_usages() {
        let mut sources = HashMap::new();
        sources.insert("foo".to_owned(), ("".into(), 0, example_lines()));
        let mut used_sources = HashSet::new();

        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("test.c");
        fs::copy("examples/input.c", &path).unwrap();

        substitute(&path, &sources, &mut used_sources, true).unwrap();

        assert_eq!(
            fs::read_to_string(path).unwrap(),
            include_str!("../examples/expected.c")
        );
        assert!(used_sources.contains("foo"));
    }
}
