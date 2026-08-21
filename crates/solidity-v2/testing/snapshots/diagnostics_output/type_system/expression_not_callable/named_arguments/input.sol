// SPDX-License-Identifier: MIT
pragma solidity *;

// Recovered from solc:
// test/libsolidity/syntaxTests/nameAndTypeResolution/462_callable_crash.sol
// The same check applies to the named-argument call form.

contract C {
    struct S {
        uint256 a;
        bool x;
    }

    constructor() {
        // TypeError 5704: This expression is not callable.
        3({a: 1, x: true});
    }
}
