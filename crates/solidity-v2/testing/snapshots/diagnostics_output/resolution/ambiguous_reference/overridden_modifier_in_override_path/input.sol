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

    // The overrides of a modifier name no base contract to override from.
    function g() public override(m) {}
}
