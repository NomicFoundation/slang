use super::fixtures;
use crate::ast::IdentifierPath;
use crate::compilation::CompilationUnit;
use crate::{ast, define_fixture};

define_fixture!(
    ChainedImports,
    file: "first.sol", r#"
pragma solidity *;
import {B2 as B1} from "second.sol";
interface I1 {}
contract A1 is I1, B1 {}
"#,
    file: "second.sol", r#"
pragma solidity *;
import {B3 as B2} from "third.sol";
"#,
    file: "third.sol", r#"
pragma solidity *;
contract B3 {}
"#,
);

#[test]
fn test_build_chained_imports_fixture() {
    let unit = ChainedImports::build_compilation_unit();
    assert_eq!(3, unit.files().count());
}

define_fixture!(
    AliasedBaseConstructor,
    file: "base.sol", r#"
pragma solidity *;
contract Base {
    constructor(uint256) {}
}
interface I {}
"#,
    file: "leaf.sol", r#"
pragma solidity *;
import {Base as Aliased, I} from "base.sol";
contract Leaf is Aliased, I {
    constructor() Aliased(1) I() {}
}
"#,
);

/// The names the two modifier-list entries of `Leaf`'s constructor carry: the aliased base, then
/// the interface.
fn leaf_base_names(unit: &CompilationUnit) -> (IdentifierPath, IdentifierPath) {
    let leaf = unit
        .find_contract_by_name("Leaf")
        .next()
        .expect("contract is found");
    let constructor = leaf.constructor().expect("Leaf declares a constructor");
    let invocations = constructor.attributes().modifier_invocations();
    let mut names = invocations.iter().map(|invocation| invocation.name());
    let (Some(aliased), Some(interface), None) = (names.next(), names.next(), names.next()) else {
        panic!("the constructor invokes both bases");
    };
    (aliased, interface)
}

#[test]
fn test_modifier_invocation_resolves_a_base_through_an_import_alias() {
    let unit = AliasedBaseConstructor::build_compilation_unit();
    let (aliased, _) = leaf_base_names(&unit);

    assert!(matches!(
        aliased.resolve_to_definition(),
        Some(ast::Definition::Contract(_))
    ));
    assert!(matches!(
        aliased.resolve_to_immediate_definition(),
        Some(ast::Definition::ImportedSymbol(_))
    ));
}

#[test]
fn test_modifier_invocation_resolves_an_interface_base() {
    let unit = AliasedBaseConstructor::build_compilation_unit();
    let (_, interface) = leaf_base_names(&unit);

    assert!(matches!(
        interface.resolve_to_definition(),
        Some(ast::Definition::Interface(_))
    ));
}

#[test]
fn test_identifier_path_resolve_to_immediate_definition() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .next()
        .expect("contract is found");
    let counter_bases: Vec<_> = counter
        .inheritance_types()
        .iter()
        .map(|base_type| base_type.type_name())
        .collect();
    assert_eq!(counter_bases.len(), 2);

    assert_eq!(counter_bases[0].name(), "Ownable");
    assert!(matches!(
        counter_bases[0].resolve_to_definition(),
        Some(ast::Definition::Contract(_))
    ));
    assert!(matches!(
        counter_bases[0].resolve_to_immediate_definition(),
        Some(ast::Definition::ImportedSymbol(_))
    ));

    assert_eq!(counter_bases[1].name(), "Activatable");
    assert!(matches!(
        counter_bases[1].resolve_to_definition(),
        Some(ast::Definition::Contract(_))
    ));
    assert!(matches!(
        counter_bases[1].resolve_to_immediate_definition(),
        Some(ast::Definition::ImportedSymbol(_))
    ));
}

#[test]
fn test_identifier_path_resolve_to_immediate_resolves_to_direct_definition() {
    let unit = ChainedImports::build_compilation_unit();

    let a1 = unit
        .find_contract_by_name("A1")
        .next()
        .expect("contract is found");
    let i1_typename = a1
        .inheritance_types()
        .iter()
        .next()
        .expect("there is at least one inheritance type")
        .type_name();
    assert_eq!(i1_typename.name(), "I1");
    let Some(i1) = i1_typename.resolve_to_definition() else {
        panic!("i1 can be resolved");
    };
    let Some(i1_immediate) = i1_typename.resolve_to_immediate_definition() else {
        panic!("i1 can be resolved immediately");
    };

    assert!(matches!(
        (i1, i1_immediate),
        (ast::Definition::Interface(_), ast::Definition::Interface(_))
    ));
}

#[test]
fn test_chained_imports_resolution() {
    let unit = ChainedImports::build_compilation_unit();

    let a1 = unit
        .find_contract_by_name("A1")
        .next()
        .expect("contract is found");
    let b1_typename = a1
        .inheritance_types()
        .iter()
        .nth(1)
        .expect("there are at least two inheritance types")
        .type_name();
    assert_eq!(b1_typename.name(), "B1");

    let b1 = b1_typename
        .resolve_to_immediate_definition()
        .expect("b1 base can be resolved");
    let ast::Definition::ImportedSymbol(b1_import) = b1 else {
        panic!("b1 resolves to an import symbol");
    };

    let b2 = b1_import
        .name()
        .resolve_to_immediate_definition()
        .expect("b1 import can be resolved");
    let ast::Definition::ImportedSymbol(b2_import) = b2 else {
        panic!("b2 resolves to an import symbol");
    };

    let b3 = b2_import
        .name()
        .resolve_to_immediate_definition()
        .expect("b2 import can be resolved");
    let ast::Definition::Contract(b3_contract) = b3 else {
        panic!("b3 resolves to a contract");
    };
    assert_eq!(b3_contract.name().name(), "B3");
}
