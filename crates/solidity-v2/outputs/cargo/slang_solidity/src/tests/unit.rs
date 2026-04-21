use slang_solidity_v2_ast::ast::Definition;

use super::fixtures;

#[test]
fn test_get_file_ast_root() {
    let unit = fixtures::Counter::build_compilation_unit();

    assert_eq!(unit.files().len(), 3);

    let main_ast = unit
        .get_file_ast_root("main.sol")
        .expect("main.sol is a file of the compilation unit");
    let ownable_ast = unit
        .get_file_ast_root("ownable.sol")
        .expect("ownable.sol is a file in the compilation unit");
    let activatable_ast = unit
        .get_file_ast_root("activatable.sol")
        .expect("activatable.sol is a file in the compilation unit");

    assert_eq!(main_ast.file_id(), "main.sol");
    assert_eq!(ownable_ast.file_id(), "ownable.sol");
    assert_eq!(activatable_ast.file_id(), "activatable.sol");

    assert_eq!(main_ast.contracts().len(), 1);
    assert_eq!(ownable_ast.contracts().len(), 1);
    assert_eq!(activatable_ast.contracts().len(), 1);

    let main_contracts = main_ast.contracts();
    let counter_contract = main_contracts.first().unwrap();
    assert_eq!(counter_contract.name().name(), "Counter");
    assert_eq!(counter_contract.inheritance_types().iter().count(), 2);

    let counter_bases = counter_contract
        .inheritance_types()
        .iter()
        .collect::<Vec<_>>();

    let Definition::Contract(ownable_contract) = counter_bases[0]
        .type_name()
        .resolve_to_definition()
        .expect("Counter base is resolved")
    else {
        panic!("Counter base is a contract");
    };
    assert_eq!(ownable_contract.name().name(), "Ownable");

    let Definition::Contract(activatable_contract) = counter_bases[1]
        .type_name()
        .resolve_to_definition()
        .expect("Counter base is resolved")
    else {
        panic!("Counter base is a contract");
    };
    assert_eq!(activatable_contract.name().name(), "Activatable");
}

#[test]
fn test_get_all_definitions() {
    let unit = fixtures::Counter::build_compilation_unit();

    let count = unit.all_definitions().count();
    assert_eq!(count, 22);
}

#[test]
fn test_find_contract_by_name() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("Counter contract is found");
    let ownable = unit
        .find_contract_by_name("Ownable")
        .expect("Ownable contract is found");
    let activatable = unit
        .find_contract_by_name("Activatable")
        .expect("Activatable contract is found");

    assert_eq!(counter.name().name(), "Counter");
    assert_eq!(ownable.name().name(), "Ownable");
    assert!(ownable.abstract_keyword());
    assert_eq!(activatable.name().name(), "Activatable");
    assert!(activatable.abstract_keyword());
}
