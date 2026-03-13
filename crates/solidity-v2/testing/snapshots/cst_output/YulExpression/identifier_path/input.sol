contract C {
    function f() {
        assembly {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/YulExpression/identifier_path/input.sol
foo . bar

// <<<
        }
    }
}
