//! Worked example: deriving the EIP-712 `encodeType` string for a struct using
//! only the public AST + ABI surface. The point is to show that those APIs are
//! *sufficient* to implement the algorithm — not to be a correct or complete
//! EIP-712 implementation, and not to test the algorithm itself.
//! DON'T USE IN PRODUCTION.
//!
//! EIP-712 (<https://eips.ethereum.org/EIPS/eip-712>) encodes a struct as
//!
//! ```text
//! name "(" type₁ " " name₁ "," … "," typeₙ " " nameₙ ")"
//! ```
//!
//! then appends the encoding of every transitively-referenced struct. A
//! struct-typed member is referred to by the struct's *name*, whereas a leaf
//! member uses its ABI type spelling — so the example leans on `AbiType` for
//! leaves and walks the AST for the struct (and array-of-struct) members.

use std::sync::Arc;

use slang_solidity_v2_common::collections::OrderedSet;

use crate::abi::AbiType;
use crate::ast::{ContractMember, Definition, SourceUnitMember, StructDefinition, Type};
use crate::compilation::CompilationUnit;
use crate::define_fixture;

define_fixture!(
    Eip712,
    file: "main.sol", r#"
import {Person as P} from "person.sol";

contract C {
    struct Mail {
        P from;
        string contents;
        P[] cc;
    }
}
"#,
    file: "person.sol", r#"
struct Person {
    string name;
    address wallet;
}
"#,
);

/// Resolves the struct named `name` anywhere in the unit — at file scope or
/// nested in a contract, in any file. Used only to pick the entry point;
/// dependency resolution never round-trips through a name.
fn find_struct(unit: &CompilationUnit, name: &str) -> StructDefinition {
    unit.file_ids()
        .iter()
        .flat_map(|file_id| {
            let ast = unit.file(file_id).expect("file exists").ast();
            ast.members().iter().collect::<Vec<_>>()
        })
        .flat_map(|member| match member {
            SourceUnitMember::StructDefinition(def) => vec![def],
            SourceUnitMember::ContractDefinition(contract) => contract
                .members()
                .iter()
                .filter_map(|member| match member {
                    ContractMember::StructDefinition(def) => Some(def),
                    _ => None,
                })
                .collect(),
            _ => vec![],
        })
        .find(|def| def.name().name() == name)
        .unwrap_or_else(|| panic!("struct `{name}` is declared"))
}

/// The EIP-712 type string for a single member's type, or `None` if it has no
/// representation. Struct (and array-of-struct) members are referred to by the
/// struct's declared name; leaf members go through `AbiType`. Because EIP-712's
/// type system is narrower than the ABI's, `function` and fixed-point types —
/// which are valid `AbiType`s — are rejected here.
fn member_type_string(ty: &Type) -> Option<String> {
    match ty {
        Type::Struct(struct_type) => {
            let Definition::Struct(def) = struct_type.definition() else {
                unreachable!("a struct type resolves to a struct definition");
            };
            Some(def.name().name().clone())
        }
        Type::Array(array) => Some(format!("{}[]", member_type_string(&array.element_type())?)),
        Type::FixedSizeArray(array) => Some(format!(
            "{}[{}]",
            member_type_string(&array.element_type())?,
            array.size()
        )),
        elementary => match AbiType::try_from(elementary).ok()? {
            // Valid ABI types, but not part of EIP-712's type system.
            AbiType::Function | AbiType::FixedPointNumber { .. } => None,
            abi => Some(abi.to_string()),
        },
    }
}

/// The struct definition (if any) reachable through a member's type, looking
/// through array nesting. Returns the resolved definition — not a name — so
/// dependencies stay identified by their `NodeId`.
fn referenced_struct(ty: &Type) -> Option<StructDefinition> {
    match ty {
        Type::Struct(struct_type) => {
            let Definition::Struct(def) = struct_type.definition() else {
                unreachable!("a struct type resolves to a struct definition");
            };
            Some(def)
        }
        Type::Array(array) => referenced_struct(&array.element_type()),
        Type::FixedSizeArray(array) => referenced_struct(&array.element_type()),
        _ => None,
    }
}

/// Encodes one struct as `Name(type₁ name₁,…)`, or `None` if any member has no
/// EIP-712 representation.
fn encode_one(def: &StructDefinition) -> Option<String> {
    let mut members = Vec::new();
    for member in def.members().iter() {
        let type_string = member_type_string(&member.get_type()?)?;
        members.push(format!("{type_string} {}", member.name().name()));
    }
    Some(format!("{}({})", def.name().name(), members.join(",")))
}

/// Computes the full EIP-712 `encodeType(primary)` string.
fn encode_type(primary: &StructDefinition) -> Option<String> {
    // Transitively collect the structs referenced by the primary type. Identity
    // is the definition's `NodeId`, so a struct is never visited (or emitted)
    // twice even when reached through several members.
    let mut seen = OrderedSet::default();
    seen.insert(primary.node_id());
    let mut referenced: Vec<StructDefinition> = Vec::new();
    let mut queue = vec![Arc::clone(primary)];
    while let Some(def) = queue.pop() {
        for member in def.members().iter() {
            if let Some(dependency) = referenced_struct(&member.get_type()?) {
                if seen.insert(dependency.node_id()) {
                    referenced.push(Arc::clone(&dependency));
                    queue.push(dependency);
                }
            }
        }
    }

    // The primary type comes first; referenced types follow, sorted by name.
    // Two distinct structs can share a name, so break ties on the definition's
    // `NodeId` to keep the ordering deterministic.
    referenced.sort_by_key(|def| (def.name().name(), def.node_id()));

    let mut result = encode_one(primary)?;
    for def in &referenced {
        result.push_str(&encode_one(def)?);
    }
    Some(result)
}

#[test]
fn derives_encode_type_from_ast_and_abi() {
    // Exercises the whole surface at once: multi-file navigation, resolving a
    // member's struct type back to its declaration (through the `P` import
    // alias, so the name is `Person`), array-of-struct members, leaf members
    // via `AbiType`, and `NodeId`-based dedup (`Person` is reached twice).
    let unit = Eip712::build_compilation_unit();
    assert_eq!(
        encode_type(&find_struct(&unit, "Mail")),
        Some(
            "Mail(Person from,string contents,Person[] cc)Person(string name,address wallet)"
                .to_string()
        ),
    );
}
