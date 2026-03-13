contract C {
    function f() {
        assembly {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/YulVariableDeclarationStatement/multiple_variables_with_value/input.sol
let x, y, z := foo()

// <<<
        }
    }
}
