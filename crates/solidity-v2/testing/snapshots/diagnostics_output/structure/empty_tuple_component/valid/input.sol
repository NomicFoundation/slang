// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f() public pure {
        uint256 a;
        uint256 b;
        // Valid: omitting a component slot on the LHS (destructuring) is fine.
        (a, ) = (b, a);
        // Valid: a fully-populated tuple has no empty components.
        (a, b) = (b, a);
    }
}
