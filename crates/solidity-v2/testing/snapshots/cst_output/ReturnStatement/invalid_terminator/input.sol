contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/ReturnStatement/invalid_terminator/input.sol
return a + 2 * some invalid tokens

;

// <<<
    }
}
