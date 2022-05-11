use std::{fs, path::PathBuf};

use clap::Parser as ClapParser;
use spec_tools::model_to_yaml;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    pub grammar_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let yaml_src = fs::read_to_string(args.grammar_file).expect("Failed to read file");
    let model = serde_yaml::from_str(&yaml_src).unwrap();
    model_to_yaml::generate(&model);
}
