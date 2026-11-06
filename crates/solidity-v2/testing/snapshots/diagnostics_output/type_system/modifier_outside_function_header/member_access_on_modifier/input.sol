// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    modifier m() {
        _;
    }

    function f() public pure {
        m.selector;
    }
}
