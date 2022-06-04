use std::path::PathBuf;

use clap::Parser;

use syntax_schema::{chumsky::generate::Context, schema::Grammar};

#[derive(Parser, Debug)]
struct ProgramArgs {
    #[clap(long)]
    manifest_input: String,

    #[clap(long)]
    no_default_map: bool,

    #[clap(long)]
    box_non_tokens: bool,

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
    grammar.generate_chumsky(
        &Context {
            no_default_map: args.no_default_map,
            box_non_tokens: args.box_non_tokens,
        },
        &PathBuf::from(args.chumsky_output),
    );
}
