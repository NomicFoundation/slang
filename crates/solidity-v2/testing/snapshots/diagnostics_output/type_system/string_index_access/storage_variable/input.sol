// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    string s;

    function f() public view {
        // A 'string' has no element type, so it cannot be indexed.
        s[0];
    }
}
