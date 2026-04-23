use anyhow::Result;
use slang_solidity_v2_ast::ast::{self, ContractBase, ContractMember, Definition, FunctionKind};
use slang_solidity_v2_common::built_ins::BuiltIn;

use super::fixtures;

#[derive(Default)]
struct IdentifierCounter {
    total: usize,
    definitions: usize,
    references: usize,
}

impl ast::visitor::Visitor for IdentifierCounter {
    fn visit_identifier(&mut self, node: &ast::Identifier) {
        if node.is_name_of_definition() {
            self.definitions += 1;
        }
        if node.is_reference() {
            self.references += 1;
        }
        self.total += 1;
    }
}

#[test]
fn test_ast_visitor() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;

    let main_ast = unit.get_file_ast_root("main.sol").unwrap();

    let mut main_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&main_ast, &mut main_visitor);

    assert_eq!(main_visitor.total, 25);
    assert_eq!(main_visitor.definitions, 9);
    assert_eq!(main_visitor.references, 18);

    let ownable_ast = unit.get_file_ast_root("ownable.sol").unwrap();

    let mut ownable_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&ownable_ast, &mut ownable_visitor);

    assert_eq!(ownable_visitor.total, 11);
    assert_eq!(ownable_visitor.definitions, 3);
    assert_eq!(ownable_visitor.references, 8);

    let activatable_ast = unit.get_file_ast_root("activatable.sol").unwrap();

    let mut activatable_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&activatable_ast, &mut activatable_visitor);

    assert_eq!(activatable_visitor.total, 31);
    assert_eq!(activatable_visitor.definitions, 10);
    assert_eq!(activatable_visitor.references, 22);

    Ok(())
}

#[test]
fn test_identifier_path_resolve_to_immediate_definition() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("contract is found");
    let counter_bases: Vec<_> = counter
        .inheritance_types()
        .iter()
        .map(|base_type| base_type.type_name())
        .collect();
    assert_eq!(counter_bases.len(), 2);

    assert_eq!(counter_bases[0].name(), "Ownable");
    assert!(matches!(
        counter_bases[0].resolve_to_definition(),
        Some(ast::Definition::Contract(_))
    ));
    assert!(matches!(
        counter_bases[0].resolve_to_immediate_definition(),
        Some(ast::Definition::ImportedSymbol(_))
    ));

    assert_eq!(counter_bases[1].name(), "Activatable");
    assert!(matches!(
        counter_bases[1].resolve_to_definition(),
        Some(ast::Definition::Contract(_))
    ));
    assert!(matches!(
        counter_bases[1].resolve_to_immediate_definition(),
        Some(ast::Definition::ImportedSymbol(_))
    ));

    Ok(())
}

#[test]
fn test_identifier_path_resolve_to_immediate_resolves_to_direct_definition() -> Result<()> {
    let unit = fixtures::ChainedImports::build_compilation_unit()?;

    let a1 = unit.find_contract_by_name("A1").expect("contract is found");
    let i1_typename = a1
        .inheritance_types()
        .iter()
        .next()
        .expect("there is at least one inheritance type")
        .type_name();
    assert_eq!(i1_typename.name(), "I1");
    let Some(i1) = i1_typename.resolve_to_definition() else {
        panic!("i1 can be resolved");
    };
    let Some(i1_immediate) = i1_typename.resolve_to_immediate_definition() else {
        panic!("i1 can be resolved immediately");
    };

    assert!(matches!(
        (i1, i1_immediate),
        (ast::Definition::Interface(_), ast::Definition::Interface(_))
    ));

    Ok(())
}

#[test]
fn test_chained_imports_resolution() -> Result<()> {
    let unit = fixtures::ChainedImports::build_compilation_unit()?;

    let a1 = unit.find_contract_by_name("A1").expect("contract is found");
    let b1_typename = a1
        .inheritance_types()
        .iter()
        .nth(1)
        .expect("there are at least two inheritance types")
        .type_name();
    assert_eq!(b1_typename.name(), "B1");

    let b1 = b1_typename
        .resolve_to_immediate_definition()
        .expect("b1 base can be resolved");
    let ast::Definition::ImportedSymbol(b1_import) = b1 else {
        panic!("b1 resolves to an import symbol");
    };

    let b2 = b1_import
        .name()
        .resolve_to_immediate_definition()
        .expect("b1 import can be resolved");
    let ast::Definition::ImportedSymbol(b2_import) = b2 else {
        panic!("b2 resolves to an import symbol");
    };

    let b3 = b2_import
        .name()
        .resolve_to_immediate_definition()
        .expect("b2 import can be resolved");
    let ast::Definition::Contract(b3_contract) = b3 else {
        panic!("b3 resolves to a contract");
    };
    assert_eq!(b3_contract.name().name(), "B3");

    Ok(())
}

#[test]
fn test_get_type() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;

    let ownable = unit
        .find_contract_by_name("Ownable")
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

    Ok(())
}

#[test]
fn test_function_get_type() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;

    let counter = unit
        .find_contract_by_name("Counter")
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
    assert!(function_type
        .associated_definition()
        .is_some_and(|definition| matches!(definition, ast::Definition::Function(_))));

    Ok(())
}

#[test]
fn test_contract_direct_bases() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;

    let counter = unit
        .find_contract_by_name("Counter")
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

    Ok(())
}

#[test]
fn test_contract_compute_linearised_bases() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("can find Counter contract");
    let bases = counter.compute_linearised_bases();
    assert_eq!(bases.len(), 3);

    let ContractBase::Contract(counter) = &bases[0] else {
        panic!("Base is not a contract");
    };
    assert_eq!(counter.name().name(), "Counter");
    let ContractBase::Contract(activatable) = &bases[1] else {
        panic!("Base is not a contract");
    };
    assert_eq!(activatable.name().name(), "Activatable");
    let ContractBase::Contract(ownable) = &bases[2] else {
        panic!("Base is not a contract");
    };
    assert_eq!(ownable.name().name(), "Ownable");

    Ok(())
}

#[test]
fn test_definition_references() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;

    let ownable = unit
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

    Ok(())
}

#[test]
fn test_contract_compute_linearised_state_variables() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("can find Counter contract");

    let state_variables = counter.compute_linearised_state_variables();
    assert_eq!(state_variables.len(), 4);

    assert_eq!(state_variables[0].name().name(), "_owner");
    assert_eq!(state_variables[1].name().name(), "_state");
    assert_eq!(state_variables[2].name().name(), "count");
    assert_eq!(state_variables[3].name().name(), "_clickers");

    Ok(())
}

#[test]
fn test_contract_compute_linearised_functions() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("can find Counter contract");

    let functions = counter.compute_linearised_functions();
    assert_eq!(functions.len(), 5);

    assert!(functions[0]
        .name()
        .is_some_and(|name| name.name() == "click"));
    assert!(functions[1]
        .name()
        .is_some_and(|name| name.name() == "disable"));
    assert!(functions[2]
        .name()
        .is_some_and(|name| name.name() == "enable"));
    assert!(functions[3]
        .name()
        .is_some_and(|name| name.name() == "increment"));
    assert!(functions[4]
        .name()
        .is_some_and(|name| name.name() == "isEnabled"));

    Ok(())
}

#[test]
fn test_contract_constructor_and_modifiers() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("can find Counter contract");

    let constructor = counter.constructor();
    assert!(constructor.is_some());

    let modifiers = counter.modifiers();
    assert_eq!(modifiers.len(), 0);

    Ok(())
}

#[test]
fn test_contract_compute_linearised_functions_with_overrides() -> Result<()> {
    let unit = fixtures::Overrides::build_compilation_unit()?;

    let inherited = unit
        .find_contract_by_name("Inherited")
        .expect("can find contract");
    let functions = inherited.compute_linearised_functions();
    assert_eq!(functions.len(), 3);
    assert!(functions[0]
        .name()
        .is_some_and(|name| name.name() == "in_base"));
    assert!(functions[1]
        .name()
        .is_some_and(|name| name.name() == "in_middle"));
    assert!(functions[2]
        .name()
        .is_some_and(|name| name.name() == "override_me"));

    Ok(())
}

#[test]
fn test_resolve_to_built_in() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;

    // Collect all identifiers that resolve to built-ins from the ownable.sol file,
    // which contains `msg.sender` and `require`.
    let ownable_ast = unit.get_file_ast_root("ownable.sol").unwrap();

    let mut built_in_collector = BuiltInCollector::default();
    ast::visitor::accept_source_unit(&ownable_ast, &mut built_in_collector);

    assert!(
        built_in_collector
            .found
            .contains(&("msg".to_string(), BuiltIn::Msg)),
        "expected Msg built-in, found: {:?}",
        built_in_collector.found,
    );
    assert!(
        built_in_collector
            .found
            .contains(&("sender".to_string(), BuiltIn::MsgSender)),
        "expected MsgSender built-in, found: {:?}",
        built_in_collector.found,
    );
    assert!(
        built_in_collector
            .found
            .contains(&("require".to_string(), BuiltIn::Require)),
        "expected Require built-in, found: {:?}",
        built_in_collector.found,
    );

    Ok(())
}

#[derive(Default)]
struct BuiltInCollector {
    found: Vec<(String, BuiltIn)>,
}

impl ast::visitor::Visitor for BuiltInCollector {
    fn visit_identifier(&mut self, node: &ast::Identifier) {
        if let Some(built_in) = node.resolve_to_built_in() {
            self.found.push((node.name(), built_in));
        }
    }
}

#[test]
fn test_get_location() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;

    let ownable = unit
        .find_contract_by_name("Ownable")
        .expect("contract is found");

    let ownable_location = ownable.get_location();
    assert_eq!(ownable_location.file_id(), "ownable.sol");

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

    let owner_location = owner.get_location();
    assert_eq!(owner_location.file_id(), "ownable.sol");

    // The state variable's range must sit within the enclosing contract's range.
    assert!(ownable_location.text_range().start <= owner_location.text_range().start);
    assert!(ownable_location.text_range().end >= owner_location.text_range().end);

    let activatable = unit
        .find_contract_by_name("Activatable")
        .expect("contract is found");
    let activatable_location = activatable.get_location();
    assert_eq!(activatable_location.file_id(), "activatable.sol");

    let counter = unit
        .find_contract_by_name("Counter")
        .expect("contract is found");
    let counter_location = counter.get_location();
    assert_eq!(counter_location.file_id(), "main.sol");

    Ok(())
}
