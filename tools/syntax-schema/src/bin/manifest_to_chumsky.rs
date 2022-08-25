use std::path::PathBuf;

use clap::Parser;

use codegen::{chumsky::generate::GenerationContext, schema::Grammar};

#[derive(Parser, Debug)]
struct ProgramArgs {
    #[clap(long)]
    manifest_input: String,

    #[clap(long)]
    chumsky_output: String,
}

fn main() {
    let args = ProgramArgs::parse();

    println!(" => Loading Manifest");
    let grammar = Grammar::from_manifest(&PathBuf::from(args.manifest_input));

    println!(" => Generating Parser");
    grammar.generate_chumsky(&GenerationContext {
        output_dir: PathBuf::from(args.chumsky_output),
    });
}
