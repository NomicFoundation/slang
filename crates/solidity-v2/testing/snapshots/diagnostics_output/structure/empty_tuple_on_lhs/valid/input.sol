// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        uint256 a;
        uint256 b;
        // Valid: a normal tuple assignment (destructuring).
        (a, b) = (1, 2);
        // Valid: omitting a component slot on the LHS is allowed.
        (a, ) = (1, 2);
    }
}
