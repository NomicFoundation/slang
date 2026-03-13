contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/HexStringLiterals/invalid_trailing_separator/input.sol
hex"1234_"
// <<<
;
    }
}
