// --- path: input.sol
pragma solidity *;

import "./other.sol";

function foo() returns (int) {
    return bar();
}

// --- path: other.sol
pragma solidity *;

function bar() pure returns (int) {
    return 1;
}
