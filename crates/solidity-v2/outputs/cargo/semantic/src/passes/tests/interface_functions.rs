//! Tests that interface bases take part in `linearised_functions`: a
//! declaration nothing implements stays in an abstract contract's list, and an
//! interface's own hierarchy is linearised.

use super::support::{Analyse, Analysis, function_names};

#[test]
fn abstract_contract_keeps_the_interface_functions_nothing_implements() {
    let analysis = Analysis::of_source(
        r#"
        pragma solidity *;
        interface IBase {
            function ping(uint256 v) external returns (uint256);
            function pong() external;
        }
        interface IBalance {
            function balanceOf(address account) external view returns (uint256);
        }
        abstract contract AB is IBase, IBalance {
            mapping(address => uint256) public override balanceOf;
            function pong() external override {}
        }
        "#,
    )
    .run(Analyse::Context)
    .expect_no_diagnostics();
    let context = analysis.context();

    // `pong` is implemented by a function and `balanceOf` by a getter, which drops
    // `IBalance.balanceOf`; `ping` is implemented by nothing and stays.
    let functions = context.linearised_functions(analysis.find_contract("AB").id());
    assert_eq!(function_names(functions), ["ping", "pong"]);
    let base_functions = context.linearised_functions(analysis.find_interface("IBase").id());
    // The kept `ping` is `IBase`'s declaration; `AB.pong` stands in for `IBase.pong`.
    assert_eq!(functions[0].id(), base_functions[0].id());
    assert_ne!(functions[1].id(), base_functions[1].id());
}

#[test]
fn interface_hierarchy_is_linearised_with_overrides() {
    let analysis = Analysis::of_source(
        r#"
        pragma solidity *;
        interface IBase {
            function ping(uint256 v) external returns (uint256);
            event Pinged(uint256 v);
        }
        interface IDerived is IBase {
            function pong() external;
            function ping(uint256 v) external override returns (uint256);
        }
        "#,
    )
    .run(Analyse::Context)
    .expect_no_diagnostics();
    let context = analysis.context();

    let base = analysis.find_interface("IBase").id();
    let derived = analysis.find_interface("IDerived").id();
    let derived_functions = context.linearised_functions(derived);
    assert_eq!(function_names(derived_functions), ["ping", "pong"]);
    // The overriding `ping` stands in for `IBase`'s.
    assert_ne!(
        derived_functions[0].id(),
        context.linearised_functions(base)[0].id()
    );
    assert_eq!(context.linearised_events(derived).len(), 1);
}
