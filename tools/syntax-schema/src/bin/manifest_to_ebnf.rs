use std::path::PathBuf;

use clap::Parser;

use syntax_schema::schema::Grammar;

#[derive(Parser, Debug)]
struct ProgramArgs {
    #[clap(long)]
    manifest_input: String,

    #[clap(long)]
    ebnf_output: String,
}

fn main() {
    let args = ProgramArgs::parse();

    println!(" => Loading Manifest");
    let grammar = Grammar::from_manifest(&PathBuf::from(args.manifest_input));

    println!(" => Validating Grammar");
    grammar.validate();

    println!(" => Generating EBNF");
    grammar.generate_ebnf(&PathBuf::from(args.ebnf_output));
}
