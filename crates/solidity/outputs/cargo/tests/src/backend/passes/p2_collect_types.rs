use std::rc::Rc;

use anyhow::{anyhow, Result};
use slang_solidity::backend::passes::{p0_build_ast, p1_flatten_contracts, p2_collect_types};
use slang_solidity::backend::types::Type;
use slang_solidity::bindings::{BindingGraph, Definition};
use slang_solidity::cst::{NodeKind, NonterminalKind};

#[test]
fn test_collect_types_pass() -> Result<()> {
    let unit = super::build_compilation_unit()?;

    let data = p0_build_ast::run(&unit).map_err(|s| anyhow!(s))?;
    let data = p1_flatten_contracts::run(data);
    let data = p2_collect_types::run(data);

    let types = &data.types;

    let Some(address_type) = types.find_type(&Type::Address { payable: false }) else {
        panic!("address type not registered");
    };
    let Some(uint_type) = types.find_type(&Type::Integer {
        signed: false,
        bits: 256,
    }) else {
        panic!("uint type not registered");
    };
    assert!(types
        .find_type(&Type::Mapping {
            key_name: None,
            key_type_id: address_type,
            value_name: None,
            value_type_id: uint_type,
        })
        .is_some());

    let Some(state_enum) = find_definition(
        unit.binding_graph(),
        "State",
        NonterminalKind::EnumDefinition,
    ) else {
        panic!("Cannot find definition for State enum");
    };
    let state_node_id = state_enum.get_definiens_cursor().node().id();
    assert!(types
        .get_type_definition_by_node_id(state_node_id)
        .is_some());
    assert!(types
        .find_type(&Type::Enum {
            node_id: state_node_id
        })
        .is_some());

    let Some(counter_contract) = find_definition(
        unit.binding_graph(),
        "Counter",
        NonterminalKind::ContractDefinition,
    ) else {
        panic!("Cannot find definition for Counter contract");
    };
    let counter_node_id = counter_contract.get_definiens_cursor().node().id();
    assert!(types
        .get_type_definition_by_node_id(counter_node_id)
        .is_some());

    let Some(ownable_contract) = find_definition(
        unit.binding_graph(),
        "Ownable",
        NonterminalKind::ContractDefinition,
    ) else {
        panic!("Cannot find definition for Ownable contract");
    };
    let ownable_node_id = ownable_contract.get_definiens_cursor().node().id();
    assert!(types
        .get_type_definition_by_node_id(ownable_node_id)
        .is_some());

    Ok(())
}

fn find_definition(
    binding_graph: &Rc<BindingGraph>,
    name: &str,
    kind: NonterminalKind,
) -> Option<Definition> {
    binding_graph.all_definitions().find(|definition| {
        definition.get_cursor().node().unparse() == name
            && definition.get_definiens_cursor().node().kind() == NodeKind::Nonterminal(kind)
    })
}
