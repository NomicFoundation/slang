mod generated;

use std::io::Write;
use std::rc::Rc;

use anyhow::Result;
use insta::assert_snapshot;
use semver::Version;
use solidity_rust_lib::generated::cst;
use solidity_rust_lib::generated::parse::Parsers;
use solidity_rust_lib::internal_api::parse;
use solidity_rust_lib::internal_api::ParserOutput;

use crate::cst_output::generated::{get_parser, BREAKING_VERSIONS};

pub fn run(parser_name: &str, test_name: &str) -> Result<()> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let test_dir = std::path::PathBuf::from(manifest_dir)
        .join("snapshots/cst_output")
        .join(parser_name)
        .join(test_name);

    let input_file = test_dir.join("input.sol");
    let input = std::fs::read_to_string(&input_file)?;

    let mut last_snapshot: String = String::new();

    for version in BREAKING_VERSIONS {
        let version = Version::parse(version)?;
        let parsers = Parsers::new(&version);
        let parser = get_parser(parsers, parser_name)?;
        let output = parse(&input, parser, /* with_color */ false);
        let current_snapshot = render_output(&output)?;

        if current_snapshot == last_snapshot {
            // Skip versions that don't change its output.
            continue;
        }

        let mut settings = insta::Settings::clone_current();
        settings.set_omit_expression(true);
        settings.set_input_file(&input_file);
        settings.set_snapshot_path(test_dir.join("output"));
        settings.set_snapshot_suffix(format!("{parser_name}@{test_name}@{version}"));
        settings.set_prepend_module_to_snapshot(false);

        settings.bind(|| assert_snapshot!(current_snapshot));

        last_snapshot = current_snapshot;
    }

    return Ok(());
}

fn render_output(output: &ParserOutput<Rc<cst::Node>>) -> Result<String> {
    let mut snapshot = Vec::new();

    if !output.error_reports.is_empty() {
        for report in &output.error_reports {
            write!(&mut snapshot, "{report}")?;
        }

        writeln!(&mut snapshot, "---")?;
    }

    if let Some(root_node) = &output.root_node {
        let ron_tree = serde_yaml::to_string(&root_node)?;
        write!(&mut snapshot, "{ron_tree}")?;
        writeln!(&mut snapshot, "---")?;
    }

    return Ok(String::from_utf8(snapshot)?);
}
