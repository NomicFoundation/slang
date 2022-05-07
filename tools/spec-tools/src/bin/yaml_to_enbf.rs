use std::path::PathBuf;

use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    pub grammar_file: PathBuf,
    pub annotations_file: PathBuf,
}

fn main() {
    let args = Args::parse();
}
