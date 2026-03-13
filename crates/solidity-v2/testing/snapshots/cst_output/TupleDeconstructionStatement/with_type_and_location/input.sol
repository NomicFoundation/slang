contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/TupleDeconstructionStatement/with_type_and_location/input.sol
(bool memory x, bool storage y) = rhs;
// <<<
    }
}
