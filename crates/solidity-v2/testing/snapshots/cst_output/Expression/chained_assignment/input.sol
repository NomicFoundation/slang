contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/Expression/chained_assignment/input.sol
a = b = c

// <<<
;
    }
}
