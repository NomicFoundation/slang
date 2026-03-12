contract C {
    function f() {
        assembly {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/YulExpression/function_call/input.sol
foo(1)
// <<<
        }
    }
}
