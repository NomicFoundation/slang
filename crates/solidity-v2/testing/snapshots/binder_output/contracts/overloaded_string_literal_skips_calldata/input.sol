// SPDX-License-Identifier: MIT
pragma solidity *;

// A string literal does not convert to `bytes calldata`, so the call is not
// ambiguous: it resolves to `g(bytes32)`.
contract C {
    function g(bytes calldata) internal pure {}

    function g(bytes32) internal pure {}

    function f() internal pure {
        g("hello");
    }
}
