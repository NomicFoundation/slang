use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::visitor::Visitor;

use super::Analysis;

/// Collects the node ids of every expression kind that can invoke a
/// user-defined operator, in source order.
#[derive(Default)]
struct OperatorExpressions {
    ids: Vec<NodeId>,
}

macro_rules! collect_operator_expression {
    ($($hook:ident: $node:ident),* $(,)?) => {
        $(
            fn $hook(&mut self, node: &ir::$node) -> bool {
                self.ids.push(node.id());
                true
            }
        )*
    };
}

impl Visitor for OperatorExpressions {
    collect_operator_expression!(
        enter_additive_expression: AdditiveExpression,
        enter_bitwise_and_expression: BitwiseAndExpression,
        enter_bitwise_or_expression: BitwiseOrExpression,
        enter_bitwise_xor_expression: BitwiseXorExpression,
        enter_equality_expression: EqualityExpression,
        enter_inequality_expression: InequalityExpression,
        enter_multiplicative_expression: MultiplicativeExpression,
        enter_prefix_expression: PrefixExpression,
    );
}

/// The name of the function each operator expression in `source` resolves
/// to, in source order. `None` for expressions that resolve to no
/// user-defined operator function.
fn operator_resolutions(source: &str) -> Vec<Option<String>> {
    let analysis = Analysis::of_source(source).run().expect_no_diagnostics();
    let mut collector = OperatorExpressions::default();
    for source_unit in analysis.source_units() {
        ir::visitor::accept_source_unit(source_unit, &mut collector);
    }
    collector
        .ids
        .iter()
        .map(|node_id| {
            analysis
                .binder()
                .resolved_operator_function(*node_id)
                .map(|definition_id| {
                    analysis
                        .binder()
                        .find_definition_by_id(definition_id)
                        .unwrap()
                        .identifier()
                        .unparse()
                        .to_string()
                })
        })
        .collect()
}

#[test]
fn binary_operator_resolves_to_bound_function() {
    let resolutions = operator_resolutions(
        "type Int is int256;
        function add(Int, Int) pure returns (Int) { return Int.wrap(0); }
        using {add as +} for Int global;
        contract C {
            function f(Int a, Int b) internal pure returns (Int) {
                return a + b;
            }
        }",
    );
    assert_eq!(vec![Some("add".into())], resolutions);
}

#[test]
fn every_overloadable_operator_resolves() {
    let resolutions = operator_resolutions(
        "type Int is int256;
        function add(Int, Int) pure returns (Int) { return Int.wrap(0); }
        function sub(Int, Int) pure returns (Int) { return Int.wrap(0); }
        function neg(Int) pure returns (Int) { return Int.wrap(0); }
        function bitnot(Int) pure returns (Int) { return Int.wrap(0); }
        function mul(Int, Int) pure returns (Int) { return Int.wrap(0); }
        function div(Int, Int) pure returns (Int) { return Int.wrap(0); }
        function rem(Int, Int) pure returns (Int) { return Int.wrap(0); }
        function bitand(Int, Int) pure returns (Int) { return Int.wrap(0); }
        function bitor(Int, Int) pure returns (Int) { return Int.wrap(0); }
        function bitxor(Int, Int) pure returns (Int) { return Int.wrap(0); }
        function eq(Int, Int) pure returns (bool) { return true; }
        function neq(Int, Int) pure returns (bool) { return true; }
        function lt(Int, Int) pure returns (bool) { return true; }
        function lte(Int, Int) pure returns (bool) { return true; }
        function gt(Int, Int) pure returns (bool) { return true; }
        function gte(Int, Int) pure returns (bool) { return true; }
        using {
            add as +, sub as -, neg as -, bitnot as ~,
            mul as *, div as /, rem as %,
            bitand as &, bitor as |, bitxor as ^,
            eq as ==, neq as !=, lt as <, lte as <=, gt as >, gte as >=
        } for Int global;
        contract C {
            function f(Int a, Int b) internal pure {
                a + b; a - b; -a; ~a;
                a * b; a / b; a % b;
                a & b; a | b; a ^ b;
                a == b; a != b; a < b; a <= b; a > b; a >= b;
            }
        }",
    );
    let expected = [
        "add", "sub", "neg", "bitnot", "mul", "div", "rem", "bitand", "bitor", "bitxor", "eq",
        "neq", "lt", "lte", "gt", "gte",
    ];
    assert_eq!(expected.map(|name| Some(name.into())).to_vec(), resolutions);
}

#[test]
fn unbound_operators_are_not_resolved() {
    let resolutions = operator_resolutions(
        "type Int is int256;
        function add(Int, Int) pure returns (Int) { return Int.wrap(0); }
        using {add as +} for Int global;
        contract C {
            function f(Int a, Int b, uint256 x) internal pure {
                a - b;
                -a;
                x + x;
                a + b;
            }
        }",
    );
    assert_eq!(vec![None, None, None, Some("add".into())], resolutions);
}

#[test]
fn duplicate_binding_is_ambiguous_and_not_resolved() {
    let resolutions = operator_resolutions(
        "type Int is int256;
        function add1(Int, Int) pure returns (Int) { return Int.wrap(0); }
        function add2(Int, Int) pure returns (Int) { return Int.wrap(0); }
        using {add1 as +} for Int global;
        using {add2 as +} for Int global;
        contract C {
            function f(Int a, Int b) internal pure {
                a + b;
            }
        }",
    );
    assert_eq!(vec![None], resolutions);
}

#[test]
fn operator_bound_to_a_different_type_is_not_resolved() {
    let resolutions = operator_resolutions(
        "type Int is int256;
        type Other is int256;
        function add(Other, Other) pure returns (Other) { return Other.wrap(0); }
        using {add as +} for Other global;
        contract C {
            function f(Int a, Int b) internal pure {
                a + b;
            }
        }",
    );
    assert_eq!(vec![None], resolutions);
}
