// --- path: input.sol
import "./other.sol";

function foo() returns (int) {
    return bar();
}

// --- path: other.sol
function bar() pure returns (int) {
    return 1;
}
