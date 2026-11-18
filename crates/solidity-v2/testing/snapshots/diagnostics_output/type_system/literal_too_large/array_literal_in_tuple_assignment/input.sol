// SPDX-License-Identifier: MIT
pragma solidity *;

// The array literal is the right-hand side of a tuple assignment, wrapped in
// parentheses. Its only item is too large for any integer type.

contract C {
    function f() public pure {
        uint256 x;
        (x, ) = ([100e100]);
    }
}
