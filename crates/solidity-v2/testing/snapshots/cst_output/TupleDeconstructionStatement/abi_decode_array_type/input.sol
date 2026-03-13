contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/TupleDeconstructionStatement/abi_decode_array_type/input.sol
(uint32 a, uint32[] memory b) = abi.decode(data, (uint32, uint32[]));
// <<<
    }
}
