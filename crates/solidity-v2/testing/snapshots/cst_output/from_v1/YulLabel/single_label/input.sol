contract C {
    function f() {
        assembly {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/YulLabel/single_label/input.sol
foo:

// <<<
        }
    }
}
