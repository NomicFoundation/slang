use super::fixtures;
use crate::ast::{ContractMember, Definition, FunctionKind};

#[test]
fn test_definition_references() {
    let unit = fixtures::Counter::build_compilation_unit();

    let ownable = unit
        .find_contract_by_name("Ownable")
        .next()
        .expect("can find Ownable contract");

    // find the `onlyOwner` modifier defined in the `Ownable` contract
    let only_owner = ownable
        .members()
        .iter()
        .find_map(|member| {
            let ContractMember::FunctionDefinition(function) = member else {
                return None;
            };
            if matches!(function.kind(), FunctionKind::Modifier)
                && function
                    .name()
                    .is_some_and(|name| name.name() == "onlyOwner")
            {
                Some(function)
            } else {
                None
            }
        })
        .expect("can find onlyOwner modifier");

    let references = only_owner.references();
    assert_eq!(references.len(), 3);
    assert!(references.iter().all(|reference| reference
        .resolve_to_definition()
        .and_then(|definition| {
            if let Definition::Modifier(modifier) = definition {
                Some(modifier)
            } else {
                None
            }
        })
        .is_some_and(|modifier| modifier
            .name()
            .is_some_and(|name| name.name() == "onlyOwner"))));
}
