// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    modifier m() {
        _;
    }
}

contract B is A {
    // `A.m` is not `virtual`, so it cannot be overridden.
    modifier m() override {
        _;
    }
}
