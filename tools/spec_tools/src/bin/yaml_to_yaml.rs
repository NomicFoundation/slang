use std::{fs, path::PathBuf};

use clap::Parser as ClapParser;
use spec_tools::model::Grammar;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    pub grammar_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let yaml_src = fs::read_to_string(args.grammar_file).expect("Failed to read file");
    let model: Grammar = serde_yaml::from_str(&yaml_src).expect("Failed to parse model");
    println!(
        "{}",
        serde_yaml::to_string(&model).expect("Failed to write model")
    );
}
