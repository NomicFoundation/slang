contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/TryStatement/method_call_with_body/input.sol
try foo() {
  bar();
} catch {
}

// <<<
    }
}
