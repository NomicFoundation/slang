// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    modifier m(uint256 a) virtual {
        _;
    }
}

contract B is A {
    // Overrides `A.m` without the `override` specifier.
    modifier m(uint256 a) {
        _;
    }
}
