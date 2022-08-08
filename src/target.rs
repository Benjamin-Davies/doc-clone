use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

pub fn substitute(
    path: &Path,
    sources: &HashMap<String, Vec<String>>,
    in_place: bool,
) -> io::Result<()> {
    let mut input = String::new();
    let path = std::env::current_dir()?.join(path);
    File::open(&path)?.read_to_string(&mut input)?;
    let mut output = String::with_capacity(input.len());

    const DOC_CLONE_ATTR: &'static str = "@doc-clone:";

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
                output.push_str(&source.join("\n * "));
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
