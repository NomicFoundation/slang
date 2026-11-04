// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    modifier m(uint256 x) {
        _;
    }

    function f() public m(true) {}
}
