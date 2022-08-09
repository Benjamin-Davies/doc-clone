use std::{borrow::Cow, env::current_dir, path::PathBuf};

use clap::Parser;

mod constants;
mod helpers;
mod source;
mod target;
mod utils;

#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Paths to be scanned for files with @doc-clone-source attributes. Defaults to the current directory.
    #[clap(short, long)]
    source_path: Vec<PathBuf>,

    /// Substitute the target attributes in-place. If absent, then the output will be printed to stdout.
    #[clap(short, long)]
    in_place: bool,

    /// Files containing the @doc-clone attributes that are to be substituted.
    target_files: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let source_paths = if args.source_path.len() > 0 {
        Cow::Borrowed(&args.source_path)
    } else {
        Cow::Owned(vec![current_dir().unwrap()])
    };
    let sources = source::scan(source_paths.as_ref()).unwrap();

    for target_file in args.target_files {
        target::substitute(&target_file, &sources, args.in_place).unwrap();
    }
}
