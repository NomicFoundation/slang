use super::fixtures;
use crate::ast::{ContractBase, Definition};

#[test]
fn test_contract_direct_bases() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .next()
        .expect("can find Counter contract");
    let bases = counter.direct_bases();
    assert_eq!(bases.len(), 2);

    let ContractBase::Contract(ownable) = &bases[0] else {
        panic!("Base is not a contract");
    };
    assert_eq!(ownable.name().name(), "Ownable");
    let ContractBase::Contract(activatable) = &bases[1] else {
        panic!("Base is not a contract");
    };
    assert_eq!(activatable.name().name(), "Activatable");
}

#[test]
fn test_contract_constructor_and_modifiers() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .next()
        .expect("can find Counter contract");

    let constructor = counter.constructor();
    assert!(constructor.is_some());

    let modifiers = counter.modifiers();
    assert_eq!(modifiers.len(), 0);
}

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
