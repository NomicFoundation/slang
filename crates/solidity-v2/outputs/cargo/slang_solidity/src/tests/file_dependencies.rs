use super::support::{compile, file_with_empty_contract};
use crate::compilation::CompilationUnit;
use crate::diagnostics::DiagnosticExtensions;

fn file_dependencies(unit: &CompilationUnit, id: &str) -> Option<Vec<String>> {
    unit.compute_file_dependencies(&id.into()).map(|ids| {
        ids.iter()
            .map(|file_id| file_id.as_str().to_owned())
            .collect()
    })
}

#[test]
fn single_file_compilation() {
    let main = file_with_empty_contract("Main", &[]);
    let unit = compile([("main.sol".into(), main.as_str())]);

    assert_eq!(file_dependencies(&unit, "main.sol").unwrap(), ["main.sol"]);
}

#[test]
fn includes_the_file_and_its_transitive_imports() {
    let main = file_with_empty_contract("Main", &["left.sol", "right.sol"]);
    let left = file_with_empty_contract("Left", &["base.sol"]);
    let right = file_with_empty_contract("Right", &["base.sol"]);
    let base = file_with_empty_contract("Base", &[]);
    let extra = file_with_empty_contract("Extra", &["main.sol"]);
    let unit = compile([
        ("main.sol".into(), main.as_str()),
        ("left.sol".into(), left.as_str()),
        ("right.sol".into(), right.as_str()),
        ("base.sol".into(), base.as_str()),
        // Imports `main.sol`, but is not imported by it.
        ("extra.sol".into(), extra.as_str()),
    ]);

    assert!(unit.diagnostics().is_empty(), "{:#?}", unit.diagnostics());

    assert_eq!(
        file_dependencies(&unit, "main.sol").unwrap(),
        ["base.sol", "left.sol", "main.sol", "right.sol"]
    );
    assert_eq!(
        file_dependencies(&unit, "left.sol").unwrap(),
        ["base.sol", "left.sol"]
    );
    assert_eq!(
        file_dependencies(&unit, "right.sol").unwrap(),
        ["base.sol", "right.sol"]
    );
    assert_eq!(file_dependencies(&unit, "base.sol").unwrap(), ["base.sol"]);
    assert_eq!(
        file_dependencies(&unit, "extra.sol").unwrap(),
        ["base.sol", "extra.sol", "left.sol", "main.sol", "right.sol"]
    );
}

#[test]
fn follows_cyclic_imports() {
    let first = file_with_empty_contract("First", &["second.sol"]);
    let second = file_with_empty_contract("Second", &["third.sol"]);
    let third = file_with_empty_contract("Third", &["first.sol"]);
    let unit = compile([
        ("first.sol".into(), first.as_str()),
        ("second.sol".into(), second.as_str()),
        ("third.sol".into(), third.as_str()),
    ]);

    assert!(unit.diagnostics().is_empty(), "{:#?}", unit.diagnostics());

    for id in ["first.sol", "second.sol", "third.sol"] {
        assert_eq!(
            file_dependencies(&unit, id).unwrap(),
            ["first.sol", "second.sol", "third.sol"]
        );
    }
}

#[test]
fn includes_imported_files_missing_from_the_unit() {
    let main = file_with_empty_contract("Main", &["lib.sol"]);
    let lib = file_with_empty_contract("Lib", &["absent.sol"]);
    let unit = compile([
        ("main.sol".into(), main.as_str()),
        ("lib.sol".into(), lib.as_str()),
    ]);

    let codes: Vec<_> = unit
        .diagnostics()
        .iter()
        .map(|diagnostic| diagnostic.code())
        .collect();
    assert_eq!(codes, ["compilation/missing-imported-file"]);

    assert_eq!(
        file_dependencies(&unit, "main.sol").unwrap(),
        ["absent.sol", "lib.sol", "main.sol"]
    );
    assert_eq!(
        file_dependencies(&unit, "lib.sol").unwrap(),
        ["absent.sol", "lib.sol"]
    );
    assert!(file_dependencies(&unit, "absent.sol").is_none());
}

#[test]
fn returns_none_for_a_file_outside_the_unit() {
    let main = file_with_empty_contract("Main", &[]);
    let unit = compile([("main.sol".into(), main.as_str())]);

    assert!(file_dependencies(&unit, "other.sol").is_none());
}
