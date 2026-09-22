//! Tests that the ids recorded for a getter's inputs and outputs are the
//! parameters and struct members the variable's type declares.

use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::support::{Analyse, Analysis};
use crate::binder::Definition;

const SOURCE: &str = r#"
    pragma solidity *;
    contract C {
        struct S { uint256 x; uint256[] ys; }
        mapping(uint256 key => uint256[] values)[] public arrayOfMappings;
        mapping(uint256 => S) public unnamedKeyToStruct;
    }
"#;

fn analyse() -> Analysis {
    Analysis::of_source(SOURCE)
        .run(Analyse::Types)
        .expect_no_diagnostics()
}

fn state_variable<'a>(analysis: &'a Analysis, name: &str) -> &'a ir::StateVariableDefinition {
    analysis
        .find_members("C")
        .iter()
        .find_map(|member| match member {
            ir::ContractMember::StateVariableDefinition(variable)
                if variable.name.unparse() == name =>
            {
                Some(variable)
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("no state variable `{name}`"))
}

fn getter_definition_ids<'a>(
    analysis: &'a Analysis,
    variable: &ir::StateVariableDefinition,
) -> (&'a [Option<NodeId>], &'a [Option<NodeId>]) {
    let Some(Definition::StateVariable(definition)) =
        analysis.binder().find_definition_by_id(variable.id())
    else {
        panic!(
            "`{name}` has no state variable definition",
            name = variable.name.unparse()
        );
    };
    (
        &definition.getter_input_definition_ids,
        &definition.getter_output_definition_ids,
    )
}

#[test]
fn getter_parameters_are_the_declared_mapping_parameters() {
    let analysis = analyse();
    let variable = state_variable(&analysis, "arrayOfMappings");

    let ir::TypeName::ArrayTypeName(array) = &variable.type_name else {
        panic!("`arrayOfMappings` is declared as an array");
    };
    let ir::TypeName::MappingType(mapping) = &array.operand else {
        panic!("`arrayOfMappings` holds mappings");
    };

    let (inputs, outputs) = getter_definition_ids(&analysis, variable);
    // The outer array index, the mapping key, then the value's array index.
    assert_eq!(inputs, [None, Some(mapping.key_type.id()), None]);
    assert_eq!(outputs, [Some(mapping.value_type.id())]);
}

#[test]
fn struct_getter_outputs_are_the_returnable_members() {
    let analysis = analyse();
    let variable = state_variable(&analysis, "unnamedKeyToStruct");

    let Some(ir::ContractMember::StructDefinition(struct_definition)) = analysis
        .find_members("C")
        .iter()
        .find(|member| matches!(member, ir::ContractMember::StructDefinition(_)))
    else {
        panic!("`C` declares a struct");
    };

    let (inputs, outputs) = getter_definition_ids(&analysis, variable);
    assert_eq!(inputs, [None], "an unnamed key names no input");
    // `ys` is an array member, which a getter does not return.
    assert_eq!(outputs, [Some(struct_definition.members[0].id())]);
}
