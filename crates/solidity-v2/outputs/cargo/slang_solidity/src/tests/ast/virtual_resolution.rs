//! `ContractDefinition::resolve_virtual`, `resolve_super` and
//! `resolve_modifier`, and the enclosing contract `super` carries: the queries
//! a compiler needs to pick the implementation a call or a modifier invocation
//! runs when a base's body is compiled into a derived contract.

use crate::ast::visitor::Visitor;
use crate::ast::{
    self, ContractDefinition, Definition, Expression, FunctionDefinition, FunctionKind,
    ModifierInvocation,
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
    constructor() {}
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
            Definition::Library(library) if library.name().name() == owner => {
                Some(library.functions())
            }
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

/// Captures the enclosing contract of every `super` keyword under a node.
#[derive(Default)]
struct SuperEnclosingContracts {
    enclosing_contracts: Vec<Option<ContractDefinition>>,
}

impl Visitor for SuperEnclosingContracts {
    fn enter_expression(&mut self, node: &Expression) -> bool {
        if let Expression::SuperKeyword(keyword) = node {
            self.enclosing_contracts.push(keyword.enclosing_contract());
        }
        true
    }
}

#[test]
fn test_resolve_virtual_picks_the_most_derived_override() {
    let unit = Hierarchy::build_compilation_unit();
    let a_f = function(&unit, "A", "f", 0);

    assert_eq!(
        contract(&unit, "C")
            .resolve_virtual(&a_f)
            .unwrap()
            .node_id(),
        function(&unit, "C", "f", 0).node_id(),
        "compiled into C, A.f runs C's override"
    );
    assert_eq!(
        contract(&unit, "B")
            .resolve_virtual(&a_f)
            .unwrap()
            .node_id(),
        function(&unit, "B", "f", 0).node_id(),
        "compiled into B, A.f runs B's override"
    );
}

#[test]
fn test_resolve_virtual_discriminates_overloads() {
    let unit = Hierarchy::build_compilation_unit();
    let a_f_x = function(&unit, "A", "f", 1);

    assert_eq!(
        contract(&unit, "C")
            .resolve_virtual(&a_f_x)
            .unwrap()
            .node_id(),
        a_f_x.node_id(),
        "the one-parameter f is not overridden, so it runs itself"
    );
}

#[test]
fn test_resolve_virtual_rejects_a_library_function() {
    let unit = Hierarchy::build_compilation_unit();
    let l_s = function(&unit, "L", "s", 0);

    assert!(contract(&unit, "C").resolve_virtual(&l_s).is_none());
}

#[test]
fn test_resolve_virtual_rejects_a_free_function() {
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

    assert!(contract(&unit, "C").resolve_virtual(&free_s).is_none());
}

#[test]
fn test_resolve_virtual_implements_an_interface_member() {
    let unit = Hierarchy::build_compilation_unit();
    let i_i = function(&unit, "I", "i", 0);

    assert_eq!(
        contract(&unit, "C")
            .resolve_virtual(&i_i)
            .unwrap()
            .node_id(),
        function(&unit, "B", "i", 0).node_id(),
        "an interface member is implicitly virtual"
    );
}

#[test]
fn test_resolve_virtual_implements_a_bodiless_declaration() {
    let unit = Hierarchy::build_compilation_unit();
    let a_g = function(&unit, "A", "g", 0);

    assert_eq!(
        contract(&unit, "C")
            .resolve_virtual(&a_g)
            .unwrap()
            .node_id(),
        function(&unit, "B", "g", 0).node_id(),
        "B implements A's bodiless declaration"
    );
}

#[test]
fn test_resolve_super_runs_the_implementation_after_the_enclosing_contract() {
    let unit = Hierarchy::build_compilation_unit();
    let c_f = function(&unit, "C", "f", 0);

    assert_eq!(
        contract(&unit, "C")
            .resolve_super(&c_f, &contract(&unit, "B"))
            .unwrap()
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
            .unwrap()
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
        c.resolve_super(&function(&unit, "C", "f", 0), &c)
            .unwrap()
            .node_id(),
        function(&unit, "B", "f", 0).node_id(),
        "M's f has no body, so super.f written in C runs B's"
    );
}

#[test]
fn test_resolve_modifier_picks_the_most_derived_override() {
    let unit = Hierarchy::build_compilation_unit();
    let bare = invocation(&unit, "A", "bare");

    assert_eq!(
        contract(&unit, "C")
            .resolve_modifier(&bare)
            .map(|target| target.node_id()),
        Some(modifier(&unit, "B").node_id()),
        "compiled into C, a bare entry naming A's virtual modifier runs B's override"
    );
    assert_eq!(
        contract(&unit, "A")
            .resolve_modifier(&bare)
            .map(|target| target.node_id()),
        Some(modifier(&unit, "A").node_id()),
        "compiled into A, nothing overrides it, so it runs itself"
    );
}

#[test]
fn test_resolve_modifier_keeps_a_qualified_declaration() {
    let unit = Hierarchy::build_compilation_unit();

    assert_eq!(
        contract(&unit, "C")
            .resolve_modifier(&invocation(&unit, "A", "qualified"))
            .map(|target| target.node_id()),
        Some(modifier(&unit, "A").node_id()),
        "a qualified entry names its target, so B's override does not run"
    );
}

#[test]
fn test_resolve_modifier_declines_a_modifier_outside_the_hierarchy() {
    let unit = Hierarchy::build_compilation_unit();

    assert!(
        contract(&unit, "C")
            .resolve_modifier(&invocation(&unit, "L", "attached"))
            .is_none(),
        "L's modifier is not in C's hierarchy although the hierarchy declares one of the same name"
    );
}

#[test]
fn test_super_is_anchored_at_the_contract_it_is_written_in() {
    let unit = Hierarchy::build_compilation_unit();

    let mut finder = SuperEnclosingContracts::default();
    ast::visitor::accept_function_definition(&function(&unit, "B", "f", 0), &mut finder);

    let [Some(enclosing_contract)] = finder.enclosing_contracts.as_slice() else {
        panic!("B.f has one `super`, anchored at a contract");
    };
    assert_eq!(
        enclosing_contract.node_id(),
        contract(&unit, "B").node_id(),
        "`super` in B.f is anchored at B"
    );
}

#[test]
fn test_resolve_super_rejects_an_enclosing_contract_of_a_foreign_compilation_unit() {
    let unit = Hierarchy::build_compilation_unit();
    let foreign_unit = Hierarchy::build_compilation_unit();
    let c = contract(&unit, "C");
    let foreign_c = contract(&foreign_unit, "C");
    assert_eq!(c.node_id(), foreign_c.node_id());
    assert!(
        c.resolve_super(&function(&unit, "C", "f", 0), &foreign_c)
            .is_none()
    );
}

#[test]
fn test_resolution_rejects_constructors() {
    let unit = Hierarchy::build_compilation_unit();
    let a = contract(&unit, "A");
    let constructor = a.constructor().unwrap();
    assert!(a.resolve_virtual(&constructor).is_none());
    assert!(a.resolve_super(&constructor, &a).is_none());
}

define_fixture!(
    Getter,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

interface I {
    function value() external view returns (uint256);
}

contract B is I {
    uint256 public override value;
}
"#,
);

#[test]
fn test_resolve_virtual_returns_a_getter() {
    let unit = Getter::build_compilation_unit();
    let b = contract(&unit, "B");
    let Some(ast::VirtualTarget::Getter(target)) =
        b.resolve_virtual(&function(&unit, "I", "value", 0))
    else {
        panic!("expected the public variable getter");
    };
    assert_eq!(target.node_id(), b.state_variables()[0].node_id());
    assert!(
        b.resolve_super(&function(&unit, "I", "value", 0), &b)
            .is_none()
    );
}

define_fixture!(
    InterfaceEntrypoints,
    file: "main.sol", r#"
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.29;

interface I {
    fallback() external;
    receive() external payable;
}

abstract contract A is I {}

contract B is I {
    fallback() external override {}
    receive() external payable override {}
}
"#,
);

#[test]
fn test_resolve_virtual_handles_implicit_interface_virtuality() {
    let unit = InterfaceEntrypoints::build_compilation_unit();
    let interface = unit
        .all_definitions()
        .find_map(|definition| match definition {
            Definition::Interface(interface) => Some(interface),
            _ => None,
        })
        .unwrap();
    let a = contract(&unit, "A");
    let b = contract(&unit, "B");
    let declarations = interface.functions();
    assert_eq!(declarations.len(), 2);
    for declaration in declarations {
        assert_eq!(
            a.resolve_virtual(&declaration).unwrap().node_id(),
            declaration.node_id()
        );
        let implementation = b
            .functions()
            .into_iter()
            .find(|function| {
                matches!(
                    (function.kind(), declaration.kind()),
                    (ast::FunctionKind::Fallback, ast::FunctionKind::Fallback)
                        | (ast::FunctionKind::Receive, ast::FunctionKind::Receive)
                )
            })
            .unwrap();
        assert_eq!(
            b.resolve_virtual(&declaration).unwrap().node_id(),
            implementation.node_id()
        );
        assert!(b.resolve_super(&declaration, &b).is_none());
    }
}
