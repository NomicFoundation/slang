// SPDX-License-Identifier: MIT
pragma solidity *;

// A type name is not a value, so it cannot be an array item.

contract C {
    function f() public pure {
        [uint];
    }
}
