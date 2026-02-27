// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

contract C {
    function f() {
        assembly {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/YulStatement/var_assign_colon_and_equals/input.sol
// Makes sure that `newB:` is not parsed as a label here, and that we continue parsing to get a whole assignment statement.
newB: = mload(0x80)
// <<<
        }
    }
}
