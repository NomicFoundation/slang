use chumsky::prelude::*;
use config::Configuration;
use std::{fs, path::PathBuf};
use util::{print_errors, rustfmt};
use yaml_rust::YamlLoader;

mod config;
mod generator;
mod parser;
mod tree_builder;
mod util;

use generator::generate_all_parsers;
use parser::create_grammar_parser;

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

    let (ebnf, errs) = create_grammar_parser().parse_recovery(ebnf_src.as_str());

    print_errors(errs, &ebnf_src);

    if let Some(productions) = ebnf {
        let annotations_src =
            fs::read_to_string(args.annotations_file).expect("Failed to read file");
        let annotations =
            YamlLoader::load_from_str(&annotations_src).expect("Failed to parse annotations");
        let configuration = Configuration::from(annotations);
        println!(
            "{}",
            rustfmt(generate_all_parsers(&productions, &configuration).to_string()).unwrap()
        )
    }
}
