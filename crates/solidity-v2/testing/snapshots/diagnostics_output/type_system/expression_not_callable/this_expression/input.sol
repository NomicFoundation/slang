// SPDX-License-Identifier: MIT
pragma solidity *;

// Recovered from solc:
// test/libsolidity/syntaxTests/functionCalls/this_not_callable.sol
// Calling `this`, which is a contract instance and not a callable.

contract C {
    function f() public {
        // TypeError 5704: This expression is not callable.
        try this() {} catch Error(string memory) {}
    }

    function g() public {
        // Calling a function through `this` is fine.
        this.f();
    }
}
