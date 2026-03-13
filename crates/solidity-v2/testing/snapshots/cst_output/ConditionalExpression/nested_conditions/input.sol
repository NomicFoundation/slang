contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/ConditionalExpression/nested_conditions/input.sol
foo ? (a + b) : (c + d)
// <<<
;
    }
}
