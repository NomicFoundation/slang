use slang_solidity_v2_common::evm_targets::EvmTarget;

use super::support::{Analyse, Analysis};
use crate::context::AbiTypeSpelling;

/// `has_abi_type` decides on its own what `write_type_abi_name` can spell. Every spelling must
/// write a type that passes it, since ABI parameters are spelled without checking for failure, and
/// the `Selector` spelling must fail on every other type.
#[test]
fn has_abi_type_agrees_with_write_type_abi_name() {
    let source = r#"
        pragma solidity *;
        interface I { function f() external; }
        type W is uint64;
        library L { function g(uint256 x) internal pure returns (uint256) { return x; } }
        contract C {
            enum E { A }
            struct T { address payable p; bytes4 sel; int8 n; bool flag; }
            struct S { uint256 a; E e; I i; W w; bytes b; string s; uint256[] xs; T[2] ts; }
            struct M { mapping(uint256 => uint256) m; uint256 x; }
            struct Node { Node[] children; uint256 v; }
            error Bad(S s);
            event Ev(T t);
            mapping(uint256 => M) ms;
            Node root;
            ufixed128x18 fx;
            function(uint256) external returns (S memory) cb;
            function f(S calldata s, T[] calldata ts, uint256[] calldata xs)
                external view returns (uint256, bytes4, bytes32, E, address)
            {
                uint256[] calldata tail = xs[1:];
                (uint256 a, bool b) = (1, true);
                uint256 g = L.g(uint256(a));
                return (
                    tail.length + ms[0].x + (b ? g : 0) + s.a + ts.length,
                    Bad.selector,
                    Ev.selector,
                    E.A,
                    address(this)
                );
            }
        }
    "#;
    let context = Analysis::of_source(source)
        .target(EvmTarget::LATEST)
        .run(Analyse::Context)
        .expect_no_diagnostics()
        .into_context();

    let (mut with_abi_type, mut without_abi_type) = (0, 0);
    for type_id in context.types().type_ids() {
        let writable = |spelling| {
            context
                .write_type_abi_name(type_id, spelling, &mut String::new())
                .is_ok()
        };
        let has_abi_type = context.has_abi_type(type_id);
        assert_eq!(
            has_abi_type,
            writable(AbiTypeSpelling::Selector),
            "`has_abi_type` and the `Selector` spelling disagree on {:?}",
            context.types().get_type_by_id(type_id),
        );
        if has_abi_type {
            for spelling in [
                AbiTypeSpelling::LibrarySelector,
                AbiTypeSpelling::Json,
                AbiTypeSpelling::LibraryJson,
            ] {
                assert!(
                    writable(spelling),
                    "{spelling:?} cannot write {:?}, which has an ABI type",
                    context.types().get_type_by_id(type_id),
                );
            }
            with_abi_type += 1;
        } else {
            without_abi_type += 1;
        }
    }
    // Both sides of the check are exercised.
    assert!(with_abi_type > 0 && without_abi_type > 0);
}
