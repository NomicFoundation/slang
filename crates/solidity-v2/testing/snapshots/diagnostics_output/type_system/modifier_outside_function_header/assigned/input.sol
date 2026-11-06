// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    modifier m(uint256) {
        _;
    }

    function f() public pure {
        uint256 y = m;
        y;
    }
}
