contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/TryStatement/method_call_with_options/input.sol
try foo() { x: 1 } {
} catch {
}

// <<<
    }
}
