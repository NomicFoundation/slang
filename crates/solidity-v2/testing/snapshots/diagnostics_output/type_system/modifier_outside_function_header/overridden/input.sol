// SPDX-License-Identifier: MIT
pragma solidity *;

contract A {
    modifier m() virtual {
        _;
    }
}

contract C is A {
    modifier m() override {
        _;
    }

    function f() public {
        // The name resolves to both overrides, which are all modifiers.
        m;
    }
}
