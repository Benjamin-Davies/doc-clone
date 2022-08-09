use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use crate::constants::{DOC_CLONE_ATTR, JAVADOC_COMMENT_LINE_DELIMETER};

pub fn substitute(
    path: &Path,
    sources: &HashMap<String, Vec<String>>,
    in_place: bool,
) -> io::Result<()> {
    let path = std::env::current_dir()?.join(path);
    let input = fs::read_to_string(&path)?;
    let mut output = String::with_capacity(input.len());

    let mut cursor = 0;
    while let Some(attr_offset) = input[cursor..].find(DOC_CLONE_ATTR) {
        let attr_index = cursor + attr_offset;
        output.push_str(&input[cursor..attr_index]);

        if let Some(key) = input[attr_index + DOC_CLONE_ATTR.len()..]
            .split_ascii_whitespace()
            .next()
        {
            let length = DOC_CLONE_ATTR.len() + key.len();

            if let Some(source) = sources.get(key) {
                output.push_str(&source.join(JAVADOC_COMMENT_LINE_DELIMETER));
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

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use crate::utils::tests::example_lines;

    use super::substitute;

    #[test]
    fn substitute_example() {
        let mut sources = HashMap::new();
        sources.insert("foo".to_owned(), example_lines());

        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("test.c");
        fs::copy("examples/input.c", &path).unwrap();

        substitute(&path, &sources, true).unwrap();

        assert_eq!(
            fs::read_to_string(path).unwrap(),
            include_str!("../examples/expected.c")
        );
    }
}
