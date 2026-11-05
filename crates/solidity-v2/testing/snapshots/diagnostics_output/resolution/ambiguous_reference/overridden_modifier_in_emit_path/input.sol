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

    // The overrides of a modifier have no members, and are reported
    // rather than `E`.
    function g() public {
        emit m.E();
    }
}
