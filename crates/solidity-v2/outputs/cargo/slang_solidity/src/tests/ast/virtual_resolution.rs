//! `ContractDefinition::resolve_virtual`, `resolve_super` and
//! `resolve_modifier`, and the anchor `super` carries: the queries a compiler
//! needs to pick the implementation a call or a modifier invocation runs when a
//! base's body is compiled into a derived contract.

use crate::ast::visitor::{Visitor, accept_function_definition};
use crate::ast::{
    ContractDefinition, Definition, Expression, FunctionDefinition, FunctionKind,
    ModifierInvocation, Type,
};
use crate::compilation::CompilationUnit;
use crate::define_fixture;

define_fixture!(
    Hierarchy,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

interface I {
    function i() external returns (uint256);
}

library L {
    modifier guarded() { _; }
    function s() internal pure returns (uint256) { return 0; }
    function attached() internal guarded {}
}

function s() pure returns (uint256) { return 0; }

abstract contract A is I {
    modifier guarded() virtual { _; }
    function bare() public guarded {}
    function qualified() public A.guarded {}
    function f() public virtual returns (uint256) { return 1; }
    function f(uint256 x) public virtual returns (uint256) { return x; }
    function g() public virtual returns (uint256);
    function s() internal pure returns (uint256) { return 0; }
}

contract B is A {
    modifier guarded() override { _; }
    function f() public virtual override returns (uint256) { return super.f() + 2; }
    function g() public virtual override returns (uint256) { return 20; }
    function i() external virtual override returns (uint256) { return 100; }
}

abstract contract M {
    function f() public virtual returns (uint256);
}

contract C is B, M {
    function f() public override(B, M) returns (uint256) { return 3; }
}
"#,
);

fn contract(unit: &CompilationUnit, name: &str) -> ContractDefinition {
    unit.find_contract_by_name(name)
        .next()
        .expect("the fixture declares the contract")
}

/// The functions and modifiers `owner`, a contract, interface or library, itself declares.
fn function_definitions(unit: &CompilationUnit, owner: &str) -> Vec<FunctionDefinition> {
    unit.all_definitions()
        .find_map(|definition| match definition {
            Definition::Contract(contract) if contract.name().name() == owner => Some(
                contract
                    .functions()
                    .into_iter()
                    .chain(contract.modifiers())
                    .collect(),
            ),
            Definition::Interface(interface) if interface.name().name() == owner => {
                Some(interface.functions())
            }
            Definition::Library(library) if library.name().name() == owner => Some(
                library
                    .functions()
                    .into_iter()
                    .chain(library.modifiers())
                    .collect(),
            ),
            _ => None,
        })
        .expect("the fixture declares the owner")
}

/// The function `name` with `arity` parameters that `owner` itself declares.
fn function(unit: &CompilationUnit, owner: &str, name: &str, arity: usize) -> FunctionDefinition {
    function_definitions(unit, owner)
        .into_iter()
        .find(|function| {
            function.name().is_some_and(|found| found.name() == name)
                && function.parameters().len() == arity
        })
        .expect("the owner declares the function")
}

/// The one modifier `owner` declares.
fn modifier(unit: &CompilationUnit, owner: &str) -> FunctionDefinition {
    function_definitions(unit, owner)
        .into_iter()
        .find(|member| matches!(member.kind(), FunctionKind::Modifier))
        .expect("the owner declares a modifier")
}

/// The one modifier-list entry on `owner`'s function `name`.
fn invocation(unit: &CompilationUnit, owner: &str, name: &str) -> ModifierInvocation {
    function(unit, owner, name, 0)
        .attributes()
        .modifier_invocations()
        .iter()
        .next()
        .expect("the function carries a modifier-list entry")
}

/// Captures the anchor of every `super` keyword under a node.
#[derive(Default)]
struct SuperAnchors {
    anchors: Vec<Option<Type>>,
}

impl Visitor for SuperAnchors {
    fn enter_expression(&mut self, node: &Expression) -> bool {
        if let Expression::SuperKeyword(keyword) = node {
            self.anchors.push(keyword.anchor());
        }
        true
    }
}

#[test]
fn test_resolve_virtual_picks_the_most_derived_override() {
    let unit = Hierarchy::build_compilation_unit();
    let a_f = function(&unit, "A", "f", 0);

    assert_eq!(
        contract(&unit, "C").resolve_virtual(&a_f).node_id(),
        function(&unit, "C", "f", 0).node_id(),
        "compiled into C, A.f runs C's override"
    );
    assert_eq!(
        contract(&unit, "B").resolve_virtual(&a_f).node_id(),
        function(&unit, "B", "f", 0).node_id(),
        "compiled into B, A.f runs B's override"
    );
}

#[test]
fn test_resolve_virtual_discriminates_overloads() {
    let unit = Hierarchy::build_compilation_unit();
    let a_f_x = function(&unit, "A", "f", 1);

    assert_eq!(
        contract(&unit, "C").resolve_virtual(&a_f_x).node_id(),
        a_f_x.node_id(),
        "the one-parameter f is not overridden, so it runs itself"
    );
}

#[test]
fn test_resolve_virtual_keeps_a_library_function() {
    let unit = Hierarchy::build_compilation_unit();
    let l_s = function(&unit, "L", "s", 0);

    assert_eq!(
        contract(&unit, "C").resolve_virtual(&l_s).node_id(),
        l_s.node_id(),
        "a library function runs itself although A declares one of the same signature"
    );
}

#[test]
fn test_resolve_virtual_keeps_a_free_function() {
    let unit = Hierarchy::build_compilation_unit();
    let free_s = unit
        .all_definitions()
        .find_map(|definition| match definition {
            Definition::Function(function)
                if function.enclosing_definition().is_none()
                    && function.name().is_some_and(|name| name.name() == "s") =>
            {
                Some(function)
            }
            _ => None,
        })
        .expect("the fixture declares the free function");

    assert_eq!(
        contract(&unit, "C").resolve_virtual(&free_s).node_id(),
        free_s.node_id(),
        "a free function runs itself although A declares one of the same signature"
    );
}

#[test]
fn test_resolve_virtual_implements_an_interface_member() {
    let unit = Hierarchy::build_compilation_unit();
    let i_i = function(&unit, "I", "i", 0);

    assert_eq!(
        contract(&unit, "C").resolve_virtual(&i_i).node_id(),
        function(&unit, "B", "i", 0).node_id(),
        "an interface member is implicitly virtual"
    );
}

#[test]
fn test_resolve_virtual_implements_a_bodiless_declaration() {
    let unit = Hierarchy::build_compilation_unit();
    let a_g = function(&unit, "A", "g", 0);

    assert_eq!(
        contract(&unit, "C").resolve_virtual(&a_g).node_id(),
        function(&unit, "B", "g", 0).node_id(),
        "B implements A's bodiless declaration"
    );
}

#[test]
fn test_resolve_super_runs_the_implementation_after_the_anchor() {
    let unit = Hierarchy::build_compilation_unit();
    let c_f = function(&unit, "C", "f", 0);

    assert_eq!(
        contract(&unit, "C")
            .resolve_super(&c_f, &contract(&unit, "B"))
            .node_id(),
        function(&unit, "A", "f", 0).node_id(),
        "C's linearisation is C, M, B, A: written in B, super.f runs A's"
    );
}

#[test]
fn test_resolve_super_searches_the_compiled_contracts_linearisation() {
    let unit = Hierarchy::build_compilation_unit();
    let c_f = function(&unit, "C", "f", 0);

    assert_eq!(
        contract(&unit, "C")
            .resolve_super(&c_f, &contract(&unit, "M"))
            .node_id(),
        function(&unit, "B", "f", 0).node_id(),
        "M has no base of its own: written in M and compiled into C, super.f runs B's"
    );
}

#[test]
fn test_resolve_super_skips_a_bodiless_override() {
    let unit = Hierarchy::build_compilation_unit();
    let c = contract(&unit, "C");

    assert_eq!(
        c.resolve_super(&function(&unit, "C", "f", 0), &c).node_id(),
        function(&unit, "B", "f", 0).node_id(),
        "M's f has no body, so super.f written in C runs B's"
    );
}

#[test]
fn test_resolve_modifier_picks_the_most_derived_override() {
    let unit = Hierarchy::build_compilation_unit();
    let bare = invocation(&unit, "A", "bare");
    let a_guarded = modifier(&unit, "A");

    assert_eq!(
        contract(&unit, "C")
            .resolve_modifier(&bare, &a_guarded)
            .node_id(),
        modifier(&unit, "B").node_id(),
        "compiled into C, a bare entry naming A's virtual modifier runs B's override"
    );
    assert_eq!(
        contract(&unit, "A")
            .resolve_modifier(&bare, &a_guarded)
            .node_id(),
        a_guarded.node_id(),
        "compiled into A, nothing overrides it, so it runs itself"
    );
}

#[test]
fn test_resolve_modifier_keeps_a_qualified_declaration() {
    let unit = Hierarchy::build_compilation_unit();
    let a_guarded = modifier(&unit, "A");

    assert_eq!(
        contract(&unit, "C")
            .resolve_modifier(&invocation(&unit, "A", "qualified"), &a_guarded)
            .node_id(),
        a_guarded.node_id(),
        "a qualified entry names its target, so B's override does not run"
    );
}

#[test]
fn test_resolve_modifier_keeps_a_library_modifier() {
    let unit = Hierarchy::build_compilation_unit();
    let l_guarded = modifier(&unit, "L");

    assert_eq!(
        contract(&unit, "C")
            .resolve_modifier(&invocation(&unit, "L", "attached"), &l_guarded)
            .node_id(),
        l_guarded.node_id(),
        "a library modifier runs itself although the hierarchy declares one of the same name"
    );
}

#[test]
fn test_super_is_anchored_at_the_contract_it_is_written_in() {
    let unit = Hierarchy::build_compilation_unit();

    let mut finder = SuperAnchors::default();
    accept_function_definition(&function(&unit, "B", "f", 0), &mut finder);

    let [Some(Type::Contract(anchor))] = finder.anchors.as_slice() else {
        panic!("B.f has one `super`, anchored at a contract");
    };
    assert_eq!(
        anchor.definition().node_id(),
        contract(&unit, "B").node_id(),
        "`super` in B.f is anchored at B"
    );
}
