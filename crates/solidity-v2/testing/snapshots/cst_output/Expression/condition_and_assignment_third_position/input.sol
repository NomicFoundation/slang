contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/Expression/condition_and_assignment_third_position/input.sol
a ? b : c = d // (a ? b : (c = d))

// <<<
;
    }
}
