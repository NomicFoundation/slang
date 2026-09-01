// SPDX-License-Identifier: MIT
pragma solidity *;

// Recovered from solc:
// test/libsolidity/syntaxTests/functionCalls/magic_not_callable.sol
// Calling a built-in object, which is only good for member access.

contract C {
    // TypeError 5704: This expression is not callable.
    uint256 a = msg(1000);

    // Selecting a member of the same object is fine.
    uint256 b = msg.value;
}
