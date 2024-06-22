use std::fs;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use semver::{Version, VersionReq};
use slang_solidity::assertions::{check_assertions, collect_assertions};
use slang_solidity::bindings;
use slang_solidity::cursor::Cursor;
use slang_solidity::language::Language;
use slang_solidity::query::Query;

use super::generated::VERSION_BREAKS;

pub fn run(group_name: &str, file_name: &str) -> Result<()> {
    let data_dir = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?
        .join("bindings_assertions")
        .join(group_name);
    let input_path = data_dir.join(file_name);
    let input = fs::read_to_string(&input_path)?;

    for version in &VERSION_BREAKS {
        check_assertions_with_version(version, input_path.to_str().unwrap(), &input)?;
    }
    Ok(())
}

fn check_assertions_with_version(
    version: &Version,
    file_path: &str,
    file_contents: &str,
) -> Result<()> {
    let language = Language::new(version.clone())?;

    let parse_output = language.parse(Language::ROOT_KIND, file_contents);
    assert!(parse_output.is_valid());

    let mut bindings = bindings::create(version.clone());
    bindings.add_file(file_path, parse_output.create_tree_cursor());

    let assertions = collect_assertions(parse_output.create_tree_cursor())?;

    let result = check_assertions(&bindings, &assertions);
    let version_req = collect_version_requirements(parse_output.create_tree_cursor())?;

    if let Some(version_req) = version_req {
        if version_req.matches(version) {
            assert!(
                result.is_ok(),
                "Version {version}, bindings assertions expected to pass but failed"
            );
        } else {
            assert!(
                result.is_err(),
                "Version {version}, bindings assertions expected to fail but passed"
            );
        }
    } else {
        assert!(
            result.is_ok(),
            "Failed bindings assertions in version {version} (no version requirements found)"
        );
    }

    Ok(())
}

fn collect_version_requirements(root_cursor: Cursor) -> Result<Option<VersionReq>> {
    // collect version requirements from pragma directive (if any)
    let mut comparators: Vec<String> = Vec::new();
    let version_pragma_query =
        Query::parse("[VersionExpressionSet item: [VersionExpression @version [_]]]")?;
    for result in root_cursor.query(vec![version_pragma_query]) {
        let captures = result.captures;
        let Some(version_expr) = captures.get("version").and_then(|v| v.first()) else {
            continue;
        };
        comparators.push(version_expr.node().unparse());
    }

    if comparators.is_empty() {
        Ok(None)
    } else {
        Ok(Some(VersionReq::parse(&comparators.join(", "))?))
    }
}
