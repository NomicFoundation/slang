use super::fixtures;
use crate::ast::Definition;
use crate::define_fixture;

#[test]
fn test_function_enclosing_definition() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .next()
        .expect("can find Counter contract");
    let increment = counter
        .functions()
        .into_iter()
        .find(|function| {
            function
                .name()
                .is_some_and(|name| name.name() == "increment")
        })
        .expect("can find increment function");

    let Some(Definition::Contract(contract)) = increment.enclosing_definition() else {
        panic!("expected increment's enclosing definition to be a contract");
    };
    assert_eq!(contract.name().name(), "Counter");
}

define_fixture!(
    EnclosingScopes,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

struct Top { uint256 x; }

library L {
    struct S { uint256 a; }
    enum E { A, B }
}
"#);

#[test]
fn test_struct_enclosing_definition() {
    let unit = EnclosingScopes::build_compilation_unit();

    // A struct declared inside a library resolves to that library.
    let struct_s = unit
        .all_definitions()
        .find_map(|definition| match definition {
            Definition::Struct(struct_definition) if struct_definition.name().name() == "S" => {
                Some(struct_definition)
            }
            _ => None,
        })
        .expect("can find struct S");

    let Some(Definition::Library(library)) = struct_s.enclosing_definition() else {
        panic!("expected struct S's enclosing definition to be a library");
    };
    assert_eq!(library.name().name(), "L");
}

#[test]
fn test_enum_enclosing_definition() {
    let unit = EnclosingScopes::build_compilation_unit();

    let enum_e = unit
        .all_definitions()
        .find_map(|definition| match definition {
            Definition::Enum(enum_definition) if enum_definition.name().name() == "E" => {
                Some(enum_definition)
            }
            _ => None,
        })
        .expect("can find enum E");

    let Some(Definition::Library(library)) = enum_e.enclosing_definition() else {
        panic!("expected enum E's enclosing definition to be a library");
    };
    assert_eq!(library.name().name(), "L");
}

#[test]
fn test_struct_member_enclosing_definition() {
    let unit = EnclosingScopes::build_compilation_unit();

    // The immediate enclosing definition of a struct member is the struct, not the library.
    let member = unit
        .all_definitions()
        .find_map(|definition| match definition {
            Definition::StructMember(struct_member) if struct_member.name().name() == "a" => {
                Some(struct_member)
            }
            _ => None,
        })
        .expect("can find struct member a");

    let Some(Definition::Struct(struct_definition)) = member.enclosing_definition() else {
        panic!("expected the struct member's enclosing definition to be the struct");
    };
    assert_eq!(struct_definition.name().name(), "S");
}

#[test]
fn test_file_level_definition_has_no_enclosing() {
    let unit = EnclosingScopes::build_compilation_unit();

    // A file-level struct's container is the source unit.
    let top = unit
        .all_definitions()
        .find_map(|definition| match definition {
            Definition::Struct(struct_definition) if struct_definition.name().name() == "Top" => {
                Some(struct_definition)
            }
            _ => None,
        })
        .expect("can find file-level struct Top");

    assert!(
        top.enclosing_definition().is_none(),
        "a file-level struct has no enclosing definition"
    );
}
