contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/Expression/two_conditions_false_case/input.sol
a ? b : c ? d : e // (a ? b : (c ? d : e))

// <<<
;
    }
}
