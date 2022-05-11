use clap::Parser;
use std::path::PathBuf;

mod generators;
mod schema;
mod validation;

#[derive(Parser, Debug)]
struct ProgramArgs {
    #[clap(long)]
    manifest_path: String,

    #[clap(long)]
    ebnf_output: String,

    #[clap(long)]
    parser_output: String,
}

fn main() {
    println!();
    let args = ProgramArgs::parse();

    println!(" => Loading Manifest");
    let grammar = schema::load_grammar(&PathBuf::from(args.manifest_path));

    println!(" => Validating Grammar");
    validation::validate(&grammar);

    println!(" => Generating: EBNF");
    generators::ebnf::generate(&grammar, &PathBuf::from(args.ebnf_output));

    println!(" => Generating: Parser");
    generators::parser::generate(&grammar, &PathBuf::from(args.parser_output));
}
