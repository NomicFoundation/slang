contract C {
    function f() {
        assembly {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/YulFunctionCallExpression/built_in_and/input.sol
// this identifier is unreserved, and bound as a built-in function. See #1203:
and()

// <<<
        }
    }
}
