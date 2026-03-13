contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/Expression/delete_conditional/input.sol
cond ? delete obj[slot1] : delete obj[slot2]

// <<<
;
    }
}
