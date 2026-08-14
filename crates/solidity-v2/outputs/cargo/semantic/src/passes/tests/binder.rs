use slang_solidity_v2_common::collections::Map;
use slang_solidity_v2_common::diagnostics::kinds::DiagnosticKind;
use slang_solidity_v2_common::diagnostics::kinds::type_system::TypeSystemDiagnosticKind;
use slang_solidity_v2_common::versions::LanguageVersion;

use super::{Analyse, Analysis, AnalysisBuilder, analyze};
use crate::binder::{Binder, Resolution};
use crate::types::TypeRegistry;

#[test]
fn test_collect_definitions_and_linearise_contracts() {
    const CONTENTS: &str = r###"
contract Base {}
contract Test is Base layout at 0 {}
    "###;

    let binder = analyse_definitions(CONTENTS)
        .expecting_no_diagnostics()
        .binder;

    // Verify definitions were collected
    assert_eq!(2, binder.definitions().len());
    // Verify linearisations were computed
    assert_eq!(2, binder.linearisations().len());
}

/// Runs only the definition-collecting and linearising passes, so a failure
/// can't come from a later pass consuming their output.
fn analyse_definitions(source: &str) -> AnalysisBuilder<'_> {
    Analysis::of_source(source).analyse(Analyse::Definitions)
}

fn get_contract_to_bases_map(binder: &Binder) -> Map<String, Vec<String>> {
    let mut contract_to_bases = Map::default();
    for (key, values) in binder.linearisations() {
        let contract_name = binder
            .find_definition_by_id(*key)
            .unwrap()
            .identifier()
            .unparse()
            .to_string();

        let base_names: Vec<String> = values
            .iter()
            .map(|value| {
                binder
                    .find_definition_by_id(*value)
                    .unwrap()
                    .identifier()
                    .unparse()
                    .to_string()
            })
            .collect();

        contract_to_bases.insert(contract_name, base_names);
    }
    contract_to_bases
}

#[test]
fn test_valid_linearisations() {
    const CONTENTS: &str = r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

contract D is A, B {}
interface C {}
abstract contract B is C {}
interface A is C {}
"#;

    let binder = analyse_definitions(CONTENTS)
        .expecting_no_diagnostics()
        .binder;

    let contract_to_bases = get_contract_to_bases_map(&binder);

    let mut expected = Map::default();
    expected.insert(
        "D".to_string(),
        vec![
            "D".to_string(),
            "B".to_string(),
            "A".to_string(),
            "C".to_string(),
        ],
    );
    expected.insert("A".to_string(), vec!["A".to_string(), "C".to_string()]);
    expected.insert("B".to_string(), vec!["B".to_string(), "C".to_string()]);
    expected.insert("C".to_string(), vec!["C".to_string()]);

    assert_eq!(contract_to_bases, expected);
}

#[test]
fn test_linearise_with_forward_reference() {
    // A base is referenced before it is defined. solc emits an error here
    // (`TypeError 2449: Definition of base has to precede definition of derived
    // contract`), but slang does not, as its linearisation pass is
    // order-independent, so it accepts the forward reference and computes the
    // linearisation `[D, B]`.
    const CONTENTS: &str = r#"
contract D is B {}
contract B {}
"#;

    let binder = analyse_definitions(CONTENTS)
        .expecting_no_diagnostics()
        .binder;

    let contract_to_bases = get_contract_to_bases_map(&binder);

    let mut expected = Map::default();
    expected.insert("D".to_string(), vec!["D".to_string(), "B".to_string()]);
    expected.insert("B".to_string(), vec!["B".to_string()]);

    assert_eq!(contract_to_bases, expected);
}

#[test]
fn test_linearise_with_invalid_input() {
    const CONTENTS: &str = r#"
contract Base {}

library Foo {}

// Foo is an invalid base, but it shouldn't crash the linearisation pass
contract Test is Base, Foo { // Base should resolve to the contract, not the var
    string Base;
}
"#;

    let analysis = analyse_definitions(CONTENTS).run();

    // `Foo` is a library which can't be used as a base, so check if the
    // expected diagnostic was emitted. Asserting on the exact count also
    // covers `p1` having reported nothing.
    let emitted: Vec<_> = analysis.diagnostics.iter().collect();
    assert_eq!(
        emitted.len(),
        1,
        "expected one diagnostic, got: {emitted:?}"
    );
    assert!(matches!(
        emitted[0].kind(),
        DiagnosticKind::TypeSystem(TypeSystemDiagnosticKind::InvalidBase(_))
    ));

    let contract_to_bases = get_contract_to_bases_map(&analysis.binder);

    let mut expected = Map::default();
    expected.insert("Base".to_string(), vec!["Base".to_string()]);
    expected.insert(
        "Test".to_string(),
        vec!["Test".to_string(), "Base".to_string()],
    );

    assert_eq!(contract_to_bases, expected);
}

#[test]
fn test_type_definitions() {
    const CONTENTS: &str = r###"
contract Base {
    uint256 public x;
    function foo(uint256 a) public pure returns (uint256) {
        return a;
    }
}

contract Test is Base {
    mapping(address => uint256) balances;
    bool flag;

    struct Point {
        uint256 x;
        uint256 y;
    }

    enum Color { Red, Green, Blue }

    function bar(uint256 b) external view returns (bool) {
        return flag;
    }
}
    "###;

    // Neither `p1` nor `p2` registers a type, so the baseline is the set of
    // core types a fresh registry starts with.
    let types_before = TypeRegistry::new(LanguageVersion::LATEST)
        .iter_types()
        .count();
    let analysis = Analysis::of_source(CONTENTS)
        .analyse(Analyse::Types)
        .expecting_no_diagnostics();
    let types_after = analysis.types.iter_types().count();

    // The pass registers new types for: contracts, mappings, structs, enums,
    // function types, getter types, and a `Type::UserMetaType` for each
    // type-naming definition (the two contracts, the struct, and the enum),
    // plus one externalized function type: `foo` is `public`, so its external
    // form is a distinct type, while `bar` is already `external` and takes no
    // calldata, so its external form is its declared type.
    let registered_types = types_after - types_before;
    assert_eq!(registered_types, 12);
}

#[test]
fn test_resolve_references() {
    const CONTENTS: &str = r###"
contract Base {
    uint256 public x;
    function foo(uint256 a) public pure returns (uint256) {
        return a;
    }
}

contract Test is Base {
    mapping(address => uint256) balances;
    bool flag;

    struct Point {
        uint256 x;
        uint256 y;
    }

    enum Color { Red, Green, Blue }

    function bar(uint256 b) external view returns (bool) {
        return flag;
    }

    function baz() public view returns (uint256) {
        Point memory p;
        p.x = 1;
        return balances[msg.sender];
    }
}
    "###;

    let analysis = analyze(CONTENTS);

    // Verify that references were created and most are resolved
    let references = analysis.binder.references();
    assert!(!references.is_empty(), "expected some references");

    let unresolved_count = references
        .values()
        .filter(|r| matches!(r.resolution, Resolution::Unresolved))
        .count();
    assert_eq!(
        0, unresolved_count,
        "expected all references to be resolved"
    );
}

#[test]
fn test_collect_assembly_references() {
    const CONTENTS: &str = r###"
contract Test {
    uint256 stateVar;
    function f() public {
        uint256 localVar = 1;
        assembly {
            let x := add(sload(stateVar.slot), localVar)
            function helper(a) -> b { b := a }
        }
    }
}
    "###;

    let analysis = Analysis::of_source(CONTENTS)
        .analyse(Analyse::Yul)
        .expecting_no_diagnostics();

    // The single `assembly` block was collected in p1.
    let blocks = analysis.binder.assembly_blocks();
    assert_eq!(blocks.len(), 1);
    let block = blocks.values().next().unwrap();

    // p6 recorded the Solidity definitions the block references (the state
    // variable and the local), but not the Yul definitions (`x`, `helper`,
    // `a`, `b`) nor the Yul built-ins (`add`, `sload`, `.slot`).
    let mut referenced: Vec<String> = block
        .solidity_references
        .iter()
        .map(|node_id| {
            analysis
                .binder
                .find_definition_by_id(*node_id)
                .unwrap()
                .identifier()
                .unparse()
                .to_string()
        })
        .collect();
    referenced.sort();
    assert_eq!(
        referenced,
        vec!["localVar".to_string(), "stateVar".to_string()]
    );
}

/// A symbol imported from a sibling file resolves to the declaration in that
/// file, which is what [`Analysis::file`] wiring the import paths up buys.
#[test]
fn test_imported_symbol_resolves_to_the_declaring_file() {
    let analysis = Analysis::builder()
        .file("a.sol", r#"import {C} from "b.sol";"#)
        .file("b.sol", "contract C {}")
        .analyse(Analyse::Definitions)
        .expecting_no_diagnostics();

    let binder = &analysis.binder;
    let resolve = |file: &str| {
        let scope_id = binder
            .scope_id_for_file_id(&file.into())
            .unwrap_or_else(|| panic!("no file scope for {file}"));
        binder.follow_symbol_aliases(binder.resolve_in_scope(scope_id, "C"))
    };

    assert_eq!(
        resolve("a.sol"),
        resolve("b.sol"),
        "the import alias should follow through to `b.sol`'s declaration"
    );
    assert!(
        matches!(resolve("a.sol"), Resolution::Definition(_)),
        "expected a single declaration"
    );
}

/// An import naming no known file stays unresolved, as it would for a file
/// that isn't there.
#[test]
fn test_import_of_an_unknown_file_stays_unresolved() {
    let analysis = Analysis::builder()
        .file("a.sol", r#"import {C} from "missing.sol";"#)
        .analyse(Analyse::Definitions)
        .expecting_no_diagnostics();

    let scope_id = analysis
        .binder
        .scope_id_for_file_id(&"a.sol".into())
        .expect("no file scope for a.sol");
    let resolution = analysis.binder.resolve_in_scope(scope_id, "C");

    assert_eq!(
        Resolution::Unresolved,
        analysis.binder.follow_symbol_aliases(resolution)
    );
}
