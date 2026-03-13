contract C {
    function f() {
        assembly
// >>> Copied from crates/solidity/testing/snapshots/cst_output/YulBlock/ignore_unknown_delim/input.sol
{
	function mult(a, b) -> result {
		result := mul(a, b)
		result := [mul(a, b)
	}
}

// <<<
    }
}
