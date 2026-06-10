use super::fixtures;
use crate::ast::ContractBase;

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
