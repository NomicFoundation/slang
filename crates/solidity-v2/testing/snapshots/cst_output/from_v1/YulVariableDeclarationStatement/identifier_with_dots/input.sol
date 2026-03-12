contract C {
    function f() {
        assembly {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/YulVariableDeclarationStatement/identifier_with_dots/input.sol
let a.b.c := 1

// <<<
        }
    }
}
