use std::{fs, path::PathBuf};

use clap::Parser as ClapParser;
use spec_tools::{model::Grammar, model_to_chumsky, util::rustfmt};

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
        rustfmt(model_to_chumsky::generate(&model).to_string())
            .expect("Failed to format rust output")
    );
}
