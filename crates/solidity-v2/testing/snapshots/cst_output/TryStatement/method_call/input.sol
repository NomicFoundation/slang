contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/TryStatement/method_call/input.sol
try foo() {
} catch {
}

// <<<
    }
}
