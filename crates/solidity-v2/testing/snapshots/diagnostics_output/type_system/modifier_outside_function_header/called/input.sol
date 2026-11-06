// SPDX-License-Identifier: MIT
pragma solidity *;

// Recovered from solc:
// test/libsolidity/syntaxTests/functionCalls/modifier_not_callable.sol
// Calling a modifier from an expression, rather than attaching it to a
// function.

contract C {
    // TypeError 5704: This expression is not callable.
    uint256 a = m(1000);

    modifier m(uint256) {
        _;
    }

    // Attaching the modifier to a function is fine.
    function f() public m(1) {}
}
