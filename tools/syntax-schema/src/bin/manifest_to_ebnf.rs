use std::path::PathBuf;

use clap::Parser;

use syntax_generator::{generators, schema, validation};

#[derive(Parser, Debug)]
struct ProgramArgs {
    #[clap(long)]
    manifest_path: String,

    #[clap(long)]
    ebnf_output: String,
}

fn main() {
    let args = ProgramArgs::parse();

    println!(" => Loading Manifest");
    let grammar = schema::load_grammar(&PathBuf::from(args.manifest_path));

    println!(" => Validating Grammar");
    validation::validate(&grammar);

    println!(" => Generating EBNF");
    generators::ebnf::generate(&grammar, &PathBuf::from(args.ebnf_output));
}
