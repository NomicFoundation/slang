use super::fixtures;
use crate::ast;

#[test]
fn test_get_type() {
    let unit = fixtures::Counter::build_compilation_unit();

    let ownable = unit
        .find_contract_by_name("Ownable")
        .next()
        .expect("contract is found");

    let state_variables = ownable
        .members()
        .iter()
        .filter_map(|member| {
            if let ast::ContractMember::StateVariableDefinition(definition) = member {
                Some(definition)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(state_variables.len(), 1);
    let owner = &state_variables[0];
    assert_eq!(owner.name().name(), "_owner");

    let owner_type = owner
        .get_type()
        .expect("_owner state variable has resolved type");
    assert!(matches!(owner_type, ast::Type::Address(_)));
}

#[test]
fn test_function_get_type() {
    let unit = fixtures::Counter::build_compilation_unit();

    let counter = unit
        .find_contract_by_name("Counter")
        .next()
        .expect("contract is found");

    let increment = counter
        .members()
        .iter()
        .find_map(|member| {
            if let ast::ContractMember::FunctionDefinition(function_definition) = member {
                if function_definition
                    .name()
                    .is_some_and(|name| name.name() == "increment")
                {
                    Some(function_definition)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .expect("increment method is found");

    let increment_type = increment.get_type().expect("increment method has a type");
    let ast::Type::Function(function_type) = increment_type else {
        panic!("method's type is expected to be a function");
    };
    assert!(function_type.is_externally_visible());
    assert!(matches!(function_type.return_type(), ast::Type::Integer(_)));
    assert!(
        function_type
            .associated_definition()
            .is_some_and(|definition| matches!(definition, ast::Definition::Function(_)))
    );
}
