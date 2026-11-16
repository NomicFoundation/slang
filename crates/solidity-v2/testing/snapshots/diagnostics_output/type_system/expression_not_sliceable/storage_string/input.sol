// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    string s;

    function f() public view {
        // Only a 'string' in calldata can be sliced.
        s[1:];
    }
}
