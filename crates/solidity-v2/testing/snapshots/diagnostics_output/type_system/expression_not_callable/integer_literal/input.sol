// SPDX-License-Identifier: MIT
pragma solidity *;

// Recovered from solc:
// test/libsolidity/syntaxTests/functionCalls/int_not_callable.sol
// Calling a number literal.

contract C {
    function f() public pure {
        // TypeError 5704: This expression is not callable.
        ((1(3)), 2);
    }
}
