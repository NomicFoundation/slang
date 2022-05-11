use std::{fs, path::PathBuf};

use chumsky::Parser;
use spec_tools::model_from_ebnf::create_grammar_parser;
use spec_tools::util::print_errors;

use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    pub grammar_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let ebnf_src = fs::read_to_string(args.grammar_file).expect("Failed to read file");
    let (model, errs) = create_grammar_parser().parse_recovery(ebnf_src.as_str());
    print_errors(errs, &ebnf_src);
    if let Some(model) = model {
        println!(
            "{}",
            serde_yaml::to_string(&model).expect("Failed to write model")
        );
    }
}
