use anyhow::{bail, Context, Result};
use semver::Version;
use solidity_rust_lib::generated::{kinds::ProductionKind, language::Language};

use crate::compatible_versions::filter_compatible_versions;

#[test]
fn exact_single_version() -> Result<()> {
    return test_aux(
        &[
            Version::parse("0.6.0")?,
            Version::parse("0.7.0")?,
            Version::parse("0.8.0")?,
        ],
        "
            pragma solidity 0.7.0;
        ",
        &vec![&Version::parse("0.7.0")?],
    );
}

#[test]
fn version_range() -> Result<()> {
    return test_aux(
        &[
            Version::parse("0.7.0")?,
            Version::parse("0.7.1")?,
            Version::parse("0.7.2")?,
            Version::parse("0.7.3")?,
            Version::parse("0.7.4")?,
            Version::parse("0.7.5")?,
            Version::parse("0.7.6")?,
        ],
        "
            pragma solidity >=0.7.2 <=0.7.4;
        ",
        &vec![
            &Version::parse("0.7.2")?,
            &Version::parse("0.7.3")?,
            &Version::parse("0.7.4")?,
        ],
    );
}

#[test]
fn multiple_version_pragmas() -> Result<()> {
    return test_aux(
        &[
            Version::parse("0.8.0")?,
            Version::parse("0.8.1")?,
            Version::parse("0.8.2")?,
            Version::parse("0.8.3")?,
            Version::parse("0.8.4")?,
            Version::parse("0.8.5")?,
            Version::parse("0.8.6")?,
            Version::parse("0.8.7")?,
            Version::parse("0.8.8")?,
            Version::parse("0.8.9")?,
        ],
        "
            pragma solidity ^0.8.3;
            pragma solidity <0.8.7;
        ",
        &vec![
            &Version::parse("0.8.3")?,
            &Version::parse("0.8.4")?,
            &Version::parse("0.8.5")?,
            &Version::parse("0.8.6")?,
        ],
    );
}

fn test_aux(all_versions: &[Version], source: &str, expected: &Vec<&Version>) -> Result<()> {
    let source_id = "test.sol";
    let latest_version = all_versions.last().context("No versions provided")?;
    let output = Language::new(latest_version.to_owned())
        .get_parser(ProductionKind::SourceUnit)
        .parse(source);

    let parse_tree = if let Some(root_node) = output.parse_tree() {
        root_node
    } else {
        let errors = output
            .errors_as_strings(source_id, source, /* with_colour */ true)
            .join("\n");

        bail!("Failed to produce CST using latest version {latest_version}:\n{errors}");
    };

    let actual =
        &filter_compatible_versions(all_versions, &parse_tree, source)?.collect::<Vec<&Version>>();

    assert_eq!(expected, actual);

    return Ok(());
}
