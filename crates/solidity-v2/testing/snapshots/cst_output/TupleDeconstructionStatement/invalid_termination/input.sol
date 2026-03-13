contract C {
    function f() {
// >>> Copied from crates/solidity/testing/snapshots/cst_output/TupleDeconstructionStatement/invalid_termination/input.sol
(a, b) = (123, 135)
/**/
{ throw; 

// <<<
    }
}
