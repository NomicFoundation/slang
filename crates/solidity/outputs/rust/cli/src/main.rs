use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser as ClapParser;
use semver::Version;
use solidity_rust_lib::generated::{kinds::ProductionKind, language::Language};

#[derive(ClapParser, Debug)]
struct ProgramArgs {
    input_file: String,

    #[clap(long)]
    version: Version,

    #[clap(long)]
    json: Option<String>,

    #[clap(long)]
    yaml: Option<String>,
}

fn main() -> Result<()> {
    let args = ProgramArgs::parse();

    let input_file = &PathBuf::from(args.input_file).canonicalize()?;
    let input =
        fs::read_to_string(input_file).context(format!("Failed to read file: {input_file:?}"))?;

    let output = Language::new(args.version).parse(ProductionKind::SourceUnit, &input);

    for report in &output.errors_as_strings(
        input_file
            .to_str()
            .context("Failed to parse {input_file:?}")?,
        &input,
        /* with_colour */ true,
    ) {
        eprintln!("{report}");
    }

    if let Some(root_node) = output.parse_tree() {
        if let Some(json_path) = args.json {
            let json = serde_json::to_string_pretty(&root_node).context("Failed to write json")?;

            if json_path == "-" {
                println!("{}", json);
            } else {
                let json_path = &PathBuf::from(json_path).canonicalize()?;
                fs::write(json_path, json)
                    .context(format!("Failed to write json file: {json_path:?}"))?;
            }
        }

        if let Some(yaml_path) = args.yaml {
            let yaml = serde_yaml::to_string(&root_node).context("Failed to write yaml")?;

            if yaml_path == "-" {
                println!("{}", yaml);
            } else {
                let yaml_path = &PathBuf::from(yaml_path).canonicalize()?;
                fs::write(&yaml_path, yaml)
                    .context(format!("Failed to write yaml file: {yaml_path:?}"))?;
            }
        }
    }

    let errors_count =
        i32::try_from(output.error_count()).context("Failed to convert errors count to i32")?;

    std::process::exit(errors_count);
}
