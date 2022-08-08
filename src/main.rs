use std::{borrow::Cow, env::current_dir, path::PathBuf};

use clap::Parser;

mod source;

#[derive(Parser, Debug)]
// TODO: author, version, etc.
struct Args {
    #[clap(short, long)]
    source_path: Vec<PathBuf>,

    #[clap(short, long)]
    in_place: bool,

    target_files: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let source_paths = if args.source_path.len() > 0 {
        Cow::Borrowed(&args.source_path)
    } else {
        Cow::Owned(vec![current_dir().unwrap()])
    };
    let sources = source::scan(source_paths.as_ref());

    println!("{:?}", sources);
}
