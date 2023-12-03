use anyhow::{bail, Result};
use semver::Version;

use crate::version_pragmas::extract_version_pragmas;

#[test]
fn exact_single_version() -> Result<()> {
    test_aux(
        &["0.6.0", "0.7.0", "0.8.0"],
        "
            pragma solidity 0.7.0;
        ",
        &["0.7.0"],
    )
}

#[test]
fn multiple_versions() -> Result<()> {
    test_aux(
        &[
            "0.7.0", "0.7.1", "0.7.2", "0.7.3", "0.7.4", "0.7.5", "0.7.6",
        ],
        "
            pragma solidity >=0.7.2 <=0.7.4;
        ",
        &["0.7.2", "0.7.3", "0.7.4"],
    )
}

#[test]
fn multiple_version_pragmas() -> Result<()> {
    test_aux(
        &[
            "0.8.0", "0.8.1", "0.8.2", "0.8.3", "0.8.4", "0.8.5", "0.8.6", "0.8.7", "0.8.8",
            "0.8.9",
        ],
        "
            pragma solidity ^0.8.3;
            pragma solidity <0.8.7;
        ",
        &["0.8.3", "0.8.4", "0.8.5", "0.8.6"],
    )
}

#[test]
fn version_alternatives() -> Result<()> {
    test_aux(
        &[
            "0.8.0", "0.8.1", "0.8.2", "0.8.3", "0.8.4", "0.8.5", "0.8.6", "0.8.7", "0.8.8",
            "0.8.9",
        ],
        "
            pragma solidity 0.8.3 || 0.8.7;
        ",
        &["0.8.3", "0.8.7"],
    )
}

#[test]
fn version_range() -> Result<()> {
    test_aux(
        &[
            "0.8.0", "0.8.1", "0.8.2", "0.8.3", "0.8.4", "0.8.5", "0.8.6", "0.8.7", "0.8.8",
            "0.8.9",
        ],
        "
            pragma solidity 0.8.3 - 0.8.7;
        ",
        &["0.8.3", "0.8.4", "0.8.5", "0.8.6", "0.8.7"],
    )
}

#[test]
fn nested_expressions() -> Result<()> {
    test_aux(
        &[
            "0.8.0", "0.8.1", "0.8.2", "0.8.3", "0.8.4", "0.8.5", "0.8.6", "0.8.7", "0.8.8",
            "0.8.9",
        ],
        "
            pragma solidity 0.8.1 || 0.8.4 - 0.8.6;
        ",
        &["0.8.1", "0.8.4", "0.8.5", "0.8.6"],
    )
}

fn test_aux(all_versions: &[&str], source: &str, expected: &[&str]) -> Result<()> {
    let all_versions = all_versions
        .iter()
        .map(|version| Version::parse(version))
        .collect::<Result<Vec<Version>, semver::Error>>()?;

    let latest_version = all_versions.iter().max().unwrap();
    let pragmas = extract_version_pragmas(source, latest_version)?;

    if pragmas.is_empty() {
        bail!("Failed to extract version pragmas.");
    }

    let actual = all_versions
        .iter()
        .filter(|version| pragmas.iter().all(|pragma| pragma.matches(version)))
        .map(|version| version.to_string())
        .collect::<Vec<String>>();

    assert_eq!(expected, actual);

    Ok(())
}
