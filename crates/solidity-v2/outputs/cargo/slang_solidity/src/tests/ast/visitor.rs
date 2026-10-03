use super::fixtures;
use crate::ast::{self, NodeLocation};
use crate::define_fixture;

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
fn test_ast_visitor() {
    let unit = fixtures::Counter::build_compilation_unit();

    let main_ast = unit.file(&"main.sol".into()).unwrap().ast();

    let mut main_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&main_ast, &mut main_visitor);

    assert_eq!(main_visitor.total, 25);
    assert_eq!(main_visitor.definitions, 9);
    assert_eq!(main_visitor.references, 18);

    let ownable_ast = unit.file(&"ownable.sol".into()).unwrap().ast();

    let mut ownable_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&ownable_ast, &mut ownable_visitor);

    assert_eq!(ownable_visitor.total, 11);
    assert_eq!(ownable_visitor.definitions, 3);
    assert_eq!(ownable_visitor.references, 8);

    let activatable_ast = unit.file(&"activatable.sol".into()).unwrap().ast();

    let mut activatable_visitor = IdentifierCounter::default();
    ast::visitor::accept_source_unit(&activatable_ast, &mut activatable_visitor);

    assert_eq!(activatable_visitor.total, 31);
    assert_eq!(activatable_visitor.definitions, 10);
    assert_eq!(activatable_visitor.references, 22);
}

const NONTERMINALS: &str = "pragma solidity ^0.8.0;
contract C { uint public x; function f(uint a) public view returns (uint) { return a + x; } }";

define_fixture!(
    Nonterminals,
    file: "main.sol", NONTERMINALS,
);

#[derive(Default)]
struct NonterminalTexts(Vec<Option<&'static str>>);

impl ast::visitor::Visitor for NonterminalTexts {
    fn enter_nonterminal(&mut self, node: &dyn NodeLocation) {
        self.0.push(
            node.calculate_text_range()
                .map(|range| &NONTERMINALS[range]),
        );
    }
}

#[test]
fn test_enter_nonterminal_order() {
    let unit = Nonterminals::build_compilation_unit();
    let source_unit = unit.file(&"main.sol".into()).unwrap().ast();

    let mut texts = NonterminalTexts::default();
    ast::visitor::accept_source_unit(&source_unit, &mut texts);

    let contract = "contract C { uint public x; function f(uint a) public view returns (uint) { return a + x; } }";
    let function = "function f(uint a) public view returns (uint) { return a + x; }";
    assert_eq!(
        texts.0,
        [
            Some(NONTERMINALS), // SourceUnit
            Some(NONTERMINALS), // SourceUnitMembers
            Some("pragma solidity ^0.8.0;"),
            Some("solidity ^0.8.0"),
            Some("^0.8.0"), // VersionPragmaExpressionSets
            Some("^0.8.0"), // VersionPragmaExpressionSet
            Some("^0.8.0"), // VersionPragmaComparator
            None, // VersionPragmaSpecifier: its components are parsed numbers, so it has no range
            Some(contract),
            None, // InheritanceTypes
            Some("uint public x; function f(uint a) public view returns (uint) { return a + x; }"),
            Some("uint public x;"),
            Some("public"), // StateVariableAttributes
            Some(function),
            Some("uint a"), // Parameters
            Some("uint a"), // Parameter
            Some("public view"),
            None,         // ModifierInvocations
            Some("uint"), // returns: Parameters
            Some("uint"), // returns: Parameter
            Some("{ return a + x; }"),
            Some("return a + x;"), // Statements
            Some("return a + x;"), // ReturnStatement
            Some("a + x"),
        ]
    );
}
