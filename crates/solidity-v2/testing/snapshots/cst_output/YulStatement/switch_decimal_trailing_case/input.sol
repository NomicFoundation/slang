contract C {
    function f() {
        assembly {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/YulStatement/switch_decimal_trailing_case/input.sol
switch 1case 0 {} default {}
// <<<
        }
    }
}
