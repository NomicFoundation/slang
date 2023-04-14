use anyhow::{bail, Context, Result};
use semver::Version;
use slang_solidity::syntax::parser::{Language, ProductionKind};

use crate::compatible_versions::filter_compatible_versions;

#[test]
fn exact_single_version() -> Result<()> {
    return test_aux(
        &["0.6.0", "0.7.0", "0.8.0"],
        "
            pragma solidity 0.7.0;
        ",
        &["0.7.0"],
    );
}

#[test]
fn multiple_versions() -> Result<()> {
    return test_aux(
        &[
            "0.7.0", "0.7.1", "0.7.2", "0.7.3", "0.7.4", "0.7.5", "0.7.6",
        ],
        "
            pragma solidity >=0.7.2 <=0.7.4;
        ",
        &["0.7.2", "0.7.3", "0.7.4"],
    );
}

#[test]
fn multiple_version_pragmas() -> Result<()> {
    return test_aux(
        &[
            "0.8.0", "0.8.1", "0.8.2", "0.8.3", "0.8.4", "0.8.5", "0.8.6", "0.8.7", "0.8.8",
            "0.8.9",
        ],
        "
            pragma solidity ^0.8.3;
            pragma solidity <0.8.7;
        ",
        &["0.8.3", "0.8.4", "0.8.5", "0.8.6"],
    );
}

#[test]
fn version_alternatives() -> Result<()> {
    return test_aux(
        &[
            "0.8.0", "0.8.1", "0.8.2", "0.8.3", "0.8.4", "0.8.5", "0.8.6", "0.8.7", "0.8.8",
            "0.8.9",
        ],
        "
            pragma solidity 0.8.3 || 0.8.7;
        ",
        &["0.8.3", "0.8.7"],
    );
}

#[test]
fn version_range() -> Result<()> {
    return test_aux(
        &[
            "0.8.0", "0.8.1", "0.8.2", "0.8.3", "0.8.4", "0.8.5", "0.8.6", "0.8.7", "0.8.8",
            "0.8.9",
        ],
        "
            pragma solidity 0.8.3 - 0.8.7;
        ",
        &["0.8.3", "0.8.4", "0.8.5", "0.8.6", "0.8.7"],
    );
}

#[test]
fn nested_expressions() -> Result<()> {
    return test_aux(
        &[
            "0.8.0", "0.8.1", "0.8.2", "0.8.3", "0.8.4", "0.8.5", "0.8.6", "0.8.7", "0.8.8",
            "0.8.9",
        ],
        "
            pragma solidity 0.8.1 || 0.8.4 - 0.8.6;
        ",
        &["0.8.1", "0.8.4", "0.8.5", "0.8.6"],
    );
}

fn test_aux(all_versions: &[&str], source: &str, expected: &[&str]) -> Result<()> {
    let all_versions = all_versions
        .iter()
        .map(|version| Version::parse(version))
        .collect::<Result<Vec<Version>, semver::Error>>()?;

    let latest_version = all_versions.last().context("No versions provided")?;
    let output =
        Language::new(latest_version.to_owned())?.parse(ProductionKind::SourceUnit, source);

    let parse_tree = if let Some(root_node) = output.parse_tree() {
        root_node
    } else {
        let source_id = "test.sol";
        let errors = output
            .errors_as_strings(source_id, source, /* with_colour */ true)
            .join("\n");

        bail!("Failed to produce CST using latest version {latest_version}:\n{errors}");
    };

    let actual = filter_compatible_versions(&all_versions, &parse_tree, source)?
        .iter()
        .map(|version| version.to_string())
        .collect::<Vec<String>>();

    assert_eq!(expected, actual);

    return Ok(());
}
