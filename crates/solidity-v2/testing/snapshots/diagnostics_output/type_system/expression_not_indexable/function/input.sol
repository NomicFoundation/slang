// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function g() internal pure {}

    function f() public pure {
        // A function cannot be indexed.
        g[0];
    }
}
