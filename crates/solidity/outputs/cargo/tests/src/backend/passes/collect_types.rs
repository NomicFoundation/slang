use std::rc::Rc;

use anyhow::Result;
use slang_solidity::backend::l1_typed_cst;
use slang_solidity::backend::passes::collect_types;
use slang_solidity::backend::types::Type;
use slang_solidity::bindings::{BindingGraph, Definition};
use slang_solidity::cst::{NodeKind, NonterminalKind};

#[test]
fn test_collect_types_pass() -> Result<()> {
    let unit = super::build_compilation_unit()?;
    let ast = l1_typed_cst::CompilationUnit::build(&unit).unwrap();

    let mut pass = collect_types::Pass::new(Rc::clone(unit.binding_graph()));
    for ast_unit in ast.files.values() {
        l1_typed_cst::visitor::accept_source_unit(ast_unit, &mut pass);
    }
    pass.types.validate();

    let Some(address_type) = pass.types.find_type(&Type::Address { payable: false }) else {
        panic!("address type not registered");
    };
    let Some(uint_type) = pass.types.find_type(&Type::Integer {
        signed: false,
        bits: 256,
    }) else {
        panic!("uint type not registered");
    };
    assert!(pass
        .types
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
    assert!(pass.types.find_definition(state_node_id).is_some());
    assert!(pass
        .types
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
    assert!(pass.types.find_definition(counter_node_id).is_some());

    let Some(ownable_contract) = find_definition(
        unit.binding_graph(),
        "Ownable",
        NonterminalKind::ContractDefinition,
    ) else {
        panic!("Cannot find definition for Ownable contract");
    };
    let ownable_node_id = ownable_contract.get_definiens_cursor().node().id();
    assert!(pass.types.find_definition(ownable_node_id).is_some());

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
