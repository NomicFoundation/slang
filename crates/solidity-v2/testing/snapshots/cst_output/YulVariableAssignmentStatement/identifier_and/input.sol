contract C {
    function f() {
        assembly {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/YulVariableAssignmentStatement/identifier_and/input.sol
// this identifier is unreserved, and bound as a variable name. See #1203:
and := 0

// <<<
        }
    }
}
