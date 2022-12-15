mod generated;

use std::{collections::HashMap, io::Write};

use anyhow::Result;
use codegen_utils::context::CodegenContext;
use semver::Version;
use solidity_rust_lib::{generated::parse::Parsers, internal_api::parser::parse};

use crate::cst_output::generated::{get_parser, TEST_VERSIONS};

pub fn run(parser_name: &str, test_name: &str) -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let test_dir = codegen
            .repo_root
            .join("code-analysis/crates/solidity/test_data/cst_output")
            .join(parser_name)
            .join(test_name);

        let input = &std::fs::read_to_string(&test_dir.join("input.sol"))?;

        let mut last_output: String = String::new();

        for version in TEST_VERSIONS {
            let current_output = run_parser(version, parser_name, input)?;

            if current_output == last_output {
                // Skip versions that don't change its output.
                continue;
            }

            let output_path = test_dir.join(format!("generated/{version}.yml"));
            codegen.write_file(&output_path, &current_output)?;

            last_output = current_output;
        }

        return Ok(());
    });
}

fn run_parser(version: &str, parser_name: &str, input: &str) -> Result<String> {
    let version = Version::parse(version)?;
    let parsers = Parsers::new(&version);
    let parser = get_parser(parsers, parser_name)?;

    let parser_output = parse(&input, parser, /* with_color */ false);
    let mut result = Vec::new();

    // Manually serializing errors for now, as serde_yaml doesn't support multi-line strings.
    // In the future, errors are moving to CST nodes: https://github.com/NomicFoundation/slang/issues/195

    if parser_output.error_reports.len() == 0 {
        writeln!(&mut result, "errors: []")?;
    } else {
        writeln!(&mut result, "errors:")?;
        for report in parser_output.error_reports {
            writeln!(&mut result, "  - |")?;
            for line in report.lines() {
                writeln!(&mut result, "    {line}")?;
            }
        }
    }

    writeln!(&mut result)?;

    {
        let mut root_node = HashMap::new();
        root_node.insert("root", parser_output.root_node);

        let root_node = serde_yaml::to_string(&root_node)?;
        writeln!(&mut result, "{root_node}")?;
    }

    return Ok(String::from_utf8(result)?);
}
