use anyhow::Result;
use slang_solidity::backend::semantic::ast;

use crate::backend::sample::build_compilation_unit;

#[test]
fn test_ast_visitor() -> Result<()> {
    let unit = build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    let main_ast = semantic.get_file_ast_root("main.sol").unwrap();

    let mut main_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&main_ast, &mut main_visitor);

    assert_eq!(main_visitor.total, 25);
    assert_eq!(main_visitor.definitions, 9);
    assert_eq!(main_visitor.references, 18);

    let ownable_ast = semantic.get_file_ast_root("ownable.sol").unwrap();

    let mut ownable_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&ownable_ast, &mut ownable_visitor);

    assert_eq!(ownable_visitor.total, 11);
    assert_eq!(ownable_visitor.definitions, 3);
    assert_eq!(ownable_visitor.references, 8);

    let activatable_ast = semantic.get_file_ast_root("activatable.sol").unwrap();

    let mut activatable_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&activatable_ast, &mut activatable_visitor);

    assert_eq!(activatable_visitor.total, 31);
    assert_eq!(activatable_visitor.definitions, 10);
    assert_eq!(activatable_visitor.references, 22);

    Ok(())
}

#[derive(Default)]
struct IdentifierCounter {
    total: usize,
    definitions: usize,
    references: usize,
}

impl ast::visitor::Visitor for IdentifierCounter {
    fn visit_identifier(&mut self, node: &ast::Identifier) {
        if node.is_definition() {
            self.definitions += 1;
        }
        if node.is_reference() {
            self.references += 1;
        }
        self.total += 1;
    }
}

#[test]
fn test_text_offsets() -> Result<()> {
    let unit = build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    let counter = semantic
        .find_contract_by_name("Counter")
        .expect("contract is found");
    assert_eq!(counter.text_offset().line, 7);
    assert_eq!(counter.text_offset().column, 0);

    let click = counter
        .members()
        .iter()
        .filter_map(|member| {
            if let ast::ContractMember::FunctionDefinition(function) = member {
                Some(function)
            } else {
                None
            }
        })
        .find(|function| {
            function
                .name()
                .is_some_and(|name| name.unparse() == "click")
        })
        .expect("click method is found");
    assert_eq!(click.text_offset().line, 18);
    assert_eq!(click.text_offset().column, 4);

    Ok(())
}

#[test]
fn test_get_type() -> Result<()> {
    let unit = build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    let ownable = semantic
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
    assert_eq!(owner.name().unparse(), "_owner");

    let owner_type = owner
        .get_type()
        .expect("_owner state variable has resolved type");
    assert!(matches!(owner_type, ast::Type::Address(_)));

    Ok(())
}

#[test]
fn test_function_get_type() -> Result<()> {
    let unit = build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    let counter = semantic
        .find_contract_by_name("Counter")
        .expect("contract is found");

    let increment = counter
        .members()
        .iter()
        .find_map(|member| {
            if let ast::ContractMember::FunctionDefinition(function_definition) = member {
                if function_definition
                    .name()
                    .is_some_and(|name| name.unparse() == "increment")
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
        panic!("method's type is expect to be a function");
    };
    assert!(function_type.external());
    assert!(matches!(function_type.return_type(), ast::Type::Integer(_)));
    assert!(function_type
        .associated_definition()
        .is_some_and(|definition| matches!(definition, ast::Definition::Function(_))));

    Ok(())
}
