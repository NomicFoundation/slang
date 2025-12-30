use anyhow::Result;
use slang_solidity::backend::ir::ast::{ContractBase, ContractMember, Definition, FunctionKind};

use super::sample::build_compilation_unit;

mod ast;

#[test]
fn test_semantic_analysis_and_ast_tree() -> Result<()> {
    let unit = build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    assert_eq!(unit.files().len(), semantic.files().len());

    let main_ir = semantic
        .get_file_ast_root("main.sol")
        .expect("main.sol is a file of the compilation unit");
    let ownable_ir = semantic
        .get_file_ast_root("ownable.sol")
        .expect("ownable.sol is a file in the compilation unit");
    let activatable_ir = semantic
        .get_file_ast_root("activatable.sol")
        .expect("activatable.sol is a file in the compilation unit");

    assert_eq!(main_ir.file_id(), "main.sol");
    assert_eq!(ownable_ir.file_id(), "ownable.sol");
    assert_eq!(activatable_ir.file_id(), "activatable.sol");

    assert_eq!(main_ir.contracts().len(), 1);
    assert_eq!(ownable_ir.contracts().len(), 1);
    assert_eq!(activatable_ir.contracts().len(), 1);

    let main_contracts = main_ir.contracts();
    let counter_contract = main_contracts.first().unwrap();
    assert_eq!(counter_contract.name().unparse(), "Counter");
    assert_eq!(counter_contract.inheritance_types().iter().count(), 2);

    let counter_bases = counter_contract
        .inheritance_types()
        .iter()
        .collect::<Vec<_>>();

    let Definition::Contract(activatable_contract) = counter_bases[0]
        .type_name()
        .resolve_to_definition()
        .expect("Counter base is resolved")
    else {
        panic!("Counter base is a contract");
    };
    assert_eq!(activatable_contract.name().unparse(), "Ownable");

    let Definition::Contract(ownable_contract) = counter_bases[1]
        .type_name()
        .resolve_to_definition()
        .expect("Counter base is resolved")
    else {
        panic!("Counter base is a contract");
    };
    assert_eq!(ownable_contract.name().unparse(), "Activatable");

    Ok(())
}

#[test]
fn test_get_all_definitions() -> Result<()> {
    let unit = build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    assert_eq!(semantic.all_definitions().count(), 22);

    Ok(())
}

#[test]
fn test_find_contract_by_name() -> Result<()> {
    let unit = build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    let counter = semantic
        .find_contract_by_name("Counter")
        .expect("Counter contract is found");
    let ownable = semantic
        .find_contract_by_name("Ownable")
        .expect("Ownable contract is found");
    let activatable = semantic
        .find_contract_by_name("Activatable")
        .expect("Activatable contract is found");

    assert_eq!(counter.name().unparse(), "Counter");
    assert_eq!(ownable.name().unparse(), "Ownable");
    assert!(ownable.abstract_keyword());
    assert_eq!(activatable.name().unparse(), "Activatable");
    assert!(activatable.abstract_keyword());

    Ok(())
}

#[test]
fn test_get_direct_contract_bases() -> Result<()> {
    let unit = build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    let counter = semantic
        .find_contract_by_name("Counter")
        .expect("can find Counter contract");
    let bases = counter.direct_bases();
    assert_eq!(bases.len(), 2);

    let ContractBase::Contract(ownable) = &bases[0] else {
        panic!("Base is not a contract");
    };
    assert_eq!(ownable.name().unparse(), "Ownable");
    let ContractBase::Contract(activatable) = &bases[1] else {
        panic!("Base is not a contract");
    };
    assert_eq!(activatable.name().unparse(), "Activatable");

    Ok(())
}

#[test]
fn test_get_linearised_contract_bases() -> Result<()> {
    let unit = build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    let counter = semantic
        .find_contract_by_name("Counter")
        .expect("can find Counter contract");
    let bases = counter.linearised_bases();
    assert_eq!(bases.len(), 3);

    let ContractBase::Contract(counter) = &bases[0] else {
        panic!("Base is not a contract");
    };
    assert_eq!(counter.name().unparse(), "Counter");
    let ContractBase::Contract(activatable) = &bases[1] else {
        panic!("Base is not a contract");
    };
    assert_eq!(activatable.name().unparse(), "Activatable");
    let ContractBase::Contract(ownable) = &bases[2] else {
        panic!("Base is not a contract");
    };
    assert_eq!(ownable.name().unparse(), "Ownable");

    Ok(())
}

#[test]
fn test_get_references() -> Result<()> {
    let unit = build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    let ownable = semantic
        .find_contract_by_name("Ownable")
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
                    .is_some_and(|name| name.unparse() == "onlyOwner")
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
            .is_some_and(|name| name.unparse() == "onlyOwner"))));

    Ok(())
}
