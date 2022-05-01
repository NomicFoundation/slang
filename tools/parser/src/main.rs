use chumsky::prelude::*;
use std::{fs, path::PathBuf};
use util::print_errors;
use yaml_rust::YamlLoader;

mod generator;
mod parser;
mod tree_builder;
mod util;

use generator::generate;

use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    pub grammar_file: PathBuf,
    pub annotations_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let ebnf_src = fs::read_to_string(args.grammar_file).expect("Failed to read file");

    let (productions, errs) = parser::create_grammar_parser().parse_recovery(ebnf_src.as_str());

    if let Some(productions) = productions {
        let yaml_src = fs::read_to_string(args.annotations_file).expect("Failed to read file");
        let annotations =
            &YamlLoader::load_from_str(&yaml_src).expect("Failed to parse annotations")[0];
        println!(
            "{}",
            generate(
                productions,
                "expression".to_owned(),
                &annotations["productions"]
            )
            .unwrap()
        );
    }

    print_errors(errs, &ebnf_src);
}
