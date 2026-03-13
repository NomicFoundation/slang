contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/ConditionalExpression/recursive/input.sol
foo ? a == b ? a + b * c : a + b * c + d : !bar ? e + f : g + h
// <<<
;
    }
}
