use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

const CACHEDIR_TAG_FILENAME: &'static str = "CACHEDIR.TAG";
const CACHEDIR_TAG_SIGNATURE: &'static [u8] = b"Signature: 8a477f597d28d172789f06886806bc55";

pub fn is_cache_dir(path: &Path) -> io::Result<bool> {
    let tag_path = path.join(CACHEDIR_TAG_FILENAME);

    if tag_path.is_file() {
        let mut file = File::open(tag_path)?;
        let mut buf = [0; CACHEDIR_TAG_SIGNATURE.len()];
        file.read_exact(&mut buf)?;
        if buf == CACHEDIR_TAG_SIGNATURE {
            return Ok(true);
        }
    }

    Ok(false)
}
