// --- path: main.sol
pragma solidity *;

import "./other.sol" as other;

function foo() returns (int) {
    return other.bar();
}

// --- path: other.sol
pragma solidity *;

function bar() pure returns (int) {
    return 1;
}
