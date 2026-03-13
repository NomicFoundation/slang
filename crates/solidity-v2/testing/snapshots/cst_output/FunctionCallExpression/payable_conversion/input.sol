contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/FunctionCallExpression/payable_conversion/input.sol
payable(msg.sender)

// <<<
;
    }
}
