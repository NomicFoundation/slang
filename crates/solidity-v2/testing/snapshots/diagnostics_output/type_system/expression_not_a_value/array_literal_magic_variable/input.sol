// SPDX-License-Identifier: MIT
pragma solidity *;

// A magic variable is not a value, so it cannot be an array item.

contract C {
    function f() public {
        [msg];
    }
}
