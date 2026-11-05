// SPDX-License-Identifier: MIT
pragma solidity *;

contract B {
    modifier m() virtual {
        _;
    }
}

contract C is B {
    modifier m() override {
        _;
    }

    // The overrides of a modifier name no type.
    m x;
}
