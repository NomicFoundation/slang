// --- path: main.sol
import "./other.sol" as other;

function foo() returns (int) {
    return other.bar();
}

// --- path: other.sol
function bar() pure returns (int) {
    return 1;
}
