use std::fs;

use clap::Parser as ClapParser;
use semver::Version;
use solidity_rust_lib::generated::parse::Parsers;

#[derive(ClapParser, Debug)]
struct ProgramArgs {
    solidity_input: String,

    #[clap(long)]
    version: Version,

    #[clap(long)]
    json_output: Option<String>,

    #[clap(long)]
    yaml_output: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = ProgramArgs::parse();

    let solidity_src = fs::read_to_string(&args.solidity_input)
        .expect(&format!("Failed to read file: {:?}", args.solidity_input));

    let parser = Parsers::new(&args.version).source_unit;

    let output =
        solidity_rust_lib::internal_api::parse(&solidity_src, parser, /* with_color */ true);

    for report in &output.error_reports {
        eprintln!("{report}");
    }

    if let Some(source_unit) = output.root_node {
        if let Some(json_output) = args.json_output {
            let json = serde_json::to_string(&source_unit).expect("Failed to produce json");
            if json_output == "-" {
                println!("{}", json);
            } else {
                fs::write(&json_output, json)
                    .expect(&format!("Failed to write json file: {json_output}"));
            }
        }

        if let Some(yaml_output) = args.yaml_output {
            let yaml = serde_yaml::to_string(&source_unit).expect("Failed to produce yaml");
            if yaml_output == "-" {
                println!("{}", yaml);
            } else {
                fs::write(&yaml_output, yaml)
                    .expect(&format!("Failed to write yaml file: {yaml_output}"));
            }
        }
    }

    let errors_count =
        i32::try_from(output.error_reports.len()).expect("Failed to convert errors count to i32");
    std::process::exit(errors_count);
}
