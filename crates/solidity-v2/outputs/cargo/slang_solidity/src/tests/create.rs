use super::support::{compile, file_with_empty_contract};
use crate::diagnostics::DiagnosticExtensions;

#[test]
fn compiles_every_source_it_is_given() {
    let main = file_with_empty_contract("Main", &["lib.sol"]);
    let lib = file_with_empty_contract("Lib", &[]);
    let extra = file_with_empty_contract("Extra", &[]);
    let unit = compile([
        ("main.sol".into(), main.as_str()),
        ("lib.sol".into(), lib.as_str()),
        // Not imported by anything, but still part of the compilation.
        ("extra.sol".into(), extra.as_str()),
    ]);

    assert!(unit.diagnostics().is_empty(), "{:#?}", unit.diagnostics());

    // `files()` yields the files sorted by ID, but that order is not part of
    // the API, so sort here and assert on the set of files instead.
    let mut file_ids: Vec<String> = unit
        .files()
        .map(|file| file.id().as_str().to_owned())
        .collect();
    file_ids.sort();
    assert_eq!(file_ids, ["extra.sol", "lib.sol", "main.sol"]);
}

#[test]
fn the_last_contents_given_for_a_file_id_win() {
    let stale = file_with_empty_contract("Stale", &[]);
    let fresh = file_with_empty_contract("Fresh", &[]);
    let unit = compile([
        ("main.sol".into(), stale.as_str()),
        ("main.sol".into(), fresh.as_str()),
    ]);

    // The repetition is reported, and the compilation proceeds with the last
    // contents given for the ID.
    let diagnostics: Vec<_> = unit.diagnostics().iter().collect();
    assert_eq!(
        1,
        diagnostics.len(),
        "expected exactly the duplicated file ID, but found: {diagnostics:#?}"
    );

    let diagnostic = diagnostics[0];
    assert_eq!("compilation/duplicated-file-id", diagnostic.code());
    assert_eq!("main.sol", diagnostic.file_id().as_str());
    assert_eq!(
        "Source file provided more than once: main.sol",
        diagnostic.message()
    );

    assert_eq!(unit.files().count(), 1);

    let contract_names: Vec<String> = unit
        .all_contracts()
        .map(|contract| contract.name().name().to_owned())
        .collect();
    assert_eq!(contract_names, ["Fresh"]);
}

#[test]
fn compiles_an_empty_source_list() {
    let unit = compile([]);

    assert!(unit.diagnostics().is_empty(), "{:#?}", unit.diagnostics());
    assert_eq!(unit.files().count(), 0);
}
