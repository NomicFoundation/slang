use anyhow::Result;
use slang_solidity::backend::semantic::ast;
use slang_solidity::utils::LanguageFacts;

use crate::backend::fixtures;
use crate::compilation::compilation_unit::build_compilation_unit_from_multi_part_file;

#[test]
fn test_ast_visitor() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;
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
fn test_text_offsets() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;
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
        .find(|function| function.name().is_some_and(|name| name.name() == "click"))
        .expect("click method is found");
    assert_eq!(click.text_offset().line, 18);
    assert_eq!(click.text_offset().column, 4);

    let click_identifier = click.name().unwrap();
    assert_eq!(click_identifier.text_offset().line, 18);
    assert_eq!(click_identifier.text_offset().column, 13);

    Ok(())
}

#[test]
fn test_resolve_to_immediate_definition() -> Result<()> {
    let unit = fixtures::Counter::build_compilation_unit()?;
    let semantic = unit.semantic_analysis();

    let counter = semantic
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

const CHAINED_SAMPLE: &str = r#"
// ---- path: first.sol
import {B2 as B1} from "second.sol";
interface I1 {}
contract A1 is I1, B1 {}

// ---- path: second.sol
import {B3 as B2} from "third.sol";

// ---- path: third.sol
contract B3 {}
"#;

#[test]
fn test_resolve_to_immediate_resolves_to_direct_definition() -> Result<()> {
    let unit = build_compilation_unit_from_multi_part_file(
        &LanguageFacts::LATEST_VERSION,
        CHAINED_SAMPLE,
    )?;
    let semantic = unit.semantic_analysis();

    let a1 = semantic
        .find_contract_by_name("A1")
        .expect("contract is found");
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
    let unit = build_compilation_unit_from_multi_part_file(
        &LanguageFacts::LATEST_VERSION,
        CHAINED_SAMPLE,
    )?;
    let semantic = unit.semantic_analysis();

    let a1 = semantic
        .find_contract_by_name("A1")
        .expect("contract is found");
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
