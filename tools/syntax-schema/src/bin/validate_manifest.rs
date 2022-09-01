use std::path::PathBuf;

use clap::Parser;

use codegen::schema::Grammar;

#[derive(Parser, Debug)]
struct ProgramArgs {
    #[clap(long)]
    manifest_input: String,
}

fn main() {
    let args = ProgramArgs::parse();
    let manifest_input = PathBuf::from(args.manifest_input);

    println!(" => Loading Manifest");
    let grammar = Grammar::from_manifest(&manifest_input);

    println!(" => Validating Manifest");
    grammar.validate(&manifest_input);
}
