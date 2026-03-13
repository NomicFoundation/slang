contract C {
    function f() {
        assembly
// >>> Copied from crates/solidity/testing/snapshots/cst_output/YulBlock/function_def/input.sol
{
	function mult(a, b) -> result {
		result := mul(a, b)
	}
}

// <<<
    }
}
