// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    modifier m(uint256 a) virtual {
        _;
    }
}

contract B is A {
    // Modifiers override by name, so the parameter types must match.
    modifier m(uint8 a) override {
        _;
    }
}
