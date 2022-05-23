use std::path::PathBuf;

use clap::Parser;

use syntax_schema::schema::Grammar;

#[derive(Parser, Debug)]
struct ProgramArgs {
    #[clap(long)]
    manifest_input: String,

    #[clap(long)]
    no_builder: bool,

    #[clap(long)]
    chumsky_output: String,
}

fn main() {
    let args = ProgramArgs::parse();

    println!(" => Loading Manifest");
    let grammar = Grammar::from_manifest(&PathBuf::from(args.manifest_input));

    println!(" => Validating Grammar");
    grammar.validate();

    println!(" => Generating Parser");
    grammar.generate_chumsky(&PathBuf::from(args.chumsky_output));
}
