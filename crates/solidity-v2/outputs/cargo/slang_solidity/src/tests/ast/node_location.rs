use super::fixtures;
use crate::ast;

#[test]
fn test_get_file_id_and_text_range() {
    let unit = fixtures::Counter::build_compilation_unit();

    let ownable = unit
        .find_contract_by_name("Ownable")
        .expect("contract is found");
    assert_eq!(ownable.get_file_id(), "ownable.sol");

    let owner = ownable
        .members()
        .iter()
        .find_map(|member| {
            if let ast::ContractMember::StateVariableDefinition(definition) = member {
                Some(definition)
            } else {
                None
            }
        })
        .expect("_owner state variable is found");
    assert_eq!(owner.name().name(), "_owner");
    assert_eq!(owner.get_file_id(), "ownable.sol");

    // The state variable's range must sit within the enclosing contract's range.
    assert!(ownable.get_text_range().start <= owner.get_text_range().start);
    assert!(ownable.get_text_range().end >= owner.get_text_range().end);

    let activatable = unit
        .find_contract_by_name("Activatable")
        .expect("contract is found");
    assert_eq!(activatable.get_file_id(), "activatable.sol");

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("contract is found");
    assert_eq!(counter.get_file_id(), "main.sol");
}
