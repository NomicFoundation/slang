use std::path::PathBuf;

use clap::Parser;

use syntax_schema::{generators, schema, validation};

#[derive(Parser, Debug)]
struct ProgramArgs {
    #[clap(long)]
    manifest_path: String,

    #[clap(long)]
    parser_output: String,
}

fn main() {
    let args = ProgramArgs::parse();

    println!(" => Loading Manifest");
    let grammar = schema::load_grammar(&PathBuf::from(args.manifest_path));

    println!(" => Validating Grammar");
    validation::validate(&grammar);

    println!(" => Generating Parser");
    generators::parser::generate(&grammar, &PathBuf::from(args.parser_output));
}
