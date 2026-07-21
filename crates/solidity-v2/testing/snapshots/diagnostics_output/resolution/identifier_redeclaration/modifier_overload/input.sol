// SPDX-License-Identifier: MIT
pragma solidity *;

// Modifiers cannot be overloaded: two modifiers sharing a name in the same
// contract is a redeclaration.
contract A {
    modifier m() {
        _;
    }

    modifier m(uint256 x) {
        _;
    }
}
