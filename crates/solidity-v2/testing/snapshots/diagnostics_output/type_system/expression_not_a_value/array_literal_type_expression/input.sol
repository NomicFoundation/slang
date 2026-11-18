// SPDX-License-Identifier: MIT
pragma solidity *;

// `type(C)` is not a value, so it cannot be an array item.

contract C {
    function f() public {
        [type(C)];
    }
}
